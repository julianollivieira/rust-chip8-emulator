use crate::lib::cpu::CPU;
use crate::lib::graphics::{HEIGHT, WIDTH};

/*
|  UNKNOWN OPCODE (Panic when encountered)
*/
pub fn unknown_opcode(opcode: u16) {
    panic!("Unknown opcode: {:X}", opcode);
}
/*
|  00E0 - CLS (Clear the display)
*/
pub fn clear_screen(cpu: &mut CPU) {
    cpu.display.clear();
}
/*
|  00EE - RET (Return from a subroutine)
|
|  The interpreter pops the last address from the stack and sets the PC to
|  it.
*/
pub fn return_from_subroutine(cpu: &mut CPU) {
    cpu.pc = cpu.stack.pop().unwrap();
}
/*
|  1NNN - JP NNN (Jump to address NNN)
|
|  The interpreter sets the program counter to nnn.
*/
pub fn jump_to_address(cpu: &mut CPU, address: u16) {
    cpu.pc = address;
}
/*
|  2NNN - CALL NNN (Call subroutine at NNN)
|
|  The interpreter puts the current PC on the top of the stack. The PC is
|  then set to nnn.
*/
pub fn call_subroutine(cpu: &mut CPU, address: u16) {
    cpu.stack.push(cpu.pc);
    cpu.pc = address;
}
/*
|  3XNN - SE VX, BYTE (Skip next instruction if VX equals NN)
|
|  The interpreter compares register VX to NN, and if they are equal,
|  increments the program counter by 2.
*/
pub fn skip_next_if_vx_equals_nn(cpu: &mut CPU, x: u8, nn: u8) {
    if cpu.v[x as usize] == nn {
        cpu.pc += 2;
    }
}
/*
|  4XNN - SNE VX, BYTE (Skip next instruction if VX doesn't equal NN)
|
|  The interpreter compares register VX to NN, and if they are not equal,
|  increments the program counter by 2.
*/
pub fn skip_next_if_vx_not_equals_nn(cpu: &mut CPU, x: u8, nn: u8) {
    if cpu.v[x as usize] != nn {
        cpu.pc += 2;
    }
}
/*
|  5XY0 - SE VX, VY (Skip next instruction if VX equals VY)
|
|  The interpreter compares register VX to register VY, and if they are
|  equal, increments the program counter by 2.
*/
pub fn skip_next_if_vx_equals_vy(cpu: &mut CPU, x: u8, y: u8) {
    if cpu.v[x as usize] == cpu.v[y as usize] {
        cpu.pc += 2;
    }
}
/*
|  6XNN - LD VX, BYTE (Set VX to NN)
|
|  The interpreter puts the value NN into register VX.
*/
pub fn set_vx_to_nn(cpu: &mut CPU, x: u8, nn: u8) {
    cpu.v[x as usize] = nn;
}
/*
|  7XNN - ADD VX, BYTE (Add NN to VX)
|
|  Adds the value NN to the value of register VX, then stores the result in VX.
*/
pub fn add_nn_to_vx(cpu: &mut CPU, x: u8, nn: u8) {
    cpu.v[x as usize] = cpu.v[x as usize].wrapping_add(nn);
}
/*
|  8XY0 - LD VX, VY (Set VX to VY)
|
|  Stores the value of register VY in register VX.
*/
pub fn set_vx_to_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.v[x as usize] = cpu.v[y as usize];
}
/*
|  8XY1 - OR VX, VY (Set VX to VX or VY)
|
|  Performs a bitwise OR on the values of VX and VY, then stores the result
|  in VX. A bitwise OR compares the corrseponding bits from two values, and
|  if either bit is 1, then the same bit in the result is also 1.
|  Otherwise, it is 0.
*/
pub fn set_vx_to_vx_or_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.v[x as usize] |= cpu.v[y as usize];
}
/*
|  8XY2 - AND VX, VY (Set VX to VX and VY)
|
|  Performs a bitwise AND on the values of VX and VY, then stores the
|  result in VX. A bitwise AND compares the corrseponding bits from two
|  values, and if both bits are 1, then the same bit in the result is also
|  1. Otherwise, it is 0.
*/
pub fn set_vx_to_vx_and_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.v[x as usize] &= cpu.v[y as usize];
}
/*
|  8XY3 - XOR VX, VY (Set VX to VX xor VY)
|
|  Performs a bitwise exclusive OR on the values of VX and VY, then stores
|  the result in VX. An exclusive OR compares the corrseponding bits from
|  two values, and if the bits are not both the same, then the
|  corresponding bit in the result is set to 1. Otherwise, it is 0.
*/
pub fn set_vx_to_vx_xor_vy(cpu: &mut CPU, x: u8, y: u8) {
    cpu.v[x as usize] ^= cpu.v[y as usize];
}
/*
|  8XY4 - ADD VX, VY (Set VX to VX + VY, set VF to carry)
|
|  The values of VX and VY are added together. If the result is greater
|  than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest
|  8 bits of the result are kept, and stored in VX.
*/
pub fn add_vy_to_vx(cpu: &mut CPU, x: u8, y: u8) {
    let (result, carry) = cpu.v[x as usize].overflowing_add(cpu.v[y as usize]);
    cpu.v[x as usize] = result;
    cpu.v[0xF] = if carry { 1 } else { 0 };
}
/*
|  8XY5 - SUB VX, VY (Set VX to VX - VY, set VF to NOT borrow)
|
|  If VX > VY, then VF is set to 1, otherwise 0. Then VY is subtracted from
|  VX, and the results stored in VX.
*/
pub fn set_vx_to_vx_minus_vy(cpu: &mut CPU, x: u8, y: u8) {
    let (result, borrow) = cpu.v[x as usize].overflowing_sub(cpu.v[y as usize]);
    cpu.v[x as usize] = result;
    cpu.v[0xF] = if borrow { 0 } else { 1 };
}
/*
|  !AMBIGUOUS!
|  8XY6 - SHR VX {, VY} (Set VX to VX SHR 1)
|
|  If the least-significant bit of VX is 1, then VF is set to 1, otherwise
|  0. Then VX is divided by 2.
*/
pub fn shift_vx_right_by_one(cpu: &mut CPU, x: u8, y: u8) {
    if cpu.options.put_value_of_vy_into_vx_before_shifting {
        cpu.v[x as usize] = cpu.v[y as usize];
    }
    cpu.v[0xF] = cpu.v[x as usize] & 0x000F;
    cpu.v[x as usize] >>= 1;
}
/*
|  8XY7 - SUBN VX, VY (Set VX to VY - VX, set VF to NOT borrow)
|
|  If VY > VX, then VF is set to 1, otherwise 0. Then VX is subtracted from
|  VY, and the results stored in VX.
*/
pub fn set_vx_to_vy_minus_vx(cpu: &mut CPU, x: u8, y: u8) {
    let (result, borrow) = cpu.v[y as usize].overflowing_sub(cpu.v[x as usize]);
    cpu.v[x as usize] = result;
    cpu.v[0xF] = if borrow { 0 } else { 1 };
}
/*
|  !AMBIGUOUS!
|  8XYE - SHL VX {, VY} (Set VX to VX SHL 1)
|
|  If the most-significant bit of VX is 1, then VF is set to 1, otherwise
|  to 0. Then VX is multiplied by 2.
*/
pub fn shift_vx_left_by_one(cpu: &mut CPU, x: u8, y: u8) {
    if cpu.options.put_value_of_vy_into_vx_before_shifting {
        cpu.v[x as usize] = cpu.v[y as usize];
    }
    cpu.v[0xF] = cpu.v[x as usize] & 0xF0;
    cpu.v[x as usize] <<= 1;
}
/*
|  9XY0 - SNE VX, VY (Skip next instruction if VX != VY)
|
|  The values of VX and VY are compared, and if they are not equal, the
|  program counter is increased by 2.
*/
pub fn skip_next_if_vx_not_equals_vy(cpu: &mut CPU, x: u8, y: u8) {
    if cpu.v[x as usize] != cpu.v[y as usize] {
        cpu.pc += 2;
    }
}
/*
|  ANNN - LD I, ADDR (Set I to NNN)
|
|  The value of register I is set to NNN.
*/
pub fn set_i_to_nnn(cpu: &mut CPU, nnn: u16) {
    cpu.i = nnn;
}
/*
|  !AMBIGUOUS!
|  BNNN - JP V0, ADDR
|
|  The program counter is set to NNN plus the value of V0.
*/
pub fn jump_to_address_plus_v0(cpu: &mut CPU, x: u8, address: u16) {
    if cpu.options.jump_to_nnn_plus_the_value_in_v0 {
        cpu.pc = address + cpu.v[0x0] as u16;
    } else {
        cpu.pc = address + cpu.v[x as usize] as u16;
    }
}
/*
|  CXNN - RND VX, BYTE (Set VX to a random number AND NN)
|
|  The interpreter generates a random number from 0 to 255, which is then
|  ANDed with the value NN. The results are stored in VX. See instruction
|  8XY2 for more information on AND.
*/
pub fn set_vx_to_random_number_and_nn(cpu: &mut CPU, x: u8, nn: u8) {
    cpu.v[x as usize] = rand::random::<u8>() & nn;
}
/*
|  DXYN - DRW VX, VY, NIBBLE (Display N-byte sprite starting at memory location I at (VX, VY), set VF = collision)
|
|  The interpreter reads N bytes from memory, starting at the address
|  stored in I. These bytes are then displayed as sprites on screen at
|  coordinates (VX, VY). Sprites are XORed onto the existing screen. If
|  this causes any pixels to be erased, VF is set to 1, otherwise it is set
|  to 0. If the sprite is positioned so part of it is outside the
|  coordinates of the display, it wraps around to the opposite side of the
|  screen. See instruction 8XY3 for more information on XOR.
*/
pub fn draw_sprite(cpu: &mut CPU, x: u8, y: u8, n: u8) {
    // Get x and y coords
    let x_coord = cpu.v[x as usize] as usize % WIDTH;
    let y_coord = cpu.v[y as usize] as usize % HEIGHT;

    // Unset flag register
    cpu.v[0xF] = 0;

    let mut j = 0;

    // For each row in height of sprite
    for row in 0..n {
        // Stop drawing this row if we reach the end of the screen
        if y_coord + n as usize >= HEIGHT {
            break;
        }

        // Get sprite from memory
        let sprite = cpu.memory[(cpu.i + j) as usize];

        // For each column in sprite width (8)
        for col in 0..8 {
            // Stop drawing this column if we reach the end of the screen
            if x_coord + col >= WIDTH {
                break;
            }

            // Set pixel on/off
            let old_pixel = cpu.pixels[y_coord + row as usize][x_coord + col];
            let new_pixel = (sprite & (1 << (7 - col))) != 0;
            cpu.pixels[y_coord + row as usize][x_coord + col] = old_pixel ^ new_pixel;
            cpu.v[0xF] |= cpu.pixels[y_coord + row as usize][x_coord + col] as u8;
        }

        j += 1;
    }

    cpu.display.draw(&cpu.pixels);
}
/*
|  EX9E - SKP VX (Skip next instruction if key with the value of VX is pressed)
|
|  Checks the keyboard, and if the key corresponding to the value of Vx is
|  currently in the down position, PC is increased by 2.
*/
pub fn skip_next_if_key_is_pressed(cpu: &mut CPU, x: u8) {
    //
}
/*
|  EXA1 - SKNP VX (Skip next instruction if key with the value of VX is not pressed)
|
|  Checks the keyboard, and if the key corresponding to the value of VX is
|  currently in the up position, PC is increased by 2.
*/
pub fn skip_next_if_key_is_not_pressed(cpu: &mut CPU, x: u8) {
    //
}
/*
|  FX07 - LD VX, DT (Set VX to the value of the delay timer)
|
|  The value of DT is placed into VX.
*/
pub fn set_vx_to_delay_timer(cpu: &mut CPU, x: u8) {
    cpu.v[x as usize] = cpu.delay_timer;
}
/*
|  FX0A - LD VX, K (Wait for a key press, store the value of the key in VX)
|
|  All execution stops until a key is pressed, then the value of that key
|  is stored in VX.
*/
pub fn wait_for_keypress(cpu: &mut CPU, x: u8) {
    //
}
/*
|  FX15 - LD DT, VX (Set the delay timer to VX)
|
|  DT is set equal to the value of VX.
*/
pub fn set_delay_timer_to_vx(cpu: &mut CPU, x: u8) {
    cpu.delay_timer = cpu.v[x as usize];
}
/*
|  FX18 - LD ST, VX (Set the sound timer to VX)
|
|  ST is set equal to the value of VX.
*/
pub fn set_sound_timer_to_vx(cpu: &mut CPU, x: u8) {
    cpu.sound_timer = cpu.v[x as usize];
}
/*
|  FX1E - ADD I, VX (Set I to the value of I plus the value of VX)
|
|  The values of I and VX are added, and the results are stored in I.
*/
pub fn add_vx_to_i(cpu: &mut CPU, x: u8) {
    cpu.i = cpu.i.wrapping_add(cpu.v[x as usize] as u16);
}
/*
|  FX29 - LD F, VX (Set I to the location of the sprite for the character in VX)
|
|  The value of I is set to the location for the hexadecimal sprite
|  corresponding to the value of VX.
*/
pub fn set_i_to_sprite_location(cpu: &mut CPU, x: u8) {
    cpu.i = cpu.v[x as usize] as u16 * 5;
}
/*
|  FX33 - LD B, VX (Store BCD representation of VX in memory locations I, I+1, and I+2)
|
|  The interpreter takes the decimal value of VX, and places the hundreds
|  digit in memory at location in I, the tens digit at location I+1, and
|  the ones digit at location I+2.
*/
pub fn set_bcd_of_vx_at_i(cpu: &mut CPU, x: u8) {
    cpu.memory[cpu.i as usize] = cpu.v[x as usize] / 100;
    cpu.memory[cpu.i as usize + 1] = (cpu.v[x as usize] / 10) % 10;
    cpu.memory[cpu.i as usize + 2] = cpu.v[x as usize] % 10;
}
/*
|  !AMBIGUOUS!
|  FX55 - LD [I], VX (Store registers V0 through VX in memory starting at location I)
|
|  The interpreter copies the values of registers V0 through VX into
|  memory, starting at the address in I.
*/
pub fn store_registers_in_memory(cpu: &mut CPU, x: u8) {
    for i in 0..x + 1 {
        cpu.memory[cpu.i as usize + i as usize] = cpu.v[i as usize];
    }
    if cpu.options.increment_i_when_storing_loading_memory {
        cpu.i = cpu.i.wrapping_add(x as u16 + 1);
    }
}
/*
|  !AMBIGUOUS!
|  FX65 - LD VX, [I] (Read registers V0 through VX from memory starting at location I)
|
|  The interpreter reads values from memory starting at location I into registers V0 through VX.
*/
pub fn load_registers_from_memory(cpu: &mut CPU, x: u8) {
    for i in 0..x + 1 {
        cpu.v[i as usize] = cpu.memory[cpu.i as usize + i as usize];
    }
    if cpu.options.increment_i_when_storing_loading_memory {
        cpu.i = cpu.i.wrapping_add(x as u16 + 1);
    }
}
