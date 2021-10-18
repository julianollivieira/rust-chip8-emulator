use std::env;
use std::fs;

mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_file_path = &args[1];
    let rom = fs::read(rom_file_path).expect("Failed to read ROM data");

    if rom.len() >= 3585 {
        panic!("ROM is too large! size: {}", rom.len());
    }

    let mut CPU = cpu::CPU::new();
    CPU.load_bin(rom);

    loop {
        CPU.step();
    }
}
