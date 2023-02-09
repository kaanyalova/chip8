// shut up clippy
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rand::random;

pub struct chip8_plugin;

pub struct Emulator {
    pub display_buffer: [[bool; 32]; 64], // first index is x, second is y
    pub key_inputs: [bool; 16],
    pub memory: [u8; 4096],
    registers: [u16; 16], // stack
    sound_timer: u8,
    delay_timer: u8,
    index_register: u16, // cpu pointer thing
    program_counter: u16,
    stack_pointer: u16,
    //wait_for_key: bool, // not needed , we can check if the value of key_inst_buffer is zero or not instead
    pub wait_for_key: bool,
    pub draw: bool,
}

impl Emulator {
    #[allow(dead_code)]
    pub const FONTS: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0  // starts at 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1  // starts at 5
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2  // starts at 10
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

    pub fn init(rom: Vec<u8>) -> Self {
        Self {
            display_buffer: [[false; 32]; 64],
            key_inputs: [false; 16],
            memory: {
                let mut mem = [0; 4096];

                /*
                Does the same as

                while i < 80:
                # load 80-char font set
                    self.memory[i] = self.fonts[i]
                    i += 1

                */
                mem[..80].clone_from_slice(&Emulator::FONTS);

                for (offset, byte) in rom.iter().enumerate() {
                    mem[0x200 + offset] = byte.clone();
                }

                //println!("{:?}", mem);

                //for m in mem {
                //    println!("0x{:X},", m)
                //}
                mem
            },
            registers: [0; 16],
            sound_timer: 0,
            delay_timer: 0,
            index_register: 512,
            program_counter: 0x200,
            stack_pointer: 0,
            wait_for_key: false,
            draw: false,
        }
    }
    /*
        fn push(&mut self, input: u16) {
            self.registers[self.index_register as usize] = input;
            self.index_register += 1
        }

        fn pop(&mut self) -> u16 {
            self.index_register -= 1;
            self.registers[self.index_register as usize]
        }
    */

    pub fn timer_tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }
    }
    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    /// Combines 2 u8 "bytes" to get a u16 "2 bytes"
    /// More info here http://www.codeslinger.co.uk/pages/projects/chip8/fetchdecode.html
    ///
    ///    Explanation:
    ///    to combine 00010001, 11101110
    ///
    ///    Make space for
    ///    00010001 => 00010001 00000000  (becasuse of <<= 8)
    ///
    ///    00010001 00000000 | (00000000) 11101110 => 00010001 11101110
    fn fetch(&mut self) -> u16 {
        let mut res = self.memory[self.program_counter as usize] as u16;
        res <<= 8;
        res = res | self.memory[self.program_counter as usize + 1] as u16;
        self.program_counter += 2;
        res
    }

    /// Explanation of getting "pieces":
    ///
    ///
    /// For example:
    /// getting the third "piece"
    ///
    /// opcode =
    /// 00E0 => 0000 0000 1110 0000
    /// comparasion =
    /// 00F0 => 0000 0000 1111 0000
    ///  
    /// 0000 0000 1110 0000
    /// 0000 0000 1111 0000
    /// &
    /// --------------------------
    /// 0000 0000 1110 0000 => 1110 0000
    ///
    /// then we right-shift 4 times to get rid of 0000
    /// we get 1110 which is 3rd "piece"
    ///

    pub fn execute(&mut self, opcode: u16) {
        println!("{:X}", opcode);
        let pieces = (
            (opcode & 0xF000) >> 12, // as u8 ?????
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );
        match pieces {
            (0x0, 0x0, 0xE, 0x0) => self.inst_00E0(), //00E0
            (0x0, 0x0, 0xE, 0xE) => self.inst_00EE(), //00EE
            (0x1, _, _, _) => self.inst_1nnn(pieces.1, pieces.2, pieces.3), //1nnn
            (0x2, _, _, _) => self.inst_2nnn(pieces.1, pieces.2, pieces.3), //2nnn
            (0x3, _, _, _) => self.inst_3xkk(pieces.1, pieces.2, pieces.3), // 3xkk
            (0x4, _, _, _) => self.inst_4xkk(pieces.1, pieces.2, pieces.3), //4xkk
            (0x5, _, _, 0x0) => self.inst_5xy0(pieces.1, pieces.2), // 5xy0
            (0x6, _, _, _) => self.inst_6xkk(pieces.1, pieces.2, pieces.3), //6xkk
            (0x7, _, _, _) => self.inst_7xkk(pieces.1, pieces.2, pieces.3), // 7xkk
            (0x8, _, _, 0x0) => self.inst_8xy0(pieces.1, pieces.2), // 8xy0
            (0x8, _, _, 0x1) => self.inst_8xy1(pieces.1, pieces.2), // 8xy1
            (0x8, _, _, 0x2) => self.inst_8xy2(pieces.1, pieces.2), // 8xy2
            (0x8, _, _, 0x3) => self.inst_8xy3(pieces.1, pieces.2), // 8xy3
            (0x8, _, _, 0x4) => self.inst_8xy4(pieces.1, pieces.2), // 8xy4
            (0x8, _, _, 0x5) => self.inst_8xy5(pieces.1, pieces.2), // 8xy5
            (0x8, _, _, 0x6) => self.inst_8xy6(pieces.1), // 8xy6
            (0x8, _, _, 0x7) => self.inst_8xy7(pieces.1, pieces.2), // 8xy7
            (0x8, _, _, 0xE) => self.inst_8xyE(pieces.1), // 8xyE
            (0x9, _, _, 0x0) => self.inst_9xy0(pieces.1, pieces.2), // 9xy0
            (0xA, _, _, _) => self.inst_Annn(pieces.1, pieces.2, pieces.3), // Ann
            (0xB, _, _, _) => self.inst_Bnnn(pieces.1, pieces.2, pieces.3), // Bnn
            (0xC, _, _, _) => self.inst_Cxkk(pieces.1, pieces.2, pieces.3),
            (0xD, _, _, _) => self.inst_Dxyn(pieces.1, pieces.2, pieces.3), // Dxyn
            (0xE, _, 0x9, 0xE) => self.inst_Ex9E(pieces.1),                 // Ex9E
            (0xE, _, 0xA, 0x1) => self.inst_ExA1(pieces.1),                 // ExA1
            (0xF, _, 0x0, 0x7) => self.inst_Fx07(pieces.1),                 // Fx07
            (0xF, _, 0x0, 0xA) => self.inst_Fx0A(pieces.1),                 // Fx0A
            (0xF, _, 0x1, 0x5) => self.inst_Fx15(pieces.1),                 // Fx15
            (0xF, _, 0x1, 0x8) => self.inst_Fx18(pieces.1),                 // Fx18
            (0xF, _, 0x1, 0xE) => self.inst_Fx1E(pieces.1),                 // Fx1E
            (0xF, _, 0x2, 0x9) => self.inst_Fx29(pieces.1),                 // Fx29
            (0xF, _, 0x3, 0x3) => self.inst_Fx33(pieces.1),                 // Fx33
            (0xF, _, 0x5, 0x5) => self.inst_Fx55(pieces.1),                 // Fx55
            (0xF, _, 0x6, 0x5) => self.inst_Fx65(pieces.1),                 // Fx65

            (_, _, _, _) => panic!(
                "no instruction found, opcode 0x{:X}, at pc {}",
                opcode, self.program_counter
            ),
        };
    }
}

