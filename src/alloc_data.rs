use common::event;

#[derive(Debug)]
pub struct AllocData {
    pub size: usize,
    pub alloc_type: event::EventType,
}

impl AllocData {}
