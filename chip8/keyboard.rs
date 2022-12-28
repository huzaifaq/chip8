#[derive(Debug)]
pub struct Chip8Keyboard {
    key_map: u16,
}

impl Chip8Keyboard {
    pub fn new() -> Chip8Keyboard {
        Chip8Keyboard { key_map: 0 }
    }

    pub fn reset_keys(&mut self) {
        self.key_map = 0;
    }

    pub fn set_key(&mut self, key: u8) {
        let byte = key & 0x0F;
        let bit_mask = 1 << byte;
        self.key_map = self.key_map | bit_mask;
    }

    pub fn get_key(&self, key: u8) -> bool {
        let byte = key & 0x0F;
        let bit_mask = 1 << byte;
        (self.key_map & bit_mask) > 0
    }

    pub fn get_key_map(&self) -> u16 {
        self.key_map
    }
}
