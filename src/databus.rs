use crate::memory::Memory;

pub struct DataBus {
    memory: Memory
}

impl DataBus {
    pub fn new() -> DataBus {
        DataBus { memory: Memory::new() }
    } 
}
