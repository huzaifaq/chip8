#[derive(Debug)]
pub struct Chip8Memory {
    pub raw_array: [u8; 4096],
}

impl Chip8Memory {
    pub fn new() -> Chip8Memory {
        let mut mem = Chip8Memory {
            raw_array: [0; 4096],
        };
        // Add sprites to memory
        // "0"
        mem.raw_array[0x0] = 0xF0;
        mem.raw_array[0x1] = 0x90;
        mem.raw_array[0x2] = 0x90;
        mem.raw_array[0x3] = 0x90;
        mem.raw_array[0x4] = 0xF0;

        // "1"
        mem.raw_array[0x5] = 0x20;
        mem.raw_array[0x6] = 0x60;
        mem.raw_array[0x7] = 0x20;
        mem.raw_array[0x8] = 0x20;
        mem.raw_array[0x9] = 0x70;

        // "2"
        mem.raw_array[0xA] = 0xF0;
        mem.raw_array[0xB] = 0x10;
        mem.raw_array[0xC] = 0xF0;
        mem.raw_array[0xD] = 0x80;
        mem.raw_array[0xE] = 0xF0;

        // "3"
        mem.raw_array[0xF] = 0xF0;
        mem.raw_array[0x10] = 0x10;
        mem.raw_array[0x11] = 0xF0;
        mem.raw_array[0x12] = 0x10;
        mem.raw_array[0x13] = 0xF0;

        // "4"
        mem.raw_array[0x14] = 0x90;
        mem.raw_array[0x15] = 0x90;
        mem.raw_array[0x16] = 0xF0;
        mem.raw_array[0x17] = 0x10;
        mem.raw_array[0x18] = 0x10;

        // "5"
        mem.raw_array[0x19] = 0xF0;
        mem.raw_array[0x1A] = 0x80;
        mem.raw_array[0x1B] = 0xF0;
        mem.raw_array[0x1C] = 0x10;
        mem.raw_array[0x1D] = 0xF0;

        // "6"
        mem.raw_array[0x1E] = 0xF0;
        mem.raw_array[0x1F] = 0x80;
        mem.raw_array[0x20] = 0xF0;
        mem.raw_array[0x21] = 0x90;
        mem.raw_array[0x22] = 0xF0;

        // "7"
        mem.raw_array[0x23] = 0xF0;
        mem.raw_array[0x24] = 0x10;
        mem.raw_array[0x25] = 0x20;
        mem.raw_array[0x26] = 0x40;
        mem.raw_array[0x27] = 0x40;

        // "8"
        mem.raw_array[0x28] = 0xF0;
        mem.raw_array[0x29] = 0x90;
        mem.raw_array[0x2A] = 0xF0;
        mem.raw_array[0x2B] = 0x90;
        mem.raw_array[0x2C] = 0xF0;

        // "9"
        mem.raw_array[0x2D] = 0xF0;
        mem.raw_array[0x2E] = 0x90;
        mem.raw_array[0x2F] = 0xF0;
        mem.raw_array[0x30] = 0x10;
        mem.raw_array[0x31] = 0xF0;

        // "A"
        mem.raw_array[0x32] = 0xF0;
        mem.raw_array[0x33] = 0x90;
        mem.raw_array[0x34] = 0xF0;
        mem.raw_array[0x35] = 0x90;
        mem.raw_array[0x36] = 0x90;

        // "B"
        mem.raw_array[0x37] = 0xE0;
        mem.raw_array[0x38] = 0x90;
        mem.raw_array[0x39] = 0xE0;
        mem.raw_array[0x3A] = 0x90;
        mem.raw_array[0x3B] = 0xE0;

        // "C"
        mem.raw_array[0x3C] = 0xF0;
        mem.raw_array[0x3D] = 0x80;
        mem.raw_array[0x3E] = 0x80;
        mem.raw_array[0x3F] = 0x80;
        mem.raw_array[0x40] = 0xF0;

        // "D"
        mem.raw_array[0x41] = 0xE0;
        mem.raw_array[0x42] = 0x90;
        mem.raw_array[0x43] = 0x90;
        mem.raw_array[0x44] = 0x90;
        mem.raw_array[0x45] = 0xE0;

        // "E"
        mem.raw_array[0x46] = 0xF0;
        mem.raw_array[0x47] = 0x80;
        mem.raw_array[0x48] = 0xF0;
        mem.raw_array[0x49] = 0x80;
        mem.raw_array[0x4A] = 0xF0;

        // "F"
        mem.raw_array[0x4B] = 0xF0;
        mem.raw_array[0x4C] = 0x80;
        mem.raw_array[0x4D] = 0xF0;
        mem.raw_array[0x4E] = 0x80;
        mem.raw_array[0x4F] = 0x80;

        mem
    }
}
