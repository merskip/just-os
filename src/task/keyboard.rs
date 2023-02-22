use alloc::boxed::Box;
use core::{
    pin::Pin,
    task::{Context, Poll},
};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{Stream, StreamExt, task::AtomicWaker};
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, layouts, ScancodeSet1};

use crate::{log_info, log_warning};

static SCAN_CODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static SCAN_CODE_QUEUE_SIZE: usize = 255;
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn add_scan_code(scan_code: u8) {
    if let Ok(queue) = SCAN_CODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scan_code) {
            log_warning!("Scan code queue full, dropping keybaord input")
        } else {
            WAKER.wake();
        }
    } else {
        log_warning!("Scan code queue uninitialized");
    }
}

pub struct ScanCodeStream {}

impl ScanCodeStream {
    pub fn new() -> Self {
        SCAN_CODE_QUEUE
            .try_init_once(|| ArrayQueue::new(SCAN_CODE_QUEUE_SIZE))
            .expect("ScancodeStream::new should only be called once");
        ScanCodeStream {}
    }
}

impl Stream for ScanCodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let queue = SCAN_CODE_QUEUE
            .try_get()
            .expect("WARNING: scan code queue uninitialized");

        if let Some(scan_code) = queue.pop() {
            return Poll::Ready(Some(scan_code));
        }

        WAKER.register(&context.waker());
        match queue.pop() {
            Some(scan_code) => {
                WAKER.take();
                Poll::Ready(Some(scan_code))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn keyboard_decoding_task(mut handler: Box<dyn FnMut(DecodedKey)>) {
    let mut scancodes = ScanCodeStream::new();
    let mut keyboard = Keyboard::<layouts::Us104Key, ScancodeSet1>::new(HandleControl::Ignore);

    while let Some(scan_code) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => log_info!("KEYBOARD CHAR={}", character),
                    DecodedKey::RawKey(key) => log_info!("KEYBOARD KEY={:?}", key),
                }
                handler(key)
            }
        }
    }
}
