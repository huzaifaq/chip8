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
    style::Color,
    symbols,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
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
    let tick_rate = Duration::from_millis(250);
    let app = App::new("roms/BC_test.ch8");
    let res = run_app(&mut terminal, app, tick_rate).await;

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
) -> io::Result<()> {
    let mut last_tick = Instant::now();

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
                        return Ok(());
                    }
                    KeyCode::Char('t') => {
                        timer_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                        return Ok(());
                    }
                    KeyCode::Char('c') => {
                        cpu_tx.send(Chip8ControlMessage::Stop).await.unwrap();
                        return Ok(());
                    }
                    KeyCode::Char('n') => {
                        cpu_tx.send(Chip8ControlMessage::Step).await.unwrap();
                        return Ok(());
                    }
                    KeyCode::Down => {}
                    KeyCode::Up => {}
                    KeyCode::Right => {}
                    KeyCode::Left => {}
                    _ => {}
                }
            }
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
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let canvas = Canvas::default()
        .marker(symbols::Marker::Block)
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
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Debug"))
        .paint(|_ctx| {
            //ctx.draw(&app.ball);
        })
        .x_bounds([10.0, 110.0])
        .y_bounds([10.0, 110.0]);
    f.render_widget(canvas, chunks[1]);
}
