#[derive(Debug)]
pub struct Chip8Keyboard {
    key_map: u16,
}

impl Chip8Keyboard {
    pub fn new() -> Chip8Keyboard {
        Chip8Keyboard { key_map: 0 }
    }
}
