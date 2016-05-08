use std::fmt;
use super::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cell {
    required: bool,
    mode: Mode
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(false, Mode::Empty)
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell { required: true, .. } => write!(f, "█"),
            Cell { required: false, .. } => write!(f, "_"),
        }
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        if ch == 'X' {
            Cell::new(true, Mode::Empty)
        } else {
            Cell::default()
        }
    }
}

impl Cell {
    pub fn new(required: bool, mode: Mode) -> Cell {
        Cell {
            required: required,
            mode: mode
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn required(&self) -> bool {
        self.required
    }

}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Marked,
    Crossed,
    Empty
}
