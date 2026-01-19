use std::{
    env,
    process
};

mod gameboy;
mod cpu;
use gameboy::Gameboy;

const NUM_ARGS: usize = 2;
    
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < NUM_ARGS {
        eprintln!("Error: No ROM provided. Please provide the path to a Gameboy ROM.");
        process::exit(1);
    }
    
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
