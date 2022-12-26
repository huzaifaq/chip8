#[derive(Debug)]
pub struct Chip8Keyboard {
    key_map: u16,
}

impl std::fmt::Display for Chip8Keyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Chip8Keyboard {
    pub fn new() -> Chip8Keyboard {
        Chip8Keyboard { key_map: 0 }
    }

    pub fn reset_keys(&mut self) {
        self.key_map = 0;
    }

    pub fn set_key() {}
}
