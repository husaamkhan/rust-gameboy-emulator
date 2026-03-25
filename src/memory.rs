pub struct Memory {
    rom: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { rom: Vec::new() }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.rom = data;
    }

    pub fn get_rom(&self) -> Vec<u8> {
        self.rom.clone()
    }

    pub fn fetch_next_byte(&self, pc: u16) -> u8 {
        *self.rom.get(pc as usize).expect("Invalid memory access! PC is out of bounds.")
    }
}
