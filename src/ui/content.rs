use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::ui::cmd::Cmd;
use crate::ui::state::State;

pub fn create_by(state: &State) -> String {
    let cmd = state.last_command();
    match cmd {
        Cmd::CurrentTime => current_time("%T"),
        Cmd::Sdf => String::from("Sdf"),
        Cmd::Quit => String::new(),
        Cmd::Empty => String::new(),
    }
}

fn current_time(fmt: &str) -> String {
    let time = SystemTime::now();
    let date_time: DateTime<Local> = time.into();
    date_time.format(fmt).to_string()
}