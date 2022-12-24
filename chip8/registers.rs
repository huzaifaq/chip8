use std::fmt::Write;

#[derive(Debug)]
pub struct Chip8Registers {
    // 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    // The F register should not be used by any program, as it is used as a flag by some instructions.
    //Vx, Vy ...
    pub genral: [u8; 16],

    // This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    // I
    pub memory_address: u16,

    // Special purpose 8-bit registers, for the delay and sound timers.
    // When these registers are non-zero, they are automatically decremented at a rate of 60Hz.
    pub special: [u8; 2],

    // Used to store the currently executing address
    pub program_counter: u16,

    // Used to point to the topmost level of the stack.
    pub stack_pointer: u8,

    // The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine.
    pub stack: [u16; 16],
}

impl std::fmt::Display for Chip8Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let tmp = self.raw_array[0][0].to_string();
        let result = self.genral.iter().fold(String::new(), |mut acc_x, x| {
            write!(&mut acc_x, "{}\n", x).unwrap();
            acc_x
        });

        f.write_fmt(format_args!("{}", result))
    }
}

impl Chip8Registers {
    pub fn new() -> Chip8Registers {
        Chip8Registers {
            genral: [0; 16],
            memory_address: 0,
            special: [0; 2],
            program_counter: 0x0200,
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
