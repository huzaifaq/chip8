#[derive(Debug)]
pub struct Chip8Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Chip8Timers {
    pub fn new() -> Chip8Timers {
        Chip8Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
