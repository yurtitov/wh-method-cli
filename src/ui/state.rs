use crate::ui::cmd::Cmd;

#[derive(Debug)]
pub struct State {
    last_command: Cmd,
}

impl State {
    pub fn new() -> State {
        State {
            last_command: Cmd::CurrentTime,
        }
    }

    pub fn last_command(&self) -> Cmd {
        self.last_command.clone()
    }

    pub fn update(&mut self, current_command: Cmd) {
        self.last_command = current_command;
    }
}