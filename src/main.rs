mod cpu;
mod display;
mod font;

fn main() {
    // Get the first argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    // Read the ROM file
    let rom_file_path = &args[1];
    let rom = std::fs::read(rom_file_path).expect("Failed to read ROM data");
    if rom.len() >= 3585 {
        panic!("ROM is too large! size: {}", rom.len());
    }

    // TODO: Initialize the display
    let sdl_context = sdl2::init().unwrap();
    let display = display::Display::new(sdl_context);

    // TODO: Initialize the CPU
    let mut cpu = cpu::CPU::new(display);
    cpu.load_rom(rom);

    loop {
        cpu.step();
    }
}
