pub mod event;

#[repr(C)]
pub struct SharedData {
    pub head: std::sync::atomic::AtomicUsize,
    pub tail: std::sync::atomic::AtomicUsize,
    pub buffer: [event::AtomicAllocEvent; 1024],
}
