#[derive(Debug, Clone)]
#[repr(C)]
pub struct MemoryEvent {
    pub size: u64,
    pub adress: u64,
    pub alloc: bool,
}

impl MemoryEvent {
    pub fn new(size: u64, adress: u64, alloc: bool) -> Self {
        MemoryEvent { size, adress, alloc }
    }


}
