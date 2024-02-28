use termion::terminal_size;
use buffer::{Buffer, TerminalSize};

pub mod buffer;

fn get_terminal_size() -> TerminalSize {
    let (x, y) = terminal_size().expect("Failed to get terminal size");
    TerminalSize {
        x,
        y,
    }
}

fn main() {
    let buffer = Buffer::new(get_terminal_size());
    println!("Width, Height: {}", buffer.size);
}
