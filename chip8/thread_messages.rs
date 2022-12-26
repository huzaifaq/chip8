#[derive(Debug)]
pub enum Chip8ControlMessage {
    Stop,
    Start,
    Step,
    Exit,
    Next,
}
