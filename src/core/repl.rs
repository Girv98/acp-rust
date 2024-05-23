use std::io::Write;
use std::io;
use std::fmt;

use colored::ColoredString;
use colored::Colorize;
use termion::clear;
use termion::cursor;

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

use crate::ply::Colour;
use crate::Game;

#[derive(Default)]
struct InputBuffer {
    buffer: Vec<char>,
    cursor: usize,
}

impl InputBuffer {
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    pub fn take(&mut self) -> String {
        let result = self.buffer.iter().collect();
        self.clear();
        result
    }

    pub fn insert_char(&mut self, ch: char) {
        self.buffer.insert(self.cursor, ch);
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.buffer.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.buffer.len() {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn jump_start(&mut self) {
        self.cursor = 0;
    }

    pub fn jump_end(&mut self) {
        self.cursor = self.buffer.len();
    }

    pub fn left_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right_char(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    pub fn render(&self, prompt: &ColoredString, sink: &mut impl Write) -> io::Result<()> {
        let buf: String = self.buffer.iter().collect();
        write!(sink, "\r{}{}{}\r", clear::AfterCursor, prompt, &buf)?;
        write!(sink,"{}", cursor::Right((prompt.len() + self.cursor).try_into().unwrap()))?;
        Ok(())
    }
}

pub enum InputType {
    String(String),
    Termination
}

pub fn get_input(prompt: &ColoredString, ) -> Result<InputType, io::Error>{
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", prompt, cursor::BlinkingBar).unwrap();
    stdout.flush().unwrap();
    
    let stdin = io::stdin();
    let mut inp_buf = InputBuffer::default();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Home => inp_buf.jump_start(),
            Key::End => inp_buf.jump_end(),
            Key::Left => inp_buf.left_char(),
            Key::Right => inp_buf.right_char(),
            Key::Backspace => inp_buf.backspace(),
            Key::Delete => inp_buf.delete(),
            Key::Ctrl('c') => {
                write!(stdout, "^C\r\n").unwrap();
                return Ok(InputType::Termination)
            },
            Key::Char('\n') => {
                write!(stdout, "\r\n").unwrap();
                return Ok(InputType::String(inp_buf.take()))
            },
            Key::Char(ch) => {
                inp_buf.insert_char(ch)
            },
            _ => {}
        }
        inp_buf.render(&prompt, &mut stdout).unwrap();
        stdout.flush().unwrap();
    }

    Ok(InputType::Termination)
}
