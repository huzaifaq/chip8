mod app;
mod chip8;
use crate::chip8::thread_messages::Chip8ControlMessage;
use app::App;
use chip8::display::Chip8Display;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::channel;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, List, ListItem,
    },
    Frame, Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(16);
    let key_timeout = Duration::from_millis(250);
    let app = App::new("roms/BRIX");
    let res = run_app(&mut terminal, app, tick_rate, key_timeout).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: App,
    tick_rate: Duration, // this defines when the "display" should be redrawn
    key_timeout: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut last_key_press = Instant::now();

    let (timer_tx, timer_rx) = channel(1);
    let (cpu_tx, cpu_rx) = channel(1);

    app.sys.start_timers_thread(timer_rx);
    app.sys.start_cpu_thread(cpu_rx);

    timer_tx.send(Chip8ControlMessage::Start).await.unwrap();
    cpu_tx.send(Chip8ControlMessage::Start).await.unwrap();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('s') => {
                        timer_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                        cpu_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                    }
                    KeyCode::Char('t') => {
                        timer_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                    }
                    KeyCode::Char('c') => {
                        cpu_tx.send(Chip8ControlMessage::Start).await.unwrap();
                    }
                    KeyCode::Char('d') => {
                        cpu_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                    }
                    KeyCode::Char('n') => {
                        cpu_tx.send(Chip8ControlMessage::Step).await.unwrap();
                    }
                    KeyCode::Char('r') => {
                        app.sys.load_file_reset("roms/PONG");
                    }
                    KeyCode::Down => {
                        app.sys.keyboard.write().unwrap().set_key(0);
                    }
                    KeyCode::Up => {
                        app.sys.keyboard.write().unwrap().set_key(1);
                    }
                    KeyCode::Right => {
                        app.sys.keyboard.write().unwrap().set_key(2);
                    }
                    KeyCode::Left => {
                        app.sys.keyboard.write().unwrap().set_key(3);
                    }
                    _ => {}
                }
                last_key_press = Instant::now();
            }
        }

        if last_key_press.elapsed() >= key_timeout {
            app.sys.keyboard.write().unwrap().reset_keys();
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let list_coords = app.sys.display.read().unwrap().get_set_pixel_coords();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(f.size());
    let canvas = Canvas::default()
        .marker(symbols::Marker::Braille)
        .block(Block::default().borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &list_coords,
                color: Color::White,
            });
        })
        .x_bounds([0.0, Chip8Display::WIDTH as f64])
        .y_bounds([0.0, Chip8Display::HEIGHT as f64]);
    f.render_widget(canvas, chunks[0]);
    let instructions = app.sys.instructions.read().unwrap();
    let mut items = instructions
        .iter()
        .map(|i| ListItem::new(i.to_owned()))
        .collect::<Vec<ListItem>>();

    items.push(ListItem::new(format!(" ")));
    let registers = app.sys.registers.read().unwrap();
    items.push(ListItem::new(format!("General: {:02X?}", registers.genral)));
    items.push(ListItem::new(format!(
        "Stack: {:02X?} : {:02X?}",
        registers.stack, registers.stack_pointer
    )));
    items.push(ListItem::new(format!(
        "I: {:#05X}",
        registers.memory_address
    )));
    items.push(ListItem::new(format!(
        "PC: {:#05X}",
        registers.program_counter
    )));
    items.push(ListItem::new(format!("Special:{:02X?}", registers.special)));
    let keyboard = app.sys.keyboard.read().unwrap();
    items.push(ListItem::new(format!(
        "Keyboard: {:#016b}",
        keyboard.get_key_map()
    )));

    let list = List::new(items)
        .block(Block::default().title("Debug Info").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(list, chunks[1]);
}
