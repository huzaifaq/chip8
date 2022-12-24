#[derive(Debug)]
pub struct Chip8InstructionDecoder {
    decoded_instructions: Vec<String>,
}

impl Chip8InstructionDecoder {
    pub fn new() -> Chip8InstructionDecoder {
        Chip8InstructionDecoder {
            decoded_instructions: vec![],
        }
    }

    fn decode_file(&mut self, filename: &str) {
        let contents = fs::read(filename).expect("Something went wrong reading the file");
        let contents: Vec<u16> = contents
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[1], a[0]]))
            .collect();

        //Each instruction is 2 bytes long
        for data in contents.iter() {
            //println!("{:#06X}", data);
            let hex1: u8 = (data >> 12) as u8;
            match hex1 {
                0x0 => match data {
                    0x00E0 => {
                        //00E0 - CLS
                        //Clear the display.
                        self.decoded_instructions.push(format!("CLS"));
                    }
                    0x00EE => {
                        //00EE - RET
                        //Return from a subroutine.
                        self.decoded_instructions.push(format!("RET"));
                    }
                    0x0000..=0x0FFF => {
                        let addr = data & 0x0FFF;
                        self.decoded_instructions
                            .push(format!("SYS addr {:#05X}", addr));
                    }
                    _ => {
                        self.decoded_instructions.push(format!("SYS UNKNOWN"));
                    }
                },
                0x1 => {
                    // 1nnn - JP addr
                    // Jump to location nnn.
                    // The interpreter sets the program counter to nnn.
                    let addr = data & 0x0FFF;
                    let res = format!("JP addr {:#05X}", addr);
                    self.decoded_instructions.push(res);
                }
                0x2 => {
                    // 2nnn - CALL addr
                    // Call subroutine at nnn.
                    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                    let addr = data & 0x0FFF;
                    let res = format!("CALL addr {:#05X}", addr);
                    self.decoded_instructions.push(res);
                }
                0x3 => {
                    // 3xkk - SE Vx, byte
                    // Skip next instruction if Vx = kk.
                    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                    let byte = data & 0x00FF;
                    let vx = (data >> 8) & 0x000F;
                    let res = format!("SE V{}, {:#04X}", vx, byte);
                    self.decoded_instructions.push(res);
                }
                0x4 => {
                    // 4xkk - SNE Vx, byte
                    // Skip next instruction if Vx != kk.
                    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                    let byte = data & 0x00FF;
                    let vx = (data >> 8) & 0x000F;
                    let res = format!("SNE V{}, {:#04X}", vx, byte);
                    self.decoded_instructions.push(res);
                }
                0x5 => {
                    // 5xy0 - SE Vx, Vy
                    // Skip next instruction if Vx = Vy.
                    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                    let vx = (data >> 8) & 0x000F;
                    let vy = (data >> 4) & 0x000F;
                    let res = format!("SE V{}, V{}", vx, vy);
                    self.decoded_instructions.push(res);
                }
                0x6 => {
                    // 6xkk - LD Vx, byte
                    // Set Vx = kk.
                    // The interpreter puts the value kk into register Vx.
                    let byte = data & 0x00FF;
                    let vx = (data >> 8) & 0x000F;
                    let res = format!("LD V{}, {:#04X}", vx, byte);
                    self.decoded_instructions.push(res);
                }
                0x7 => {
                    // 7xkk - ADD Vx, byte
                    // Set Vx = Vx + kk.
                    // Adds the value kk to the value of register Vx, then stores the result in Vx.
                    let byte = data & 0x00FF;
                    let vx = (data >> 8) & 0x000F;
                    let res = format!("ADD V{}, {:#04X}", vx, byte);
                    self.decoded_instructions.push(res);
                }
                0x8 => {
                    let hex4: u8 = (data & 0x000F) as u8;
                    match hex4 {
                        0x0 => {
                            // 8xy0 - LD Vx, Vy
                            // Set Vx = Vy.
                            // Stores the value of register Vy in register Vx.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("LD V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x1 => {
                            // 8xy1 - OR Vx, Vy
                            // Set Vx = Vx OR Vy.
                            // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("OR V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x2 => {
                            // 8xy2 - AND Vx, Vy
                            // Set Vx = Vx AND Vy.
                            // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("AND V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x3 => {
                            // 8xy3 - XOR Vx, Vy
                            // Set Vx = Vx XOR Vy.
                            // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("XOR V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x4 => {
                            // 8xy4 - ADD Vx, Vy
                            // Set Vx = Vx + Vy, set VF = carry.
                            // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("ADD V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x5 => {
                            // 8xy5 - SUB Vx, Vy
                            // Set Vx = Vx - Vy, set VF = NOT borrow.
                            // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("SUB V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x6 => {
                            // 8xy6 - SHR Vx {, Vy}
                            // Set Vx = Vx SHR 1.
                            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("SHR V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0x7 => {
                            // 8xy7 - SUBN Vx, Vy
                            // Set Vx = Vy - Vx, set VF = NOT borrow.
                            // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("SUBN V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        0xE => {
                            // 8xyE - SHL Vx {, Vy}
                            // Set Vx = Vx SHL 1.
                            // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                            let vx = (data >> 8) & 0x000F;
                            let vy = (data >> 4) & 0x000F;
                            let res = format!("SHL V{}, V{}", vx, vy);
                            self.decoded_instructions.push(res);
                        }
                        _ => {
                            self.decoded_instructions.push("UNKNOWN 8".to_string());
                        }
                    }
                }
                0x9 => {
                    // 9xy0 - SNE Vx, Vy
                    // Skip next instruction if Vx != Vy.
                    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                    let vx = (data >> 8) & 0x000F;
                    let vy = (data >> 4) & 0x000F;
                    let res = format!("SNE V{}, V{}", vx, vy);
                    self.decoded_instructions.push(res);
                }
                0xA => {
                    // Annn - LD I, addr
                    // Set I = nnn.
                    // The value of register I is set to nnn.
                    let addr = data & 0x0FFF;
                    let res = format!("LD I, addr {:#05X}", addr);
                    self.decoded_instructions.push(res);
                }
                0xB => {
                    // Bnnn - JP V0, addr
                    // Jump to location nnn + V0.
                    // The program counter is set to nnn plus the value of V0.
                    let addr = data & 0x0FFF;
                    let res = format!("JP V0, addr {:#05X}", addr);
                    self.decoded_instructions.push(res);
                }
                0xC => {
                    // Cxkk - RND Vx, byte
                    // Set Vx = random byte AND kk.
                    // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
                    let byte = data & 0x00FF;
                    let vx = (data >> 8) & 0x000F;
                    let res = format!("RND V{}, {:#04X}", vx, byte);
                    self.decoded_instructions.push(res);
                }
                0xD => {
                    // Dxyn - DRW Vx, Vy, nibble
                    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                    // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                    let nibble = data & 0x000F;
                    let vx = (data >> 8) & 0x000F;
                    let vy = (data >> 4) & 0x000F;
                    let res = format!("DRW V{}, V{}, {:#03X}", vx, vy, nibble);
                    self.decoded_instructions.push(res);
                }
                0xE => {
                    let hex34: u8 = *data as u8;
                    match hex34 {
                        0x9E => {
                            // Ex9E - SKP Vx
                            // Skip next instruction if key with the value of Vx is pressed.
                            // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("SKP V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0xA1 => {
                            // ExA1 - SKNP Vx
                            // Skip next instruction if key with the value of Vx is not pressed.
                            // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("SKNP V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        _ => {
                            self.decoded_instructions.push("UNKNOWN E".to_string());
                        }
                    }
                }
                0xF => {
                    let hex34: u8 = *data as u8;
                    match hex34 {
                        0x07 => {
                            // Fx07 - LD Vx, DT
                            // Set Vx = delay timer value.
                            // The value of DT is placed into Vx.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD V{}, DT", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x0A => {
                            // Fx0A - LD Vx, K
                            // Wait for a key press, store the value of the key in Vx.
                            // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD V{}, K", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x15 => {
                            // Fx15 - LD DT, Vx
                            // Set delay timer = Vx.
                            // DT is set equal to the value of Vx.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD DT, V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x18 => {
                            // Fx18 - LD ST, Vx
                            // Set sound timer = Vx.
                            // ST is set equal to the value of Vx.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD ST, V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x1E => {
                            // Fx1E - ADD I, Vx
                            // Set I = I + Vx.
                            // The values of I and Vx are added, and the results are stored in I.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("ADD I, V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x29 => {
                            // Fx29 - LD F, Vx
                            // Set I = location of sprite for digit Vx.
                            // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD F, V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x33 => {
                            // Fx33 - LD B, Vx
                            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                            // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD B, V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x55 => {
                            // Fx55 - LD [I], Vx
                            // Store registers V0 through Vx in memory starting at location I.
                            // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD [I], V{}", vx);
                            self.decoded_instructions.push(res);
                        }
                        0x65 => {
                            // Fx65 - LD Vx, [I]
                            // Read registers V0 through Vx from memory starting at location I.
                            // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                            let vx = (data >> 8) & 0x000F;
                            let res = format!("LD V{}, [I]", vx);
                            self.decoded_instructions.push(res);
                        }
                        _ => {
                            self.decoded_instructions.push("UNKNOWN F".to_string());
                        }
                    }
                }
                _ => {
                    self.decoded_instructions.push("UNKNOWN".to_string());
                }
            }
        }
    }

    fn print_loaded_inst(self) {
        // 0x200 is start or program / data block
        let mut c = 0x200;
        for i in self.decoded_instructions {
            println!("{:#05X}: {}", c, i);
            c = c + 2;
        }
    }
}
