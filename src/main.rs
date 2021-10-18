use std::env;
use std::fs;

mod cpu;
mod display;

fn main() {
    // Read the ROM from a file
    let args: Vec<String> = env::args().collect();
    let rom_file_path = &args[1];
    let rom = fs::read(rom_file_path).expect("Failed to read ROM data");
    if rom.len() >= 3585 {
        panic!("ROM is too large! size: {}", rom.len());
    }

    // Initialize the DISPLAY
    let sdl_context = sdl2::init().unwrap();
    let mut DISPLAY = display::DISPLAY::new(&sdl_context);

    // Initialize the CPU
    let mut CPU = cpu::CPU::new();
    CPU.load_bin(rom);

    loop {
        CPU.step(&mut DISPLAY);
    }
}
