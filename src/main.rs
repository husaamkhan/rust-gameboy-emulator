use std::{
    env,
    process
};

mod gameboy;
mod cpu;
use gameboy::Gameboy;
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    
    let gb = Gameboy::new();
    match gb.load_rom(rom_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to load rom: {e}");
            process::exit(1);
        }
    }
}
