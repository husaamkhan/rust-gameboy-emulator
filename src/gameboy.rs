use std::{
    fs::File,
    io::Error
};

use crate::cpu::CPU;

// TODO:
//  - Define libraries for the individual HW components
//  - Complete GB struct definition

pub struct Gameboy {
    cpu: CPU    
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { cpu: CPU::new() }
    }

    pub fn load_rom(&self, filepath: &str) -> Result<(), Error> {
        let file = File::open(filepath)?;
        Ok(())
    }
}

