pub mod display;
mod keyboard;
mod memory;
mod registers;
pub mod thread_messages;
mod timers;

use display::Chip8Display;
use memory::Chip8Memory;
use registers::Chip8Registers;
use thread_messages::Chip8ControlMessage;
use timers::Chip8Timers;
use tokio::sync::mpsc::Receiver;
use tokio::time::{self};

use std::{fs, sync::Arc, sync::RwLock, time::Duration};

type SharedDisplay = Arc<RwLock<Chip8Display>>;
type SharedMemory = Arc<RwLock<Chip8Memory>>;
type SharedRegisters = Arc<RwLock<Chip8Registers>>;
type SharedTimers = Arc<RwLock<Chip8Timers>>;
pub struct Chip8 {
    pub display: SharedDisplay,
    pub memory: SharedMemory,
    pub registers: SharedRegisters,
    pub timers: SharedTimers,
}

impl Chip8 {
    const PROGRAM_START_ADDRESS: usize = 0x200;

    pub fn new(filename: &str) -> Chip8 {
        let mut sys = Chip8 {
            display: Arc::new(RwLock::new(Chip8Display::new())),
            memory: Arc::new(RwLock::new(Chip8Memory::new())),
            registers: Arc::new(RwLock::new(Chip8Registers::new())),
            timers: Arc::new(RwLock::new(Chip8Timers::new())),
        };
        sys.load_file(filename);
        return sys;
    }

    fn load_file(&mut self, filename: &str) {
        let contents = fs::read(filename).expect("Something went wrong reading the file");
        let mut memory = self.memory.write().unwrap();
        assert!(
            contents.len() < (memory.raw_array.len() - Chip8::PROGRAM_START_ADDRESS),
            "Cannot load selected file as it is greater than program memory size"
        );

        let current_address = Chip8::PROGRAM_START_ADDRESS;
        for (index, data) in contents.iter().enumerate() {
            memory.raw_array[current_address + index] = data.to_owned();
        }
    }

