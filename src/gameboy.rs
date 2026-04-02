use std::{
    fs::File,
    fmt,
    rc::Rc,
    cell::RefCell
};

use crate::cpu::CPU;
use crate::memory::Memory;

const MAX_ROM_SIZE: usize = 8000;

#[derive(Debug)]
pub enum GameboyError {
    EmptyRom,
    RomTooLarge,
    IOError(std::io::Error)
}

impl std::error::Error for GameboyError {}

impl fmt::Display for GameboyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameboyError::EmptyRom => write!(f, "The ROM file is empty."),
            GameboyError::RomTooLarge => write!(f, "The ROM file is too large."),
            GameboyError::IOError(err) => write!(f, "{err}")
        }
    }
}

pub struct Gameboy {
    cpu: CPU,
    memory: Rc<RefCell<Memory>>
}

impl Gameboy {
    pub fn new() -> Gameboy {
        let memory = Rc::new(RefCell::new(Memory::new()));
        Gameboy {
            cpu: CPU::new(Rc::clone(&memory)),
            memory
        }
    }

    pub fn load_rom(&mut self, filepath: &str) -> Result<(), GameboyError> {
        let mut file = File::open(filepath).map_err(GameboyError::IOError)?;
        self.read_rom_from_buffer(&mut file)
    }

    /*
     * Loads in data from a buffer to rom
     * Takes a buffer to read in data (e.g std::fs::File)
     *
     * This approach of taking an open buffer, rather than just opening+reading in from a file
     * facilitates unit tests that do not interact with any actual files
     */
    fn read_rom_from_buffer<Buffer: std::io::Read>(&mut self, buffer: &mut Buffer) -> Result<(), GameboyError> {
        let mut data: Vec<u8> = Vec::new();
        buffer.read_to_end(&mut data).map_err(GameboyError::IOError)?;

        if data.is_empty() {
            return Err(GameboyError::EmptyRom);
        } else if data.len() > MAX_ROM_SIZE {
            return Err(GameboyError::RomTooLarge);
        }

        self.memory.borrow_mut().load_rom(data);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_rom_from_buffer_empty() {
        let mut gb = Gameboy::new();
        
        let result = gb.read_rom_from_buffer(&mut [].as_ref());

        assert!(matches!(result, Err(GameboyError::EmptyRom)));
        assert_eq!(gb.memory.borrow_mut().get_rom().len(), 0);
    }

    #[test]
    fn read_rom_from_buffer_rom_too_large() {
        let mut gb = Gameboy::new();
        let data: Vec<u8> = vec![0; MAX_ROM_SIZE+1];
        
        let result = gb.read_rom_from_buffer(&mut data.as_slice());

        assert!(matches!(result, Err(GameboyError::RomTooLarge)));
        assert_eq!(gb.memory.borrow_mut().get_rom().len(), 0);
    }

    #[test]
    fn read_rom_from_buffer_valid() {
        let mut gb = Gameboy::new();
        let data = vec![1, 2, 3, 4, 5];

        let result = gb.read_rom_from_buffer(&mut data.as_slice());
        
        assert!(matches!(result, Ok(())));
        assert_eq!(gb.memory.borrow_mut().get_rom(), data);
    }

    #[test]
    fn read_rom_from_buffer_large_rom() {
        let mut gb = Gameboy::new();
        let data: Vec<u8> = vec![1; MAX_ROM_SIZE];
        
        let result = gb.read_rom_from_buffer(&mut data.as_slice());

        assert!(matches!(result, Ok(())));
        assert_eq!(gb.memory.borrow_mut().get_rom(), data);
    }
}



