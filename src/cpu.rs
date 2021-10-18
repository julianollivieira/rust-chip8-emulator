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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
]

struct CPU {
    memory: [u8; 4096];     // 4 kilobytes of RAM
    pc: u16;                // program counter
    i: u16;                 // index register
    stack: [u16; 16];       // stack
    sp: u8;                 // stack pointer
    delay_timer: u8;        // delay timer (decrement 60 times a sec until 0)
    sound_timer: u8;        // sound timer (same as delay_timer but beep while not 0)
    v: [u8; 16];            // general purpose variable registers V0 through VF
}

impl CPU {
    pub fn new() -> CPU {

        let mut memory = [0; 4096];         // initialize memory

        for i in 0..80 {
            memory[i + 80] = FONT[i]        // load font into memory at 050 -> 09F
        }
        
        CPU {                               // initialize CPU
            memory: memory,
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16]
        }

    }
}
