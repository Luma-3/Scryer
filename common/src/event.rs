use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
pub enum EventType {
    Alloc = (1 << 1),
    Dealloc = (1 << 2),
    Realloc = (1 << 3),
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct AllocEvent {
    pub size: usize,
    pub ptr: usize,
    pub event_type: usize,
}

#[repr(C)]
pub struct AtomicAllocEvent {
    pub size: AtomicUsize,
    pub ptr: AtomicUsize,
    pub event_type: AtomicUsize,
}

const _: () = assert!(std::mem::size_of::<AllocEvent>() == std::mem::size_of::<AtomicAllocEvent>());
const _: () =
    assert!(std::mem::align_of::<AllocEvent>() == std::mem::align_of::<AtomicAllocEvent>());

impl AtomicAllocEvent {
    pub fn store(&self, event: AllocEvent, order: Ordering) {
        self.size.store(event.size, order);
        self.ptr.store(event.ptr, order);
        self.event_type.store(event.event_type, order);
    }

    pub fn load(&self, order: Ordering) -> AllocEvent {
        AllocEvent {
            size: self.size.load(order),
            ptr: self.ptr.load(order),
            event_type: self.event_type.load(order),
        }
    }
}
