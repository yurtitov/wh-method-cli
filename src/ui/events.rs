use std::io;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use crate::ui::cmd::Cmd;

pub fn get_command() -> io::Result<Cmd> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let current_command = match key.code {
                    KeyCode::Char('q') => Cmd::Quit,
                    KeyCode::Char('t') => Cmd::CurrentTime,
                    KeyCode::Char('s') => Cmd::Sdf,
                    _ => Cmd::Empty
                };
                return Ok(current_command);
            }
        }
    }
    Ok(Cmd::Empty)
}