// Instructions
impl Emulator {
    /// 00E0
    ///
    /// Clear the display.
    fn inst_00E0(&mut self) {
        self.display_buffer = [[false; 32]; 64];
        self.draw = true;
    }

    /// 00EE
    ///
    /// Return from a subroutine.
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn inst_00EE(&mut self) {
        self.program_counter = self.registers[self.stack_pointer as usize];
        self.stack_pointer -= 1;
    }

    /// 1nnn
    ///
    /// Jump to location nnn.
    /// The interpreter sets the program counter to nnn.
    fn inst_1nnn(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut bin = 0x000;
        bin |= t2 << 8;
        bin |= t3 << 4;
        bin |= t4;
        self.program_counter = bin as u16;
    }

    /// 2nnn
    ///
    /// Call subroutine at nnn.
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn inst_2nnn(&mut self, t2: u16, t3: u16, t4: u16) {
        self.stack_pointer += 1;
        self.registers[self.stack_pointer as usize] = self.program_counter;
        self.inst_1nnn(t2, t3, t4);
    }

    /// 3xkk
    ///
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn inst_3xkk(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut kk = t3 << 4;
        kk |= t4;
        if self.registers[t2 as usize] == kk {
            self.program_counter += 2;
        }
    }

    /// 4xkk
    ///
    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    /// opposite of 3xkk
    fn inst_4xkk(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut kk = t3 << 4;
        kk |= t4;

        if self.registers[t2 as usize] != kk {
            self.program_counter += 2;
        }
    }

    ///5xy0
    ///
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn inst_5xy0(&mut self, t2: u16, t3: u16) {
        if self.registers[t2 as usize] == self.registers[t3 as usize] {
            self.program_counter += 2
        }
    }

    /// 6xkk
    ///
    /// Set Vx = kk.
    /// The interpreter puts the value kk into register Vx.
    fn inst_6xkk(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut kk = t3 << 4;
        kk |= t4;
        self.registers[t2 as usize] = kk;
    }

    /// 7xkk
    ///
    /// Set Vx = Vx + kk.
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn inst_7xkk(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut kk = t3 << 4;
        kk |= t4;
        (self.registers[t2 as usize], _) = kk.overflowing_add(self.registers[t2 as usize]);
    }

