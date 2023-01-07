use core::{pin::Pin, future::Future, task::{Context, Poll}};

use alloc::boxed::Box;

pub mod simple_executor;
pub mod keyboard;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Task {
            future: Box::pin(future)
        }
    }
}

impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}