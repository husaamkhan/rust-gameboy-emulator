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
            hram: vec![0u8; (HRAM_END - HRAM_START+1) as usize],
            external_ram: vec![0u8; (EXTERN_RAM_END - EXTERN_RAM_START-1) as usize],
            ram: vec![0u8; (WRAM_END - WRAM_START-1) as usize]
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

    pub fn write(&mut self, addr: u16, value: u8) {
        // TODO: add writes to remaining writeable memory regions
        // Writes value to the correct ram vector, based on the memory map
        if addr >= HRAM_START && addr < HRAM_END {
            self.hram[(addr - HRAM_START) as usize] = value;
        } else if addr >= EXTERN_RAM_START && addr < EXTERN_RAM_END {
            self.external_ram[(addr - EXTERN_RAM_START) as usize] = value;
        } else if addr >= WRAM_START && addr < WRAM_END {
            self.ram[(addr - WRAM_START) as usize] = value;
        } else {
            panic!("Error: Attempted write to invalid memory region!");
        }
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