    /// 8xy0
    ///
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    fn inst_8xy0(&mut self, t2: u16, t3: u16) {
        self.registers[t2 as usize] = self.registers[t3 as usize].clone() // .clone() because rust
    }

    /// 8xy1
    ///
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn inst_8xy1(&mut self, t2: u16, t3: u16) {
        self.registers[t2 as usize] = self.registers[t2 as usize] | self.registers[t3 as usize];
    }

    /// 8xy2
    ///
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn inst_8xy2(&mut self, t2: u16, t3: u16) {
        self.registers[t2 as usize] = self.registers[t2 as usize] & self.registers[t3 as usize];
    }

    /// 8xy3
    ///
    ///  Set Vx = Vx XOR Vy
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn inst_8xy3(&mut self, t2: u16, t3: u16) {
        self.registers[t2 as usize] = self.registers[t2 as usize] ^ self.registers[t3 as usize];
    }

    /// 8xy4
    ///
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn inst_8xy4(&mut self, t2: u16, t3: u16) {
        let mut sum = t2 + t3;
        if sum > 255 {
            self.registers[0xF] = 1; // 0xF = 15
                                     //
                                     // sum
                                     //
                                     // 0000 0001 0000 1110
                                     // 0000 0000 0000 0000
                                     // |
                                     // ---------------------
                                     // 0000 0001 0000 1110

            sum |= 0x0000;
            sum <<= 8;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[t2 as usize] = sum;
    }

    /// 8xy5
    ///
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn inst_8xy5(&mut self, t2: u16, t3: u16) {
        if t2 > t3 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        (self.registers[t2 as usize], _) =
            self.registers[t2 as usize].overflowing_sub(self.registers[t3 as usize]);
    }

    /// 8xy6  
    ///
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    /// https://stackoverflow.com/questions/69819960/how-to-find-most-and-least-significant-bits-of-an-unsigned-number-u64-in-rust
    ///
    /// least significant bit is one on the right most
    /// n & 1
    ///
    /// 1011
    /// 0001
    /// &
    /// ----
    /// 0001
    ///
    ///
    fn inst_8xy6(&mut self, t2: u16) {
        // y unused
        if (self.registers[t2 as usize] & 1) == 1 {
            self.registers[0xF] = 1
        } else {
            self.registers[0xF] = 0
        }
        self.registers[t2 as usize] /= 2;
    }

    /// 8xy7
    ///
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn inst_8xy7(&mut self, t2: u16, t3: u16) {
        if self.registers[t2 as usize] > self.registers[t3 as usize] {
            self.registers[0xF] = 1
        } else {
            self.registers[0xF] = 0
        }

        (self.registers[t2 as usize], _) =
            self.registers[t2 as usize].overflowing_sub(self.registers[t3 as usize]);
    }

    /// 8xyE
    ///
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn inst_8xyE(&mut self, t2: u16) {
        if ((self.registers[t2 as usize] >> 15) & 1) == 1 {
            // similar to 8xy6
            self.registers[0xF] = 1
        } else {
            self.registers[0xF] = 0
        }
        self.registers[t2 as usize] *= 2
    }

    /// 9xy0
    ///
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn inst_9xy0(&mut self, t2: u16, t3: u16) {
        if self.registers[t2 as usize] != self.registers[t3 as usize] {
            self.program_counter += 2
        }
    }

    /// Annn    
    ///
    /// Set I = nnn.
    /// The value of register I is set to nnn.
    fn inst_Annn(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut bin = 0x000;
        bin |= t2 << 8;
        bin |= t3 << 4;
        bin |= t4;
        self.index_register = bin;
    }

    /// Bnnn
    ///
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    fn inst_Bnnn(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut bin = 0x000;
        bin |= t2 << 8;
        bin |= t3 << 4;
        bin |= t4;
        self.program_counter = bin + self.registers[0x0]
    }

    /// Cxkk - RND Vx, byte
    ///
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn inst_Cxkk(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut kk = t3 << 4;
        kk |= t4;
        let rand: u8 = random();
        self.registers[t2 as usize] = kk & rand as u16;
    }

    /// Dxyn
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    ///
    ///
    /// The sprite themselfs don't actually wrap, only the starting cordinates (Vx, Vy) do
    ///
    ///
    /// so actual starting positions are:
    /// Vx % 64 and (Vy % 32) + "index of the byte we are iterating(0..n)"
    ///
    /// sprites are always 8 bits "wide" and n bits "long"
    ///
    /// for example Dxyn
    ///
    /// memory has n bytes 01010001 ,11110000, 11111111, 00000000, ... starting at memory location VI
    /// (starts at x:x y:y)
    ///
    ///
    ///
    ///
    ///                Sprite           Screen          Screen(Next frame)
    ///                  
    ///                Vy
    /// 1st byte    Vx 01010001   XOR   01010101        00000100
    /// 2nd byte       11110000    ^    11110000   ==>  00000000
    /// 3rd byte       11111111         00001111        11111111
    /// 4th byte       00000000         11111111        11111111
    /// ...            ...              ...             ...
    /// ...            ...              ...             ...
    ///
    /// more info at https://www.reddit.com/r/EmuDev/comments/sa5cyf/eli5_how_chip8_display_work/
    fn inst_Dxyn(&mut self, t2: u16, t3: u16, t4: u16) {
        let mut is_changed = false;

        // iterate for each byte of sprite
        for sprite_byte_iter in 0..t4 {
            let sprite = self.memory[(self.index_register + sprite_byte_iter) as usize]; // sprite byte
            let x_pos = self.registers[t2 as usize] % 64; // actual x position
            let y_pos = (self.registers[t3 as usize] % 32) + sprite_byte_iter; // actual y position
                                                                               // sprite_byte_iter added to get the y position of the spites to draw
                                                                               // second sprite needs to start at y+1 third needs to start at y+2 etc.

            // iterate over each bit of the sprite
            // could & with 1000 0000 then << n to get bits insead...
            for bit_iter in 0..8 {
                let bit = (sprite << bit_iter) & 0x80 != 0; // "!= 0" is same as "as bool"
                self.display_buffer[((x_pos + bit_iter) as u16 % 64) as usize]  // %64 again 
                    [y_pos as usize % 32] ^= bit; // xor the bits to display buffer
                if bit {
                    // bit "toggles" only if the bit is 1, so set the VF to 1
                    is_changed = true;
                }
            }
        }

        // set vf to if changed
        self.registers[0xF] = is_changed as u16;
        self.draw = is_changed;
    }

    /// Ex9E
    ///
    /// Skip next instruction if key with the value of Vx is pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    fn inst_Ex9E(&mut self, t2: u16) {
        if self.key_inputs[self.registers[t2 as usize] as usize] {
            self.program_counter += 2;
        }
    }

