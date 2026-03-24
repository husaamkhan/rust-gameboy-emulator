use std::{
    fs::File,
    fmt,
    rc::Rc
};

use crate::cpu::CPU;
use crate::databus::DataBus;

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
    bus: Rc<DataBus>
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { cpu: CPU::new(), bus: Rc::new(DataBus::new()) }
    }

    pub fn init(&mut self) {
        self.cpu.connect_bus(self.bus.clone());
    }

    pub fn load_rom(&mut self, filepath: &str) -> Result<(), GameboyError> {
        let mut file = File::open(filepath).map_err(|err| GameboyError::IOError(err))?;
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
        buffer.read_to_end(&mut data).map_err(|err| GameboyError::IOError(err))?;

        if data.is_empty() {
            return Err(GameboyError::EmptyRom);
        } else if data.len() > MAX_ROM_SIZE {
            return Err(GameboyError::RomTooLarge);
        }

        self.bus.load_rom(data);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_rom_from_buffer_empty() {
        let mut gb = Gameboy::new();
        let mut buffer = Cursor::new(Vec::new());
        
        let result = gb.read_rom_from_buffer(&mut buffer);

        assert!(matches!(
            result,
            Err(GameboyError::EmptyRom)));
        
        assert_eq!(gb.rom.len(), 0);
    }

    #[test]
    fn read_rom_from_buffer_rom_too_large() {
        let mut gb = Gameboy::new();
        let data: Vec<u8> = vec![0; MAX_ROM_SIZE+1];
        let mut buffer = Cursor::new(data);
        
        let result = gb.read_rom_from_buffer(&mut buffer);

        assert!(matches!(
            result,
            Err(GameboyError::RomTooLarge)));
        
        assert_eq!(gb.rom.len(), 0);
}

    #[test]
    fn read_rom_from_buffer_valid() {
        let mut gb = Gameboy::new();

        let data = vec![1, 2, 3, 4, 5];
        let mut buffer = Cursor::new(&data);

        let result = gb.read_rom_from_buffer(&mut buffer);
        
        assert!(matches!(result, Ok(())));
        assert_eq!(gb.rom, data);
    }

    #[test]
    fn read_rom_from_buffer_large_rom() {
        let mut gb = Gameboy::new();
        let data: Vec<u8> = vec![1; MAX_ROM_SIZE];
        let mut buffer = Cursor::new(data.clone());
        
        let result = gb.read_rom_from_buffer(&mut buffer);

        assert!(matches!(result, Ok(())));
        assert_eq!(gb.rom, data);
    }
}



