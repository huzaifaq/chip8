use crate::chip8::Chip8;

pub struct App {
    pub sys: Chip8,
}

impl App {
    pub fn new(filename: &str) -> App {
        let c8 = Chip8::new(filename);
        App { sys: c8 }
    }
}
