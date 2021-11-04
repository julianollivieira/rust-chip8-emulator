use crate::cpu::{CPU};

pub fn log_instruction(cpu: &mut CPU, opcode: u16, nnn: u16, nn: u8, n: usize, x: usize, y: usize) {
    match opcode >> 12 {
        0x0 => match opcode & 0x00FF {
            0xE0 => println!("CLEAR"),
            0xEE => println!("RETURN FROM SUBROUTINE"),
            _ => handle_unimplemented_opcode_debug_message(opcode),
        },
        0x1 => println!("JUMP TO [{}]", nnn),
        0x2 => println!("CALL SUBROUTINE AT [{}]", nnn),
        0x3 => println!("SKIP ONE INSTRUCTION IF [V{}, {}] IS EQUAL TO [{}]", x, cpu.v[x], nn),
        0x4 => println!("SKIP ONE INSTRUCTION IF [V{}, {}] IS NOT EQUAL TO [{}]", x, cpu.v[x], nn),
        0x5 => println!("SKIP ONE INSTRUCTION IF [V{}, {}] IS EQUAL TO [V{}, {}]", x, cpu.v[x], y, cpu.v[y]),
        0x6 => println!("SET REGISTER [V{}, {}] TO [{}]", x, cpu.v[x], nn),
        0x7 => println!("ADD [{}] TO REGISTER [V{}, {}]", nn, x, cpu.v[x]),
        0x8 => match opcode & 0x000F {
            0x0 => println!("SET [V{}, {}] TO [V{}, {}]", x, cpu.v[x], y, cpu.v[y]),
            0x1 => println!("SET [V{}, {}] TO BITWISE OR OF [V{}, {}] AND [V{}, {}]", x, cpu.v[x], x, cpu.v[x], y, cpu.v[y]),
            0x2 => println!("SET [V{}, {}] TO BITWISE AND OF [V{}, {}] AND [V{}, {}]", x, cpu.v[x], x, cpu.v[x], y, cpu.v[y]),
            0x3 => println!("SET [V{}, {}] TO BITWISE XOR OF [V{}, {}] AND [V{}, {}]", x, cpu.v[x], x, cpu.v[x], y, cpu.v[y]),
            0x4 => println!("ADD [V{}, {}] TO REGISTER [V{}, {}] AND SET CARRY TO SOMETHING", x, cpu.v[x], y, cpu.v[y]),
            0x5 => {  },
            0x6 => {  },
            0x7 => {  },
            0xE => {  },
            _ => handle_unimplemented_opcode_debug_message(opcode),
        },
        0x9 => {  },
        0xA => {  },
        0xB => {  },
        0xC => {  },
        0xD => {  },
        0xE => {  },
        0xF => match opcode & 0x00FF { 
            0x07 => {  },
            0x15 => {  },
            0x18 => {  },
            0x1E => {  },
            0x0A => {  },
            0x29 => {  },
            0x33 => {  },
            0x55 => {  },
            0x65 => {  },
            _ => handle_unimplemented_opcode_debug_message(opcode),
        },
        _ => handle_unimplemented_opcode_debug_message(opcode),
    }
}

pub fn handle_unimplemented_opcode_debug_message(opcode: u16) {
    println!("Unimplemented opcode debug message {:#0X}", opcode);
}