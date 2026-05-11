use std::sync::atomic::{AtomicUsize, Ordering};

pub mod event;

#[repr(C)]
pub struct SharedData {
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
    pub buffer: [event::AtomicAllocEvent; 1024],
}

impl SharedData {
    pub fn pop(&self) -> Option<event::AllocEvent> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if head == tail {
            None
        } else {
            let event = self.buffer[tail % self.buffer.len()].load(Ordering::Acquire);
            self.tail.store(tail.wrapping_add(1), Ordering::Release);
            Some(event)
        }
    }

    pub fn push(&self, event: event::AllocEvent) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);
        if head.wrapping_add(1) % self.buffer.len() == tail {
            false
        } else {
            self.buffer[head % self.buffer.len()].store(event, Ordering::Release);
            self.head.store(head.wrapping_add(1), Ordering::Release);
            true
        }
    }
}
