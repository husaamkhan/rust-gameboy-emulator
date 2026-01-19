use std::{
    io::Read,
    fs::File,
    io::Error
};

use crate::cpu::CPU;

// TODO:
//  - Define libraries for the individual HW components
//  - Complete GB struct definition

struct Cartridge {
    rom: Vec<u8>   
}

impl Cartridge {
    fn new() -> Cartridge {
        Cartridge { rom: Vec::new() }
    }

    fn load_rom(&mut self, data: Vec<u8>) {
        self.rom = data;
    }
}

pub struct Gameboy {
    cartridge: Cartridge,
    cpu: CPU
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy { cartridge: Cartridge::new(),  cpu: CPU::new() }
    }

    pub fn load_rom(&mut self, filepath: &str) -> Result<(), Error> {
        let mut file = File::open(filepath)?;

        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data)?;

        self.cartridge.load_rom(data);
        
        Ok(())
    }
}