    //Execute current instruction
    pub fn run_next(
        s_memory: &SharedMemory,
        s_registers: &SharedRegisters,
        s_display: &SharedDisplay,
        s_timers: &SharedTimers,
        is_print_inst: bool,
    ) {
        let mut memory = s_memory.write().unwrap();
        let mut registers = s_registers.write().unwrap();

        //Each instruction is 2 bytes long
        let instruction = u16::from_ne_bytes([
            memory.raw_array[(registers.program_counter + 1) as usize],
            memory.raw_array[registers.program_counter as usize],
        ]);
        let mut res: String = "Unimplemented".to_owned();
        let mut is_inc_program_counter = true;
        let hex1: u8 = (instruction >> 12) as u8;
        match hex1 {
            0x0 => match instruction {
                0x00E0 => {
                    //00E0 - CLS
                    //Clear the display.
                    s_display.write().unwrap().clear();
                    res = format!("CLS");
                }
                0x00EE => {
                    // 00EE - RET
                    // Return from a subroutine.
                    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                    registers.stack_pointer = registers.stack_pointer - 1;
                    registers.program_counter = registers.stack[registers.stack_pointer as usize];
                    res = format!("RET");
                }
                0x0000..=0x0FFF => {
                    // 0nnn - SYS addr
                    // Jump to a machine code routine at nnn.

                    // This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
                    let addr = instruction & 0x0FFF;
                    res = format!("SYS {}", addr);
                }
                _ => {}
            },
            0x1 => {
                // 1nnn - JP addr
                // Jump to location nnn.
                // The interpreter sets the program counter to nnn.
                let addr = instruction & 0x0FFF;
                registers.program_counter = addr;
                is_inc_program_counter = false;
                res = format!("JP addr {:#05X}", addr);
            }
            0x2 => {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                let addr = instruction & 0x0FFF;
                let sp = registers.stack_pointer as usize;
                registers.stack[sp] = registers.program_counter;
                registers.stack_pointer = registers.stack_pointer + 1;
                registers.program_counter = addr;
                is_inc_program_counter = false;
                res = format!("CALL addr {:#05X}", addr);
            }
            0x3 => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                let byte = instruction & 0x00FF;
                let vx = (instruction >> 8) & 0x000F;
                if registers.genral[vx as usize] == byte as u8 {
                    registers.program_counter = registers.program_counter + 2;
                }
                res = format!("SE V{}, {:#04X}", vx, byte);
            }
            0x4 => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                let byte = instruction & 0x00FF;
                let vx = (instruction >> 8) & 0x000F;
                if registers.genral[vx as usize] != byte as u8 {
                    registers.program_counter = registers.program_counter + 2;
                }
                res = format!("SNE V{}, {:#04X}", vx, byte);
            }
            0x5 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                let vx = (instruction >> 8) & 0x000F;
                let vy = (instruction >> 4) & 0x000F;
                if registers.genral[vx as usize] == registers.genral[vy as usize] {
                    registers.program_counter = registers.program_counter + 2;
                }
                res = format!("SE V{}, V{}", vx, vy);
            }
            0x6 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                // The interpreter puts the value kk into register Vx.
                let byte = instruction & 0x00FF;
                let vx = (instruction >> 8) & 0x000F;
                registers.genral[vx as usize] = byte as u8;
                res = format!("LD V{}, {:#04X}", vx, byte);
            }
            0x7 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                // Adds the value kk to the value of register Vx, then stores the result in Vx.
                let byte = instruction & 0x00FF;
                let vx = (instruction >> 8) & 0x000F;
                registers.genral[vx as usize] =
                    registers.genral[vx as usize].wrapping_add(byte as u8);
                res = format!("ADD V{}, {:#04X}", vx, byte);
            }
            0x8 => {
                let hex4: u8 = (instruction & 0x000F) as u8;
                match hex4 {
                    0x0 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy.
                        // Stores the value of register Vy in register Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        registers.genral[vx as usize] = registers.genral[vy as usize];
                        res = format!("LD V{}, V{}", vx, vy);
                    }
                    0x1 => {
                        // 8xy1 - OR Vx, Vy
                        // Set Vx = Vx OR Vy.
                        // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        registers.genral[vx as usize] =
                            registers.genral[vx as usize] | registers.genral[vy as usize];
                        res = format!("OR V{}, V{}", vx, vy);
                    }
                    0x2 => {
                        // 8xy2 - AND Vx, Vy
                        // Set Vx = Vx AND Vy.
                        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        registers.genral[vx as usize] =
                            registers.genral[vx as usize] & registers.genral[vy as usize];
                        res = format!("AND V{}, V{}", vx, vy);
                    }
                    0x3 => {
                        // 8xy3 - XOR Vx, Vy
                        // Set Vx = Vx XOR Vy.
                        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        registers.genral[vx as usize] =
                            registers.genral[vx as usize] ^ registers.genral[vy as usize];
                        res = format!("XOR V{}, V{}", vx, vy);
                    }
                    0x4 => {
                        // 8xy4 - ADD Vx, Vy
                        // Set Vx = Vx + Vy, set VF = carry.
                        // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        let ans = registers.genral[vx as usize]
                            .overflowing_add(registers.genral[vy as usize]);
                        registers.genral[vx as usize] = ans.0;
                        registers.genral[15] = ans.1 as u8;
                        res = format!("ADD V{}, V{}", vx, vy);
                    }
                    0x5 => {
                        // 8xy5 - SUB Vx, Vy
                        // Set Vx = Vx - Vy, set VF = NOT borrow.
                        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        let ans = registers.genral[vx as usize]
                            .overflowing_sub(registers.genral[vy as usize]);
                        registers.genral[15] = (!ans.1) as u8;
                        // Only if no borrow save the answer to Vx else ignore.
                        if !ans.1 {
                            registers.genral[vx as usize] = ans.0;
                        }
                        res = format!("SUB V{}, V{}", vx, vy);
                    }
                    0x6 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Set Vx = Vx SHR 1.
                        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        registers.genral[15] = registers.genral[vx as usize] & 1;
                        registers.genral[vx as usize] = registers.genral[vx as usize] >> 1;
                        res = format!("SHR V{}, V{}", vx, vy);
                    }
                    0x7 => {
                        // 8xy7 - SUBN Vx, Vy
                        // Set Vx = Vy - Vx, set VF = NOT borrow.
                        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        let vy = (instruction >> 4) & 0x000F;
                        let ans = registers.genral[vy as usize]
                            .overflowing_sub(registers.genral[vx as usize]);
                        registers.genral[vx as usize] = ans.0;
                        registers.genral[15] = (!ans.1) as u8;
                        res = format!("SUBN V{}, V{}", vx, vy);
                    }
                    0xE => {
                        // 8xyE - SHL Vx {, Vy}
                        // Set Vx = Vx SHL 1.
                        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                        let vx = (instruction >> 8) & 0x000F;
                        registers.genral[15] = ((registers.genral[vx as usize] & 0x80) > 0) as u8;
                        registers.genral[vx as usize] = registers.genral[vx as usize] << 1;
                        res = format!("SHL V{}", vx);
                    }
                    _ => {}
                }
            }
            0x9 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                let vx = (instruction >> 8) & 0x000F;
                let vy = (instruction >> 4) & 0x000F;
                if registers.genral[vx as usize] != registers.genral[vy as usize] {
                    registers.program_counter = registers.program_counter + 2;
                }
                res = format!("SNE V{}, V{}", vx, vy);
            }
            0xA => {
                // Annn - LD I, addr
                // Set I = nnn.
                // The value of register I is set to nnn.
                let addr = instruction & 0x0FFF;
                registers.memory_address = addr;
                res = format!("LD I, addr {:#05X}", addr);
            }
            0xB => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                // The program counter is set to nnn plus the value of V0.
                let addr = instruction & 0x0FFF;
                registers.program_counter = (registers.genral[0] as u16) + addr;
                is_inc_program_counter = false;
                res = format!("JP V0, addr {:#05X}", addr);
            }
            0xC => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
                let byte = (instruction & 0x00FF) as u8;
                let vx = (instruction >> 8) & 0x000F;
                let r: u8 = rand::random();
                registers.genral[vx as usize] = r & byte;
                res = format!("RND V{}, {:#04X}", vx, byte);
            }
            0xD => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // The interpreter reads n bytes from memory, starting at the address stored in I.
                // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
                // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
                // If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                let number_bytes = instruction & 0x000F;
                let vx = (instruction >> 8) & 0x000F;
                let vy = (instruction >> 4) & 0x000F;
                let mut display = s_display.write().unwrap();

                //Simple implementation can speedup.
                //Assuming sprite resolution is 8xn.
                //Get sprite bytes slice
                let sprite = &memory.raw_array[registers.memory_address as usize
                    ..(registers.memory_address + number_bytes) as usize];

                for (index, s_data) in sprite.iter().enumerate() {
                    for bit_index in 0u8..8 {
                        let bit_mask = 128 >> bit_index;
                        let bit = (s_data & bit_mask) > 0;
                        if bit {
                            let current = display.get_pixel(
                                (registers.genral[vx as usize] + bit_index) as usize,
                                registers.genral[vy as usize] as usize + index,
                            );
                            if (current == bit) && (bit == true) {
                                display.unset_pixel(
                                    (registers.genral[vx as usize] + bit_index) as usize,
                                    registers.genral[vy as usize] as usize + index,
                                );
                                registers.genral[15] = 1; //Set VF = 1 for collision
                            } else {
                                display.set_pixel(
                                    (registers.genral[vx as usize] + bit_index) as usize,
                                    registers.genral[vy as usize] as usize + index,
                                );
                            }
                        }
                        //display.set_pixel((vx as usize) + index, 1);
                    }
                }
                res = format!("DRW V{}, V{}, {:#03X}", vx, vy, number_bytes);
            }
            0xE => {
                let hex34: u8 = instruction as u8;
                match hex34 {
                    0x9E => {
                        // Ex9E - SKP Vx
                        // Skip next instruction if key with the value of Vx is pressed.
                        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                        let vx = (instruction >> 8) & 0x000F;
                        res = format!("TODO: SKP V{}", vx);
                    }
                    0xA1 => {
                        // ExA1 - SKNP Vx
                        // Skip next instruction if key with the value of Vx is not pressed.
                        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                        let vx = (instruction >> 8) & 0x000F;
                        res = format!("TODO: SKNP V{}", vx);
                    }
                    _ => {}
                }
            }
            0xF => {
                let hex34: u8 = instruction as u8;
                match hex34 {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        // Set Vx = delay timer value.
                        // The value of DT is placed into Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        registers.genral[vx as usize] = s_timers.read().unwrap().delay_timer;
                        res = format!("LD V{}, DT", vx);
                    }
                    0x0A => {
                        // Fx0A - LD Vx, K
                        // Wait for a key press, store the value of the key in Vx.
                        // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        res = format!("TODO: LD V{}, K", vx);
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        // Set delay timer = Vx.
                        // DT is set equal to the value of Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        s_timers.write().unwrap().delay_timer = registers.genral[vx as usize];
                        res = format!("LD DT, V{}", vx);
                    }
                    0x18 => {
                        // Fx18 - LD ST, Vx
                        // Set sound timer = Vx.
                        // ST is set equal to the value of Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        s_timers.write().unwrap().sound_timer = registers.genral[vx as usize];
                        res = format!("LD ST, V{}", vx);
                    }
                    0x1E => {
                        // Fx1E - ADD I, Vx
                        // Set I = I + Vx.
                        // The values of I and Vx are added, and the results are stored in I.
                        let vx = (instruction >> 8) & 0x000F;
                        registers.memory_address = registers
                            .memory_address
                            .wrapping_add(registers.genral[vx as usize] as u16);
                        res = format!("ADD I, V{}", vx);
                    }
                    0x29 => {
                        // Fx29 - LD F, Vx
                        // Set I = location of sprite for digit Vx.
                        // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                        let vx = ((instruction >> 8) & 0x000F) as u8;
                        registers.memory_address =
                            ((registers.genral[vx as usize] & 0x0F) * 5) as u16;
                        res = format!("LD F, V{}", vx);
                    }
                    0x33 => {
                        // Fx33 - LD B, Vx
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                        let vx = (instruction >> 8) & 0x000F;
                        memory.raw_array[registers.memory_address as usize] =
                            (registers.genral[vx as usize] / 100) % 10;
                        memory.raw_array[(registers.memory_address + 1) as usize] =
                            (registers.genral[vx as usize] / 10) % 10;
                        memory.raw_array[(registers.memory_address + 2) as usize] =
                            (registers.genral[vx as usize] / 1) % 10;
                        res = format!(
                            "LD B, V{} (I: {}, I+1: {}, I+2:{})",
                            vx,
                            memory.raw_array[registers.memory_address as usize],
                            memory.raw_array[(registers.memory_address + 1) as usize],
                            memory.raw_array[(registers.memory_address + 2) as usize]
                        );
                    }
                    0x55 => {
                        // Fx55 - LD [I], Vx
                        // Store registers V0 through Vx in memory starting at location I.
                        // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                        let vx = (instruction >> 8) & 0x000F;
                        let current_address = registers.memory_address;
                        for data in 0..=vx {
                            memory.raw_array[(current_address + data) as usize] =
                                registers.genral[data as usize];
                        }
                        res = format!("LD [I], V{}", vx);
                    }
                    0x65 => {
                        // Fx65 - LD Vx, [I]
                        // Read registers V0 through Vx from memory starting at location I.
                        // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                        let vx = (instruction >> 8) & 0x000F;
                        let current_address = registers.memory_address;
                        for data in 0..=vx {
                            registers.genral[data as usize] =
                                memory.raw_array[(current_address + data) as usize];
                        }
                        res = format!("LD V{}, [I]", vx);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if is_print_inst {
            println!("{:#05X}: {}", registers.program_counter, res);
        }
        //Increment in program counter after instruction is processed
        if is_inc_program_counter {
            registers.program_counter = registers.program_counter + 2;
        }
    }

    //Start a thread to print display buffer to stdout every second (for debug perpose)
    pub fn _start_display_thread(
        &self,
        mut rx: Receiver<Chip8ControlMessage>,
    ) -> tokio::task::JoinHandle<()> {
        let m_display = self.display.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
            tokio::pin!(interval);
            let mut is_running = false;
            loop {
                tokio::select! {
                _ = interval.tick() => {
                if is_running {
                print!("{}[2J", 27 as char);
                println!("{}", m_display.read().unwrap());
                }
                },
                Some(msg) = rx.recv() => {
                match msg {
                Chip8ControlMessage::Start => { is_running = true; },
                Chip8ControlMessage::Stop => { is_running = false; },
                Chip8ControlMessage::Step => {
                print!("{}[2J", 27 as char);
                println!("{}", m_display.read().unwrap());
                },
                }
                }
                }
            }
        })
    }

    pub fn start_timers_thread(
        &self,
        mut rx: Receiver<Chip8ControlMessage>,
    ) -> tokio::task::JoinHandle<()> {
        let m_timers = self.timers.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(17));
            interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
            tokio::pin!(interval);
            let mut is_running = false;
            loop {
                tokio::select! {
                _ = interval.tick() => {
                if is_running {
                let mut timers = m_timers.write().unwrap();
                if timers.delay_timer > 0 {
                timers.delay_timer = timers.delay_timer - 1;
                //println!("{}", timers.delay_timer);
                }
                if timers.sound_timer > 0 {
                timers.sound_timer = timers.sound_timer - 1;
                //println!("{}", timers.sound_timer);
                }
                }
                },
                Some(msg) = rx.recv() => {
                match msg {
                Chip8ControlMessage::Start => { is_running = true; },
                Chip8ControlMessage::Stop => { is_running = false; },
                Chip8ControlMessage::Step => {
                let mut timers = m_timers.write().unwrap();
                if timers.delay_timer > 0 {
                timers.delay_timer = timers.delay_timer - 1;
                //println!("{}", timers.delay_timer);
                }
                if timers.sound_timer > 0 {
                timers.sound_timer = timers.sound_timer - 1;
                //println!("{}", timers.sound_timer);
                }
                },
                }
                }
                }
            }
        })
    }

    pub fn start_cpu_thread(
        &self,
        mut rx: Receiver<Chip8ControlMessage>,
    ) -> tokio::task::JoinHandle<()> {
        let m_display = self.display.clone();
        let m_timers = self.timers.clone();
        let m_memory = self.memory.clone();
        let m_registers = self.registers.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(2));
            interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
            tokio::pin!(interval);
            let mut is_running = false;
            loop {
                tokio::select! {
                _ = interval.tick() => {
                if is_running {
                Chip8::run_next(&m_memory,&m_registers,&m_display,&m_timers,false);
                }
                },
                Some(msg) = rx.recv() => {
                match msg {
                Chip8ControlMessage::Start => { is_running = true; },
                Chip8ControlMessage::Stop => { is_running = false; },
                Chip8ControlMessage::Step => {
                Chip8::run_next(&m_memory,&m_registers,&m_display,&m_timers,true);
                },
                }
                }
                }
            }
        })
    }
}
