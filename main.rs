mod chip8;
use chip8::Chip8;
use tokio::time::{self, Instant};

use std::{env, time::Duration};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut sys = Chip8::new("BC_test.ch8");
    sys.start_display_thread();
    sys.start_timers_thread();

    let _sys_handle = tokio::spawn(async move {
        let sleep = time::sleep(Duration::from_millis(0));
        tokio::pin!(sleep);

        loop {
            tokio::select! {
            () = &mut sleep => {
            sleep.as_mut().reset(Instant::now() + Duration::from_millis(2));
            sys.run_next(false);
            },
            }
        }
    });

    let mut line: String;
    loop {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "q" => {
                break;
            }
            "" => {}
            _ => {}
        }
    }
    return Ok(());
}
