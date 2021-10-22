pub fn log_instruction(opcode: u16, nnn: u16, nn: u8, n: usize, x: usize, y: usize) {
    match opcode & 0xF000 {
        0x0000 => match opcode & 0x00FF {
            0x00E0 => println!("CLEAR"),
            0x00EE => println!("RETURN FROM SUBROUTINE"),
        },
        0x1000 => println!("JUMP TO [{:#0X}]", nnn),
        0x2000 => println!("CALL SUBROUTINE AT {}", nnn),
        0x3000 => println!("SKIP IF VX [{}] IS EQUAL TO NN [{}] {}", self.v[x], nn, self.v[x] == nn),
        0x4000 => println!("SKIP IF VX [{}] IS NOT EQUAL TO NN [{}] {}", self.v[x], nn, self.v[x] != nn),
        0x5000 => println!("SKIP IF VX [{}] IS EQUAL TO VY [{}] {}", self.v[x], self.v[y], self.v[x] == self.v[y]),
        0x6000 => println!("SET REGISTER [V{:#}] TO [{:#0X}]", x, nn),
        0x7000 => println!("ADD [{:#0X}] TO REGISTER [V{:#}]", nn, x),
        0x8000 => match opcode & 0x000F {
            0x0001 =>  println!("SET VX {} TO VY {}", self.v[x], self.v[y]),
        },
        0x9000 => println!("SKIP IF VX [{}] IS NOT EQUAL TO VY [{}] {}", self.v[x], self.v[y], self.v[x] != self.v[y]),
        0xA000 => println!("SET INDEX REGISTER I TO [{:#0X}]", nnn),
        0xD000 => println!("DRAW SPRITE AT [{}, {}] WITH HEIGHT OF [{}]", x, y, n),
    }
}