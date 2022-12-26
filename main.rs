mod chip8;
use crate::chip8::thread_messages::Chip8ControlMessage;
use chip8::Chip8;
use std::env;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (display_tx, display_rx) = channel(1);
    let (timer_tx, timer_rx) = channel(1);
    let (cpu_tx, cpu_rx) = channel(1);

    let sys = Chip8::new("roms/BRIX");
    sys.start_display_thread(display_rx);
    sys.start_timers_thread(timer_rx);
    sys.start_cpu_thread(cpu_rx);

    display_tx.send(Chip8ControlMessage::Start).await.unwrap();
    timer_tx.send(Chip8ControlMessage::Start).await.unwrap();
    cpu_tx.send(Chip8ControlMessage::Start).await.unwrap();

    let mut line: String;
    loop {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "q" => {
                break;
            }
            "s" => {
                timer_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                display_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                cpu_tx.send(Chip8ControlMessage::Stop).await.unwrap();
            }
            "t" => {
                timer_tx.send(Chip8ControlMessage::Stop).await.unwrap();
            }
            "d" => {
                display_tx.send(Chip8ControlMessage::Stop).await.unwrap();
            }
            "c" => {
                cpu_tx.send(Chip8ControlMessage::Stop).await.unwrap();
            }
            "n" => {
                display_tx.send(Chip8ControlMessage::Step).await.unwrap();
                cpu_tx.send(Chip8ControlMessage::Step).await.unwrap();
            }
            "" => {}
            _ => {}
        }
    }
    return Ok(());
}
