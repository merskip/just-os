use core::task::{
    Context,
    Poll::{Pending, Ready},
    Waker,
};

use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use crossbeam_queue::ArrayQueue;
use futures_util::Future;
use x86_64::instructions::interrupts::enable_and_hlt;

use crate::interrupts::enable;

use super::{Task, TaskId};

const TASK_QUEUE_SIZE: usize = 255;

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(TASK_QUEUE_SIZE)),
            waker_cache: BTreeMap::new(),
        }
    }
}

impl Executor {
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        let task = Task::new(future);
        let task_id = task.id;
        if self.tasks.insert(task_id, task).is_some() {
            panic!("task with the same ID already exists in tasks");
        }
        self.task_queue.push(task_id).expect("task queue is full");
    }
}

impl Executor {
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts;

        interrupts::disable();
        if self.task_queue.is_empty() {
            interrupts::enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    fn run_ready_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task now longer exists
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));

            let mut context = Context::from_waker(waker);

            match task.poll(&mut context) {
                Ready(_) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Pending => {}
            }
        }
    }
}

struct TaskWaker {
    task_id: TaskId,
    tasks_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, tasks_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            tasks_queue,
        }))
    }
}

impl TaskWaker {
    fn wake_task(&self) {
        self.tasks_queue
            .push(self.task_id)
            .expect("task queue is full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task()
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}
