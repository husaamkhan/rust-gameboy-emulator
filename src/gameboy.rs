use std::{
    fs::File,
    io::Error
};

// TODO:
//  - Define libraries for the individual HW components
//  - Complete GB struct definition

struct Gameboy;

impl Gameboy {
    pub fn load_rom(filepath: String) -> Result<(), io::Error> {
        let file = File::open(filepath)?;
        Ok(())
    }
}