    /// ExA1
    ///
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    ///
    /// Opposite of Ex9E
    fn inst_ExA1(&mut self, t2: u16) {
        if !self.key_inputs[self.registers[t2 as usize] as usize] {
            self.program_counter += 2;
        }
    }

    /// Fx07
    ///
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    fn inst_Fx07(&mut self, t2: u16) {
        self.registers[t2 as usize] = self.delay_timer as u16
    }

    // Fx0A
    //
    // Wait for a key press, store the value of the key in Vx.
    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn inst_Fx0A(&mut self, t2: u16) {
        self.wait_for_key = true;
        if self.wait_for_key == true {
            for (key_index, key_is_pressed) in self.key_inputs.iter().enumerate() {
                if *key_is_pressed {
                    self.registers[t2 as usize] = key_index as u16;
                    self.wait_for_key = false;
                    return; // this should only get the "first" indexed key ? This should be return or break ?
                }
            }
        }

        if self.wait_for_key {
            self.program_counter -= 2;
        }
    }

    /// Fx15
    ///
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    fn inst_Fx15(&mut self, t2: u16) {
        self.delay_timer = self.registers[t2 as usize] as u8;
    }

    /// Fx18
    ///     
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    fn inst_Fx18(&mut self, t2: u16) {
        self.sound_timer = self.registers[t2 as usize] as u8;
    }

    /// Fx1E
    ///
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn inst_Fx1E(&mut self, t2: u16) {
        (self.index_register, _) = self
            .index_register
            .overflowing_add(self.registers[t2 as usize])
    }

    /// Fx29
    ///
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    fn inst_Fx29(&mut self, t2: u16) {
        self.index_register = self.registers[t2 as usize] * 5 // each sprite is 5 bytes
    }

    /// Fx33
    ///
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn inst_Fx33(&mut self, t2: u16) {
        let vx = self.registers[t2 as usize];
        self.memory[self.index_register as usize] = (vx / 100) as u8;
        self.memory[(self.index_register + 1) as usize] = (vx % 100 / 10) as u8;
        self.memory[(self.index_register + 2) as usize] = (vx % 10 / 10) as u8;
    }

    /// Fx55
    ///
    /// Store registers V0 through Vx in memory starting at location I.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn inst_Fx55(&mut self, t2: u16) {
        for register_iter in 0..t2 {
            self.memory[(self.index_register + register_iter) as usize] =
                self.registers[register_iter as usize] as u8
        }
    }

    /// Fx65
    ///
    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    /// opposite of Fx55
    fn inst_Fx65(&mut self, t2: u16) {
        for register_iter in 0..t2 {
            self.registers[register_iter as usize] =
                self.memory[(self.index_register + register_iter) as usize] as u16
        }
    }
}
