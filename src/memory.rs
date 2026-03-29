const HRAM_START: u16 =         0xFF80; // 127 bytes of High RAM
const HRAM_END: u16 =           0xFFFE;
const EXTERN_RAM_START: u16 =   0xA000; // 8 KiB of external RAM in a Gameboy Colour
const EXTERN_RAM_END: u16 =     0xBFFF; // cartridge
const WRAM_START: u16 =         0xC000; // 32 KiB of Work RAM, provided through a fixed
const WRAM_END: u16 =           0xDFFF; // 4 KiB unit, and 7 switchable banks


pub struct Memory {
    rom: Vec<u8>,
    hram: Vec<u8>,
    external_ram: Vec<u8>,
    ram: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: Vec::new(),
            hram: Vec::new(),
            external_ram: Vec::new(),
            ram: Vec::new()
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.rom = data;
    }

    // TODO: Is this function really needed?
    pub fn get_rom(&self) -> Vec<u8> {
        self.rom.clone()
    }

    pub fn fetch_byte(&self, index: u16) -> u8 {
        *self.rom.
            get(index as usize).
            expect("Invalid memory access! PC is out of bounds.")
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
