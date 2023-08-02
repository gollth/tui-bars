use std::{
    error::Error,
    f32::consts::PI,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame, Terminal,
};
use tui_bars::ValueBar;

#[derive(Default)]
struct App(f32);

impl App {
    fn on_tick(&mut self, t: f32) {
        self.0 = (2. * PI * t / 5.).sin();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(50);
    let app = App::default();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut t = 0.;
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            t += tick_rate.as_secs_f32();
            app.on_tick(t);
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let bar = ValueBar::default()
        .value(app.0)
        .range(1.)
        .label(format!("{:.2}", app.0))
        .block(Block::default().title("SinWave").borders(Borders::ALL))
        .style(Style::default().fg(Color::Red));
    f.render_widget(bar, f.size());
}
