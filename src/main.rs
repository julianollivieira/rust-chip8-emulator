mod lib;

use clap::Parser;
use lib::cpu::{Options, CPU};
use lib::graphics::Display;
use std::{thread, time};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short)]
    rom_file_path: String,
    #[clap(short)]
    put_value_of_vy_into_vx_before_shifting: bool,
    #[clap(short)]
    jump_to_nnn_plus_the_value_in_v0: bool,
    #[clap(short)]
    increment_i_when_storing_loading_memory: bool,
}

fn main() {
    let (
        rom_file_path,
        put_value_of_vy_into_vx_before_shifting,
        jump_to_nnn_plus_the_value_in_v0,
        increment_i_when_storing_loading_memory,
    );

    // Parse the command line arguments
    Args {
        rom_file_path,
        put_value_of_vy_into_vx_before_shifting,
        jump_to_nnn_plus_the_value_in_v0,
        increment_i_when_storing_loading_memory,
    } = Args::parse();

    // Read the ROM file
    let rom = std::fs::read(rom_file_path).expect("Failed to read ROM data");
    if rom.len() >= 3585 {
        panic!("ROM is too large! size: {}", rom.len());
    }

    // TODO: Initialize the display
    let sdl_context = sdl2::init().unwrap();
    let display = Display::new(sdl_context);

    // TODO: Initialize the CPU
    let mut cpu = CPU::new(
        display,
        Options {
            put_value_of_vy_into_vx_before_shifting,
            jump_to_nnn_plus_the_value_in_v0,
            increment_i_when_storing_loading_memory,
        },
    );
    cpu.load_rom(rom);

    loop {
        thread::sleep(time::Duration::from_millis(1));
        cpu.step();
    }
}
