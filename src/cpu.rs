use crate::display::DISPLAY;

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
    memory: [u8; 4096], // 4 kilobytes of RAM
    pc: u16,            // program counter
    i: u16,             // index register
    stack: [u16; 16],   // stack
    sp: u8,             // stack pointer
    delay_timer: u8,    // delay timer (decrement 60 times a sec until 0)
    sound_timer: u8,    // sound timer (same as delay_timer but beep while not 0)
    v: [u8; 16],        // general purpose variable registers V0 through VF
}

impl CPU {
    pub fn new() -> CPU {
        let mut memory = [0; 4096];         // initialize memory

        for i in 0..80 {
            memory[i + 80] = FONT[i];       // load font into memory at 050 -> 09F
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
        }
    }

    pub fn load_bin(&mut self, rom: Vec<u8>) {
        let rom_length = rom.len();
        for i in 0..rom_length {
            self.memory[i + 512] = rom[i];                  // load ROM into memory
        }
    }

    pub fn step(&mut self, DISPLAY: &mut DISPLAY) {
        // Fetch the instruction that PC is currently pointing at from memory
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);

        let nnn: u16 = (opcode & 0x0FFF) as u16;            // 0000NNNN NNNNNNNN | lowest 12 bits
        let kk: u8 = (opcode & 0x00FF) as u8;               // 00000000 KKKKKKKK | lowest 8 bits
        let x = ((opcode & 0x0F00) >> 8) as usize;          // 0000XXXX 00000000 | lower 4 bits of the high byte
        let y = ((opcode & 0x00F0) >> 4) as usize;          // 00000000 YYYY0000 | upper 4 bits of the lower byte
        let n = (opcode & 0x000F) as usize;                 // 00000000 0000NNNN | lowest 4 bits

         // println!("opcode: {:X} | PC: {:#?}", opcode, self.pc);

        self.pc += 2;

        // Decode and execute the instruction
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => {
                    println!("CLEAR");
                }

                _ => panic!("Unimplemented opcode {:?}", opcode),
            },

            0x1000 => {
                println!("JUMP");
            }

            0x6000 => {
                println!("SET REGISTER VX");
            }

            0x7000 => {
                println!("ADD VALUE TO REGISTER VX");
            }

            0xA000 => {
                println!("SET INDEX REGISTER I");
            }

            0xD000 => {
                println!("DISPLAY / DRAW");
            }

            _ => panic!("Unimplemented opcode {:?}", opcode),
        }
    }
}
