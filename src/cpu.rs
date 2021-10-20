use crate::display::{DISPLAY, HEIGHT, WIDTH};
use std::borrow::BorrowMut;

// Font data
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {
    memory: [u8; 4096],                 // RAM (4 KiB)
    pc: u16,                            // Program counter
    i: u16,                             // Index register
    stack: [u16; 16],                   // Stack
    sp: u8,                             // Stack pointer
    delay_timer: u8,                    // Delay timer (decrement 60 times a sec until 0)
    sound_timer: u8,                    // Sound timer (same as delay_timer but beep while not 0)
    v: [u8; 16],                        // General purpose variable registers V0 through VF
    pixels: [[bool; WIDTH]; HEIGHT]     // Display (64x32)
}

impl CPU {
    pub fn new() -> CPU {

        // Initialize memory and load font at address 0x050 -> 0x09F
        let mut memory = [0; 4096];

        for i in 0..80 {
            memory[i + 80] = FONT[i];
        }

        CPU {
            memory: memory,
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {

        // Load ROM into memory
        let rom_length = rom.len();
        for i in 0..rom_length {
            self.memory[i + 512] = rom[i];
        }

    }

    pub fn step(&mut self, display: &mut DISPLAY) {

        // Fetch instruction that PC is currently pointing at from memory
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);

        let nnn: u16 = (opcode & 0x0FFF) as u16;            // 0000NNNN NNNNNNNN | lowest 12 bits
        let nn: u8 = (opcode & 0x00FF) as u8;               // 00000000 NNNNNNNN | lowest 8 bits
        let n = (opcode & 0x000F) as usize;                 // 00000000 0000NNNN | lowest 4 bits
        let x = ((opcode & 0x0F00) >> 8) as usize;          // 0000XXXX 00000000 | lower 4 bits of high byte
        let y = ((opcode & 0x00F0) >> 4) as usize;          // 00000000 YYYY0000 | upper 4 bits of low byte

        // Increment program counter
        self.pc += 2;

        // Decode and execute instruction
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => { // [00E0]
                    println!("CLEAR");
                    self.pixels = [[false; WIDTH]; HEIGHT];
                }

                0x00EE => { //[00EE]
                    println!("RETURN FROM SUBROUTINE");
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }

                _ => println!("Unimplemented opcode {:?}", opcode),
            },

            0x1000 => { // [1NNN]
                println!("JUMP TO [{:#0X}]", nnn);
                self.pc = nnn;
            }

            0x2000 => { // [2NNN]
                println!("CALL SUBROUTINE AT {}", nnn);
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp.wrapping_add(1);
                self.pc = nnn;
            }

            0x6000 => { // [6XNN]
                println!("SET REGISTER [V{:#}] TO [{:#0X}]", x, nn);
                self.v[x] = nn;
            }

            0x7000 => { // [7XNN]
                println!("ADD [{:#0X}] TO REGISTER [V{:#}]", nn, x);
                self.v[x] += self.v[x].wrapping_add(nn);
            }

            0xA000 => { // [ANNN]
                println!("SET INDEX REGISTER I TO [{:#0X}]", nnn);
                self.i = nnn;
            }

            0xD000 => { // [DXYN]
                println!("DRAW SPRITE AT [{}, {}] WITH HEIGHT OF [{}]", x, y, n);
                self.draw(x, y, n, (*display).borrow_mut());
            }

            _ => println!("Unimplemented opcode {:?}", opcode),
        }
        
    }

    pub fn draw(&mut self, vx: usize, vy: usize, h: usize, display: &mut DISPLAY) {
        
        // Get x and y coords from VX and VY registers
        let x_coord = self.v[vx] as usize % WIDTH;
        let y_coord = self.v[vy] as usize % HEIGHT;
        let mut j = 0;

        self.v[0xF] = 0;
        
        // For every row in render height
        for row in 0..h {
            // Stop drawing this row if we reach the end of the screen
            if y_coord >= HEIGHT {
                break;
            }
            // Get sprite from memory
            let sprite = self.memory[usize::from(self.i + j)];
            // For every column in sprite width (8)
            for col in 0..8 {
                // Stop drawing this column if we reach the end of the screen
                if x_coord >= WIDTH {
                    break;
                }
                // Set pixel on/off
                let old_pixel = self.pixels[y_coord + row][x_coord + col];
                let new_pixel = (sprite & (1 << (7 - col))) != 0;
                self.pixels[y_coord + row][x_coord + col] = old_pixel ^ new_pixel;
                self.v[0xF] |= self.pixels[y_coord + row][x_coord + col] as u8;
            }

            j += 1;
        }

        // Draw pixels to the display
        display.draw(&self.pixels);
    }

}