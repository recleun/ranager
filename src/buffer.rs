use std::fmt;
use std::mem;
use termion::{clear, cursor};

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
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(size: TerminalSize) -> Self {
        println!("{}", cursor::Hide);
        let mut lines: Vec<String> = Vec::new();
        for _ in 0..size.y {
            lines.push(String::new());
        }
        Buffer {
            size,
            lines,
        }
    }

    pub fn get_line(&self, index: u16) -> String {
        if !self.check_height_range(index) {
            panic!("Trying to reach for line out of reach");
        }
        self.lines[index as usize].clone()
    }

    pub fn set_line(&mut self, index: u16, line: &String) {
        if !self.check_height_range(index) && !self.check_width_range(line.len() as u16) {
            panic!("Trying to reach for line out of reach");
        }
        self.replace(index as usize, line);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        for _ in 0..self.size.y {
            self.lines.push(String::new());
        }
    }

    pub fn update_display(&self) {
        self.clear_display();
        for line in &self.lines {
            println!("{}", line);
        }
    }

    fn clear_display(&self) {
        println!("{}{}", clear::All, cursor::Goto(1, 1)); // why the hell is it one-based
    }

    fn replace(&mut self, index: usize, line: &String) -> String {
        mem::replace(&mut self.lines[index], line.to_string())
    }

    fn check_height_range(&self, index: u16) -> bool {
        index < self.size.y
    }

    fn check_width_range(&self, index: u16) -> bool {
        index < self.size.x
    }
}
