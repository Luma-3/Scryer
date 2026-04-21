use std::sync::atomic::AtomicUsize;

#[repr(C)]
pub enum EventType {
    Alloc,
    Dealloc,
    Realloc,
}

#[repr(C)]
pub struct AllocEvent {
    pub size: AtomicUsize,
    pub ptr: AtomicUsize,
    pub event_type: EventType,
}
