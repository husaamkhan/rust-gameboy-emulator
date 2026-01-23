use std::{
    io::Read,
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
        self.read_rom_from_stream(&mut file)
    }

    /*
     * Loads in data from a stream to rom
     * Takes a stream to read in data (e.g std::fs::File)
     *
     * This approach of taking an open stream, rather than just opening+reading in from a file
     * facilitates unit tests that do not interact with any actual files
     */
    fn read_rom_from_stream<Stream: std::io::Read>(&mut self, stream: &mut Stream) -> Result<(), GameboyError> {
        let mut data: Vec<u8> = Vec::new();
        stream.read_to_end(&mut data).map_err(|err| GameboyError::IOError(err))?;

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
    fn load_rom_with_empty_file_path() {
        let mut gb = Gameboy { rom: Vec::new(), cpu: CPU::new()};

        let filepath = "";
        let result = gb.load_rom(filepath);

        assert!(matches!(
                result,
                Err(GameboyError::IOError(e)) if e.kind() == std::io::ErrorKind::NotFound));

        assert_eq!(gb.rom.len(), 0);
    }
}



