use std::{
    env
};

mod gameboy;
mod cpu;
use gameboy::Gameboy;
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];

    
}
