use std::io;
use std::io::stdout;
use std::time::SystemTime;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{event, ExecutableCommand};
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{Frame, Terminal};
use ratatui::crossterm::event::{Event, KeyCode};
use chrono::{DateTime, Local};
use ratatui::prelude::*;
use tui_big_text::{BigText, PixelSize};
use cmd::Cmd;
use state::State;

mod cmd;
mod state;

pub fn start() -> io::Result<()> {
    let mut state = State::new();
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        if state.last_command() == Cmd::Quit {
            break;
        }

        let content: String = content(state.last_command());
        terminal.draw(|f| render(f, &content))?;

        handle_events(&mut state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn content(cmd: Cmd) -> String {
    match cmd {
        Cmd::Quit => String::new(),
        Cmd::CurrentTime => current_time("%T"),
        Cmd::Sdf => String::from("Sdf"),
        Cmd::Empty => String::new(),
    }
}

fn current_time(fmt: &str) -> String {
    let time = SystemTime::now();
    let date_time: DateTime<Local> = time.into();
    date_time.format(fmt).to_string()
}

fn handle_events(state: &mut State) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let current_command = match key.code {
                    KeyCode::Char('q') => Cmd::Quit,
                    KeyCode::Char('t') => Cmd::CurrentTime,
                    KeyCode::Char('s') => Cmd::Sdf,
                    _ => Cmd::Empty
                };
                if current_command != Cmd::Empty {
                    state.update(current_command);
                }
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, text: &str) {
    let big_text = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Full)
        .lines(vec![Line::from(text)])
        .style(Style::new().light_green())
        .build()
        .expect("Something went wrong!");

    frame.render_widget(big_text, frame.size());
}