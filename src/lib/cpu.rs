use crate::lib::font::FONT;
use crate::lib::graphics::{Display, HEIGHT, WIDTH};
use crate::lib::ops;

pub struct Options {
    pub put_value_of_vy_into_vx_before_shifting: bool,
    pub jump_to_nnn_plus_the_value_in_v0: bool,
    pub increment_i_when_storing_loading_memory: bool,
}

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
    pub options: Options,                // Extra options for compatibility
}

impl CPU {
    pub fn new(display: Display, options: Options) -> CPU {
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
            options,
        }
    }
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // Load the ROM into memory starting at 0x200
        let rom_length = rom.len();
        for i in 0..rom_length {
            self.memory[0x200 + i] = rom[i];
        }
    }
    pub fn step(&mut self) {
        // Fetch instruction that the PC is currently pointing to from memory
        let first_opcode_byte = self.memory[self.pc as usize];
        let second_opcode_byte = self.memory[self.pc as usize + 1];
        let opcode = (first_opcode_byte as u16) << 8 | second_opcode_byte as u16;

        // Increment PC
        self.pc += 2;

        // Execute instruction
        self.execute_instruction(opcode);
    }
    pub fn execute_instruction(&mut self, opcode: u16) {
        /*
        | nnn = 0000NNNN NNNNNNNN | low byte + lower 4 bits of high byte
        |  nn = 00000000 NNNNNNNN | low byte
        |   n = 00000000 0000NNNN | lower 4 bits of low byte
        |   x = 0000XXXX 00000000 | lower 4 bits of high byte
        |   y = 00000000 YYYY0000 | upper 4 bits of low byte
        */
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        // Print opcode in hex
        println!("{:#02X}", opcode);

        match opcode >> 12 {
            0x0 => match nn {
                0x00 => (),
                0xE0 => ops::clear_screen(self),
                0xEE => ops::return_from_subroutine(self),
                _ => ops::unknown_opcode(opcode),
            },
            0x1 => ops::jump_to_address(self, nnn),
            0x2 => ops::call_subroutine(self, nnn),
            0x3 => ops::skip_next_if_vx_equals_nn(self, x, nn),
            0x4 => ops::skip_next_if_vx_not_equals_nn(self, x, nn),
            0x5 => ops::skip_next_if_vx_equals_vy(self, x, y),
            0x6 => ops::set_vx_to_nn(self, x, nn),
            0x7 => ops::add_nn_to_vx(self, x, nn),
            0x8 => match n {
                0x0 => ops::set_vx_to_vy(self, x, y),
                0x1 => ops::set_vx_to_vx_or_vy(self, x, y),
                0x2 => ops::set_vx_to_vx_and_vy(self, x, y),
                0x3 => ops::set_vx_to_vx_xor_vy(self, x, y),
                0x4 => ops::add_vy_to_vx(self, x, y),
                0x5 => ops::set_vx_to_vx_minus_vy(self, x, y),
                0x6 => ops::shift_vx_right_by_one(self, x, y),
                0x7 => ops::set_vx_to_vy_minus_vx(self, x, y),
                0xE => ops::shift_vx_left_by_one(self, x, y),
                _ => ops::unknown_opcode(opcode),
            },
            0x9 => ops::skip_next_if_vx_not_equals_vy(self, x, y),
            0xA => ops::set_i_to_nnn(self, nnn),
            0xB => ops::jump_to_address_plus_v0(self, x, nnn),
            0xC => ops::set_vx_to_random_number_and_nn(self, x, nn),
            0xD => ops::draw_sprite(self, x, y, n),
            0xE => match nn {
                0x9E => ops::skip_next_if_key_is_pressed(self, x),
                0xA1 => ops::skip_next_if_key_is_not_pressed(self, x),
                _ => ops::unknown_opcode(opcode),
            },
            0xF => match nn {
                0x07 => ops::set_vx_to_delay_timer(self, x),
                0x0A => ops::wait_for_keypress(self, x),
                0x15 => ops::set_delay_timer_to_vx(self, x),
                0x18 => ops::set_sound_timer_to_vx(self, x),
                0x1E => ops::add_vx_to_i(self, x),
                0x29 => ops::set_i_to_sprite_location(self, x),
                0x33 => ops::set_bcd_of_vx_at_i(self, x),
                0x55 => ops::store_registers_in_memory(self, x),
                0x65 => ops::load_registers_from_memory(self, x),
                _ => ops::unknown_opcode(opcode),
            },
            _ => ops::unknown_opcode(opcode),
        }
    }
}
