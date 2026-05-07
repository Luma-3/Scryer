pub mod event;

#[repr(C)]
pub struct SharedData {
    pub head: std::sync::atomic::AtomicUsize,
    pub tail: std::sync::atomic::AtomicUsize,
    pub buffer: [event::AllocEvent; 1024],
}
