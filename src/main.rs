use std::fmt;
use termion::terminal_size;

struct TerminalSize {
    x: u16,
    y: u16,
}

impl fmt::Display for TerminalSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

fn get_terminal_size() -> TerminalSize {
    let (x, y) = terminal_size().expect("Failed to get terminal size");
    TerminalSize {
        x,
        y,
    }
}

fn main() {
    println!("Width, Height: {}", get_terminal_size());
}
