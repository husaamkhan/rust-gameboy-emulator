use std::{
    fs,
    error
}

// TODO:
//  - Define GB as a struct
//  - Define libraries for the individual HW components

pub struct Gameboy {

}

impl Gameboy {
    pub fn load_rom(filepath: String) -> Result<String, String> {
        Ok("ROM successfully loaded.".to_string())
    }
}

