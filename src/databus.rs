use crate::memory::Memory;

pub struct DataBus {
    memory: Memory
}

impl DataBus {
    pub fn new() -> DataBus { // TODO: Modify so that memory can be mocked
        DataBus { memory: Memory::new() }
    } 

    pub fn load_rom(&mut self, data: Vec<u8>) { // TODO: Test
        self.memory.load_rom(data);
    }

    pub fn fetch_next_byte(&self, pc: u16) -> u8 {
        self.memory.fetch_next_byte(pc)
    }
}

