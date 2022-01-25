extern crate rand;
use crate::display::{Display, HEIGHT, WIDTH};
use crate::font::FONT;
use rand::Rng;

pub struct CPU {
    pub memory: [u8; 0x1000],            // RAM (4KiB)
    pub pc: u16,                         // Program counter
    pub delay_timer: u8,                 // Delay timer
    pub sound_timer: u8,                 // Sound timer
    pub stack: Vec<u16>,                 // Stack
    pub sp: u8,                          // Stack pointer
    pub i: u16,                          // Index register
    pub v: [u8; 16],                     // General purpose registers (V0 through VF)
    pub pixels: [[bool; WIDTH]; HEIGHT], // Display (64 x 32)
    pub display: Display,                // Display
}

impl CPU {
    pub fn new(display: Display) -> CPU {
        // Initialize memory
        let mut memory = [0; 0x1000];

        // Load font into memory at address 0x050 -> 0x09F
        for i in 0x050..0x09F {
            memory[i] = FONT[i - 0x050];
        }

        // Initialize the CPU with default values
        CPU {
            memory,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
            sp: 0,
            i: 0,
            v: [0; 16],
            pixels: [[false; WIDTH]; HEIGHT],
            display,
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // Load the ROM into memory
        let rom_length = rom.len();
        for i in 0..rom_length {
            self.memory[0x200 + i] = rom[i];
        }
    }

    pub fn step(&mut self) {
        // Fetch instruction that the PC is currently pointing at from memory
        let first_opcode_byte = self.memory[self.pc as usize];
        let second_opcode_byte = self.memory[(self.pc + 1) as usize];
        let opcode: u16 = (first_opcode_byte as u16) << 8 | second_opcode_byte as u16;

        // Increment program counter
        self.pc += 2;

        // Execute opcode
        self.execute_opcode(opcode);
    }

    pub fn execute_opcode(&mut self, opcode: u16) {
        /*
        | nnn = 0000NNNN NNNNNNNN | lowest 12 bits
        |  nn = 00000000 NNNNNNNN | lowest 8 bits
        |   n = 00000000 0000NNNN | lowest 4 bits
        |   x = 0000XXXX 00000000 | lower 4 bits of high byte
        |   y = 00000000 YYYY0000 | upper 4 bits of low byte
        */
        let nnn: u16 = (opcode & 0x0FFF) as u16;
        let nn: u8 = (opcode & 0x00FF) as u8;
        let n: usize = (opcode & 0x000F) as usize;
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;

        match opcode >> 12 {
            0x0 => match opcode & 0x00FF {
                0xE0 => {
                    self.display.clear();
                }
                0xEE => {
                    self.pc = self.stack.pop().unwrap();
                }
                _ => self.handle_unimplemented_opcode(opcode),
            },
            0x1 => {
                self.pc = nnn;
            }
            0x2 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x3 => {
                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.v[x] != nn {
                    self.pc += 2;
                }
            }
            0x5 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6 => {
                self.v[x] = nn;
            }
            0x7 => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0x8 => match opcode & 0x000F {
                0x0 => {
                    self.v[x] = self.v[y];
                }
                0x1 => {
                    self.v[x] |= self.v[y];
                }
                0x2 => {
                    self.v[x] &= self.v[y];
                }
                0x3 => {
                    self.v[x] ^= self.v[y];
                }
                0x4 => {
                    if self.v[x] + self.v[y] > 0xFF as u8 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x] = self.v[x].wrapping_add(self.v[y]);
                }
                0x5 => {
                    if self.v[x] > self.v[y] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                }
                0x6 => {
                    self.v[0xF] = self.v[x] & 0x000F;
                    self.v[x] >>= 1;
                }
                0x7 => {
                    if self.v[y] > self.v[x] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                }
                0xE => {
                    self.v[0xF] = self.v[x] & 0xF0;
                    self.v[x] <<= 1;
                }
                _ => self.handle_unimplemented_opcode(opcode),
            },
            0x9 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                self.i = nnn;
            }
            0xB => {
                self.pc = nnn + self.v[0x0] as u16;
            }
            0xC => {
                self.v[x] = nn & rand::thread_rng().gen::<u8>();
            }
            0xD => {
                // Get x and y coords
                let x_coord = self.v[x] as usize % WIDTH;
                let y_coord = self.v[y] as usize % HEIGHT;

                // Unset flag register
                self.v[0xF] = 0;

                let mut j = 0;

                // For each row in height of sprite
                for row in 0..n {
                    // Stop drawing this row if we reach the end of the screen
                    if y_coord + n >= HEIGHT {
                        break;
                    }

                    // Get sprite from memory
                    let sprite = self.memory[(self.i + j) as usize];

                    // For each column in sprite width (8)
                    for col in 0..8 {
                        // Stop drawing this column if we reach the end of the screen
                        if x_coord + col >= WIDTH {
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

                self.display.draw(&self.pixels);
            }
            0xE => match opcode & 0x00FF {
                0x9E => {
                    // TODO: Implement instruction
                }
                0xA1 => {
                    // TODO: Implement instruction
                }
                _ => self.handle_unimplemented_opcode(opcode),
            },
            0xF => match opcode & 0x00FF {
                0x07 => {
                    self.v[x] = self.delay_timer;
                }
                0x15 => {
                    self.delay_timer = self.v[x];
                }
                0x18 => {
                    self.sound_timer = self.v[x];
                }
                0x1E => {
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }
                0x0A => {
                    // TODO: Implement instruction
                }
                0x29 => {
                    self.i = (self.v[x] * 0x5) as u16;
                }
                0x33 => {
                    self.memory[(self.i) as usize] = self.v[x] / 100;
                    self.memory[(self.i + 1) as usize] = self.v[x] % 100 / 10;
                    self.memory[(self.i + 2) as usize] = self.v[x] % 10;
                }
                0x55 => {
                    for i in 0..(x + 1) {
                        self.memory[i + self.i as usize] = self.v[i];
                    }
                }
                0x65 => {
                    for i in 0..(x + 1) {
                        self.v[i] = self.memory[i + self.i as usize];
                    }
                }
                _ => self.handle_unimplemented_opcode(opcode),
            },
            _ => self.handle_unimplemented_opcode(opcode),
        }
    }

    pub fn handle_unimplemented_opcode(&mut self, opcode: u16) {
        println!("Unimplemented opcode {:#06X}", opcode);
    }
}
