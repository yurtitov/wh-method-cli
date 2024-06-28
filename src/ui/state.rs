use crate::ui::cmd::Cmd;

#[derive(Debug)]
pub struct State {
    command: Cmd,
}

impl State {
    pub fn new() -> State {
        State {
            command: Cmd::CurrentTime,
        }
    }

    pub fn last_command(&self) -> Cmd {
        self.command.clone()
    }

    pub fn update(&mut self, command: Cmd) {
        if command != Cmd::Empty {
            self.command = command;
        }
    }
}