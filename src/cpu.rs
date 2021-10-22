extern crate rand;

use crate::display::{DISPLAY, HEIGHT, WIDTH};
use std::borrow::BorrowMut;

use rand::Rng;
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
        let n: usize = (opcode & 0x000F) as usize;          // 00000000 0000NNNN | lowest 4 bits
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;   // 0000XXXX 00000000 | lower 4 bits of high byte
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;   // 00000000 YYYY0000 | upper 4 bits of low byte

        // Increment program counter
        self.pc += 2;

        // Decode and execute instruction
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => { self.pixels = [[false; WIDTH]; HEIGHT]; }  // [00E0]

                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }  //[00EE]

                _ => println!("Unimplemented opcode {:?}", opcode),
            },

            0x1000 => { self.pc = nnn; } // [1NNN]

            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp.wrapping_add(1);
                self.pc = nnn;
            }  // [2NNN]

            0x3000 => { if self.v[x] == nn { self.pc += 2; } }  // [3XNN]

            0x4000 => { if self.v[x] != nn { self.pc += 2; } }  // [4XNN]

            0x5000 => { if self.v[x] == self.v[y] { self.pc += 2; } }  // [4XY0]

            0x6000 => { self.v[x] = nn; } // [6XNN]

            0x7000 => { self.v[x] = self.v[x].wrapping_add(nn); }  // [7XNN]

            0x8000 => match opcode & 0x000F {
                0x0000 => { self.v[x] = self.v[y]; }  // [8XY0]

                0x0001 => { self.v[x] |= self.v[y] } // [8XY1]

                0x0002 => { self.v[x] &= self.v[y] } // [8XY2]

                0x0003 => { self.v[x] ^= self.v[y] } // [8XY3]

                0x0004 => {
                    self.v[x] = self.v[x].wrapping_add(self.v[y]);
                    if self.v[x] + self.v[y] > 0xFF as u8 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                } // [8XY4]

                0x0005 => { 
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    if self.v[x] > self.v[y] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                } // [8XY5]

                0x0006 => { 
                    self.v[0xF] = self.v[x] & 0x0F;
                    self.v[x] >>= 1;
                } // [8XY6]

                0x0007 => { 
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    if self.v[y] > self.v[x] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                } // [8XY7]

                0x000E => { 
                    self.v[0xF] = self.v[x] & 0xF0;
                    self.v[x] <<= 1;
                } // [8XYE]

                _ => println!("Unimplemented opcode {:?}", opcode),
            }

            0x9000 => { if self.v[x] != self.v[y] { self.pc += 2; } }  // [9XY0]

            0xA000 => { self.i = nnn; }  // [ANNN]

            0xB000 => { self.pc = nnn + self.v[0x0] as u16; } // [BNNN]

            0xC000 => { self.v[x] = nn & rand::thread_rng().gen::<u8>() } // [CXNN]

            0xD000 => { self.draw(x, y, n, (*display).borrow_mut()); }  // [DXYN]

            0xE000 => match opcode & 0x000F {
                0x009E => {  } // [EX9E]

                0x00A1 => {  } // [EXA1]

                _ => println!("Unimplemented opcode {:?}", opcode),
            }

            0xF000 => match opcode & 0x000F {
                0x0007 => {  } // [FX07]

                0x0015 => {  } // [FX15]

                0x0018 => {  } // [FX18]

                0x001E => {  } // [FX1E]

                0x000A => {  } // [FX0A]

                0x0029 => {  } // [FX29]

                0x0033 => {  } // [FX33]

                0x0055 => {  } // [FX55]

                0x0065 => {  } // [FX65]

                _ => println!("Unimplemented opcode {:?}", opcode),
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