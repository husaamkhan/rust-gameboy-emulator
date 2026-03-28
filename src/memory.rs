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

    pub fn fetch_byte(&self, index: u16) -> u8 {
        *self.rom.get(index as usize).expect("Invalid memory access! PC is out of bounds.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_byte() {
        let mut memory = Memory::new();
        memory.rom = vec![1, 2, 3];

        assert_eq!(memory.fetch_byte(0), 1);
        assert_eq!(memory.fetch_byte(1), 2);
        assert_eq!(memory.fetch_byte(2), 3);
    }

    #[test]
    fn fetch_byte_out_of_bounds() {
        let mut memory = Memory::new();
        memory.rom = vec![];

        let result = std::panic::catch_unwind(|| memory.fetch_byte(0));
        assert!(result.is_err());
    }

}
