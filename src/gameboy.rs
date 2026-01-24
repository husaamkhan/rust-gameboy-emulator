use std::{
    fs::File,
    fmt,
    io::Cursor
};

use crate::cpu::CPU;

#[derive(Debug)]
pub enum GameboyError {
    EmptyRom,
    IOError(std::io::Error)
}

impl std::error::Error for GameboyError {}

impl fmt::Display for GameboyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameboyError::EmptyRom => write!(f, "The ROM file is empty."),
            GameboyError::IOError(err) => write!(f, "{err}")
        }
    }
}

pub struct Gameboy {
    rom: Vec<u8>,
    cpu: CPU
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { rom: Vec::new(), cpu: CPU::new() }
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
        }

        self.rom = data;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn read_rom_from_buffer_valid() {
        let mut gb = Gameboy::new();

        let data = vec![1, 2, 3, 4, 5];
        let mut buffer = Cursor::new(&data);

        let result = gb.read_rom_from_buffer(&mut buffer);
        
        assert!(matches!(result, Ok(())));
        assert_eq!(gb.rom, data);
    }
}



