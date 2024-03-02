use std::fmt;
use std::mem;
use termion::{clear, cursor};
use std::io::Write;

#[derive(Copy)]
pub struct TerminalSize {
    pub x: u16,
    pub y: u16,
}

impl Clone for TerminalSize {
    fn clone(&self) -> Self {
        TerminalSize {
            x: self.x,
            y: self.y,
        }
    }
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

pub struct LineOptions {
    pub alignment: LineAlignment,
}

pub enum LineAlignment {
    Left,
    Center,
    Right,
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
        if !self.check_height_range(index) {
            panic!("Trying to reach for line out of reach");
        }
        self.lines[index as usize].clone()
    }

    pub fn set_line(&mut self, index: u16, line: &String, options: Option<LineOptions>) {
        if !self.check_height_range(index) && !self.check_width_range(line.len() as u16) {
            panic!("Trying to reach for line out of reach");
        }
        if let Some(_options) = options {
            match _options.alignment {
                LineAlignment::Left => {
                    self.replace(index as usize, line);
                }
                LineAlignment::Center => {
                    self.replace(index as usize, &self.align_center(line));
                }
                LineAlignment::Right => {
                    self.replace(index as usize, &self.align_right(line));
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        for _ in 0..self.size.y {
            self.lines.push(String::new());
        }
    }

    pub fn update_display<W: Write>(&self, stdout: &mut W) {
        self.clear_display();
        let mut count = 1;
        for line in &self.lines {
            write!(stdout, "{}{}", cursor::Goto(1, count), line).unwrap();
            count += 1;
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

    fn align_center(&self, string: &String) -> String {
        let mut result = String::new();
        let space_count = (self.size.x - string.len() as u16) / 2;
        for _ in 0..space_count {
            result.push(' ');
        }
        result.push_str(string);
        result
    }

    fn align_right(&self, string: &String) -> String {
        let mut result = String::new();
        let space_count = self.size.x - string.len() as u16;
        for _ in 0..space_count {
            result.push(' ');
        }
        result.push_str(string);
        result
    }
    
}
