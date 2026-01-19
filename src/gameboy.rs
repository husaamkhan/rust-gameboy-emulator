use std::{
    fs::File,
    io::Error
};

use crate::cpu::CPU;

// TODO:
//  - Define libraries for the individual HW components
//  - Complete GB struct definition

pub struct Gameboy;

impl Gameboy {
    pub fn load_rom(filepath: String) -> Result<(), Error> {
        let file = File::open(filepath)?;
        Ok(())
    }
}

