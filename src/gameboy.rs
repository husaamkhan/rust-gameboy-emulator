use std::{
    io::Read,
    fs::File,
    io::Error
};

use crate::cpu::CPU;

// TODO:
//  - Define libraries for the individual HW components
//  - Complete GB struct definition

pub struct Gameboy {
    rom: Vec<u8>,
    cpu: CPU
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { rom: Vec::new(), cpu: CPU::new() }
    }

    pub fn load_rom(&mut self, filepath: &str) -> Result<(), Error> {
        let mut file = File::open(filepath)?;

        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data)?;

        self.rom = data;
        
        Ok(())
    }
}

