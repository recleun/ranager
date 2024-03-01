use termion::{cursor, raw::IntoRawMode, screen::IntoAlternateScreen, terminal_size};
use buffer::{Buffer, TerminalSize};
use std::io::{stdin, stdout, Read, Write};

pub mod buffer;

fn get_terminal_size() -> TerminalSize {
    let (x, y) = terminal_size().expect("Failed to get terminal size");
    TerminalSize {
        x,
        y,
    }
}

fn main() {

    let stdin = stdin().lock();
    let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
    let mut buffer = Buffer::new(get_terminal_size());

    write!(screen, "{}", cursor::Hide).unwrap();

    screen.flush().unwrap();

    buffer.set_line(0, &String::from("1 - Start\n"));
    buffer.set_line(1, &String::from("2 - Help\n"));
    buffer.set_line(2, &String::from("3 - Options\n"));
    buffer.set_line(3, &String::from("0 - Exit\n"));

    buffer.update_display(&mut screen);

    let mut bytes = stdin.bytes();

    loop {
        match bytes.next().unwrap().unwrap() {
            b'0' => break,
            b'1' => {
                buffer.clear();
                buffer.set_line(0, &"Welcome to the start menu.".to_string());
                buffer.update_display(&mut screen);
            },
            b'2' => {
                buffer.clear();
                buffer.set_line(0, &"Welcome to the help menu.".to_string());
                buffer.update_display(&mut screen);
            },
            b'3' => {
                buffer.clear();
                buffer.set_line(0, &"Welcome to the options menu.".to_string());
                buffer.update_display(&mut screen);
            },
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
