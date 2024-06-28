#[derive(PartialEq, Debug)]
pub enum Cmd {
    Quit,
    CurrentTime,
    Sdf, // todo Need to be removed
    Empty,
}

impl Clone for Cmd {
    fn clone(&self) -> Self {
        return match self {
            Cmd::Quit => Cmd::Quit,
            Cmd::CurrentTime => Cmd::CurrentTime,
            Cmd::Sdf => Cmd::Sdf,
            Cmd::Empty => Cmd::Empty,
        };
    }
}