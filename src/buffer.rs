use std::fmt;
use std::mem;

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
        if index > self.size.y {
            panic!("Trying to reach for line out of reach")
        };
        self.lines[index as usize].clone()
    }

    pub fn set_line(&mut self, index: u16, line: &String) {
        if index > self.size.y {
            panic!("Trying to reach for line out of reach");
        }
        self.replace(index as usize, line);
    }

    fn replace(&mut self, index: usize, line: &String) -> String {
        mem::replace(&mut self.lines[index], line.to_string())
    }
}
