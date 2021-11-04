extern crate rand;

use crate::display::{DISPLAY, HEIGHT, WIDTH};
use std::borrow::BorrowMut;
use crate::debug;

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
    pub memory: [u8; 4096],                 // RAM (4 KiB)
    pub pc: u16,                            // Program counter
    pub i: u16,                             // Index register
    pub stack: [u16; 16],                   // Stack
    pub sp: u8,                             // Stack pointer
    pub delay_timer: u8,                    // Delay timer (decrement 60 times a sec until 0)
    pub sound_timer: u8,                    // Sound timer (same as delay_timer but beep while not 0)
    pub v: [u8; 16],                        // General purpose variable registers V0 through VF
    pub pixels: [[bool; WIDTH]; HEIGHT],     // Display (64x32)
    pub display: DISPLAY,
}

impl CPU {
    pub fn new(display: DISPLAY) -> CPU {

        // Initialize memory
        let mut memory = [0; 4096];

        // Load font at address 0x050 -> 0x09F
        for i in 0..80 {
            memory[i + 80] = FONT[i];
        }

        // Initialize CPU with default values
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
            display: display
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {

        // Get ROM length
        let rom_length = rom.len();

        // Load ROM into memory
        for i in 0..rom_length {
            self.memory[i + 512] = rom[i];
        }

    }

    pub fn step(&mut self) {

        // Fetch instruction that PC is currently pointing at from memory
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
        | (self.memory[self.pc as usize + 1] as u16);

        // Increment program counter
        self.pc += 2;

        // Execute opcode
        self.execute_opcode(opcode);
    }

    pub fn execute_opcode(&mut self, opcode: u16) {

        // Decode opcode
        let nnn: u16 = (opcode & 0x0FFF) as u16;            // 0000NNNN NNNNNNNN | lowest 12 bits
        let nn: u8 = (opcode & 0x00FF) as u8;               // 00000000 NNNNNNNN | lowest 8 bits
        let n: usize = (opcode & 0x000F) as usize;          // 00000000 0000NNNN | lowest 4 bits
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;   // 0000XXXX 00000000 | lower 4 bits of high byte
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;   // 00000000 YYYY0000 | upper 4 bits of low byte  

        // Log instruction
        debug::log_instruction(self, opcode, nnn, nn, n, x, y);

        // Execute opcode
        match opcode >> 12 {
            0x0 => match opcode & 0x00FF {
                0xE0 => {
                    // TODO: Implement instruction
                },
                0xEE => {
                    // TODO: Implement instruction
                },
                _ => self.handle_unimplemented_opcode(opcode),
            },
            0x1 => {
                self.pc = nnn;
            },
            0x2 => {
                // TODO: Implement instruction
            },
            0x3 => { 
                if self.v[x] == nn {
                    self.pc += 2;
                }
             },
            0x4 => { 
                if self.v[x] != nn {
                    self.pc += 2;
                }
             },
            0x5 => { 
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            },
            0x6 => { 
                self.v[x] = nn;
            },
            0x7 => {
                self.v[x] = self.v[x].wrapping_add(nn);
             },
            0x8 => match opcode & 0x000F {
                0x0 => {
                    self.v[x] = self.v[y];
                },
                0x1 => {
                    self.v[x] |= self.v[y];
                },
                0x2 => {
                    self.v[x] &= self.v[y];
                },
                0x3 => {
                    self.v[x] ^= self.v[y];
                },
                0x4 => {
                    if self.v[x] + self.v[y] > 0xFF as u8 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x] = self.v[x].wrapping_add(self.v[y]);
                },
                0x5 => {
                    if self.v[x] > self.v[y] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                },
                0x6 => {
                    self.v[0xF] = self.v[x] & 0x000F;
                    self.v[x] >>= 1;
                },
                0x7 => {
                    if self.v[y] > self.v[x] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                },
                0xE => {
                    self.v[0xF] = self.v[x] & 0xF0;
                    self.v[x] <<= 1;
                },
                _ => self.handle_unimplemented_opcode(opcode),
            },
            0x9 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },
            0xA => {
                self.i = nnn;
            },
            0xB => {
                self.pc = nnn + self.v[0x0] as u16;
            },
            0xC => {
                self.v[x] = nn & rand::thread_rng().gen::<u8>();
            },
            0xD => {
                self.draw(x, y, n);
            },
            0xE => match opcode & 0x00FF {
                0x9E => {
                    // TODO: Implement instruction
                },
                0xA1 => {
                    // TODO: Implement instruction
                },
                _ => self.handle_unimplemented_opcode(opcode),
            },
            0xF => match opcode & 0x00FF { 
                0x07 => {
                    self.v[x] = self.delay_timer;
                },
                0x15 => {
                    self.delay_timer = self.v[x];
                },
                0x18 => {
                    self.sound_timer = self.v[x];
                },
                0x1E => {
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                },
                0x0A => {
                    // TODO: Implement instruction
                },
                0x29 => {
                    self.i = (self.v[x] * 0x5) as u16;
                },
                0x33 => {
                    self.memory[(self.i) as usize] = self.v[x] / 100;
                    self.memory[(self.i + 1) as usize] = self.v[x] % 100 / 10;
                    self.memory[(self.i + 2) as usize] = self.v[x] % 10;
                },
                0x55 => {
                    for i in 0..(x + 1) {
                        self.memory[i + self.i as usize] = self.v[i];
                    }
                },
                0x65 => {
                    for i in 0..(x + 1) {
                        self.v[i] = self.memory[i + self.i as usize];
                    }
                },
                _ => self.handle_unimplemented_opcode(opcode),
            },
            _ => self.handle_unimplemented_opcode(opcode),
        }

    }

    pub fn handle_unimplemented_opcode(&mut self, opcode: u16) {
        println!("Unimplemented opcode {:#0X}", opcode);
    }

    pub fn draw(&mut self, vx: usize, vy: usize, h: usize) {
        
        // Get x and y coords from VX and VY registers
        let x_coord = self.v[vx] as usize % WIDTH;
        let y_coord = self.v[vy] as usize % HEIGHT;
        let mut j = 0;

        // Unset flag register
        self.v[0xF] = 0;
        
        // For every row in render height
        for row in 0..h {

            // Stop drawing this row if we reach the end of the screen
            if y_coord >= HEIGHT { break }

            // Get sprite from memory
            let sprite = self.memory[usize::from(self.i + j)];

            // For every column in sprite width (8)
            for col in 0..8 {

                // Stop drawing this column if we reach the end of the screen
                if x_coord >= WIDTH { break }

                // Set pixel on/off
                let old_pixel = self.pixels[y_coord + row][x_coord + col];
                let new_pixel = (sprite & (1 << (7 - col))) != 0;
                self.pixels[y_coord + row][x_coord + col] = old_pixel ^ new_pixel;
                self.v[0xF] |= self.pixels[y_coord + row][x_coord + col] as u8;

            }

            j += 1;

        }

        // Draw pixels to the display
        self.display.draw(&self.pixels);

    }

}