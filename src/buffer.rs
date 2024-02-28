use std::fmt;

pub struct TerminalSize {
    pub x: u16,
    pub y: u16,
}

impl fmt::Display for TerminalSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

pub struct Buffer {
    pub size: TerminalSize,
}

impl Buffer {
    pub fn new(size: TerminalSize) -> Self {
        Buffer {
            size
        }
    }
}
