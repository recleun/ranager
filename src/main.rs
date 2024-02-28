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

    let mut changed = true;
    let mut buffer = Buffer::new(get_terminal_size());

    buffer.set_line(0, &String::from("1 - Start"));
    buffer.set_line(1, &String::from("2 - Help"));
    buffer.set_line(2, &String::from("3 - Options"));
    buffer.set_line(3, &String::from("0 - Exit"));

    loop {
        if changed {
            changed = false;
            buffer.update_display();
        }
    }
}
