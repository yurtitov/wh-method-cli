use std::io;
use std::io::stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{Frame, Terminal};
use ratatui::prelude::{Alignment, Line, Style, Stylize};
use tui_big_text::{BigText, PixelSize};
use cmd::Cmd;
use state::State;

mod cmd;
mod state;
mod events;
mod content;

pub fn start() -> io::Result<()> {
    let mut state = State::new();
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        if state.last_command() == Cmd::Quit {
            break;
        }

        let content: String = content::create_by(&state);
        terminal.draw(|f| render(f, &content))?;

        let cmd = events::get_command()?;
        state.update(cmd)
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

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