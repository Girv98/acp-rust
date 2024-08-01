use std::collections::HashMap;
use std::io::Write;
use std::io;

use colored::ColoredString;
use termion::clear;
use termion::cursor;

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

use crate::ply::Colour;
use crate::ply::Movement;
use crate::Game;

use super::Square;


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

    pub fn take_raw(&mut self) -> String {
        let res = self.buffer.iter().collect();
        self.clear();
        res
    }

    pub fn take_trimmed(&mut self) -> String {
        let res = self.buffer.iter().collect::<String>().trim().to_string();
        self.clear();
        res
    }

    pub fn peak_trimmed(&self) -> String {
        self.buffer.iter().collect::<String>().trim().to_string()
    }

    pub fn is_only_whitespace(&self) -> bool {
        self.buffer.iter().collect::<String>().trim() == ""
    }

    pub fn is_in_responses(&self, responses: &HashMap<&str, &str>) -> bool {
        responses.contains_key(self.buffer.iter().collect::<String>().to_lowercase().trim())
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

    pub fn render(&self, board: &String, prompt: &ColoredString, response: &Option<String>, sink: &mut impl Write) -> io::Result<()> {
        let buf: String = self.buffer.iter().collect();
        write!(sink, "\r{}{}{}\r", cursor::Up(11), board, cursor::Down(3))?;
        write!(sink, "\r{}{}{}\r", clear::AfterCursor, prompt, &buf)?;

        if let Some (resp) = response {
            // protects against wrapping
            let (x, _) = termion::terminal_size()?;
            write!(sink, "\n{}\r{}", resp, cursor::Up(1 + resp.chars().count() as u16 / x))?;
        }

        write!(sink,"{}", cursor::Right((prompt.len() + self.cursor).try_into().unwrap()))?;
        Ok(())
    }
}

pub enum InputType {
    String(String),
    Termination
}

fn update_response(game: &Game, colour: Colour, input: &String) -> (Option<String>, Option<String>) {
    // NOTE: assumes input is lowercase
    
    let buf: Vec<_> = input.split_whitespace().collect();

    let pos = game.last_position();

    if buf.len() == 1 {
        match buf[0] {
            "" => unreachable!(),
            "draw" => (None, Some("Must make a move before offering a draw".to_string())),
            x => {
                let Some(sq) = Square::from_str(x) else { return (None, None) };

                match pos.board.piece_at(sq.to_bb()) {
                    Some((p, c)) if c == colour => {
                        (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, sq)), None)
                    },
                    Some((p, c)) if c != colour => {
                        (None, Some(format!("{p} at {x} is not yours")))
                    },
                    _ => (None, Some(format!("No piece at {x}"))) // No owned piece at position
                }
            },
        }
    } else if buf.len() == 2 {
        match (buf[0], buf[1]) {
            (x, "draw") | ("draw", x) => {
                let Some(sq) = Square::from_str(x) else { return (None, Some("Must make a move before offering a draw".to_string())) };

                match pos.board.piece_at(sq.to_bb()) {
                    Some((p, c)) if c == colour => {
                        (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, sq)), Some("Must make a move before offering a draw".to_string()))
                    },
                    _ => (None, Some("Must make a move before offering a draw".to_string())), // No owned piece at position
                }
        },
            (x, y) => {
                let Some(fr_sq) = Square::from_str(x) else { return (None, Some(format!("Unknown input `{x}`"))) };
                let Some(to_sq) = Square::from_str(y) else { return (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, fr_sq)), Some(format!("Unknown input `{y}`"))) };
                // {
                //     let mov = Movement::new(colour, pos.board.piece_at(to_sq.to_bb()).unwrap().0, fr_sq, to_sq);
                //     if let Some(ply) = game.validate_movement(mov) {
                    
                //     } else { return None }
                // }

                match (pos.board.piece_at(fr_sq.to_bb()), pos.board.piece_at(to_sq.to_bb())) {
                    (None, _) => (None, Some(format!("No piece at {x}"))),
                    (Some((fp, fc)), None) if fc == colour => 
                    (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, fr_sq)), Some(format!("{fp} to {y}"))),
                    (Some((fp, fc)), None) if fc != colour => (None, Some(format!("{fp} at {x} is not yours"))),
                    (Some((fp, fc)), Some((tp, tc))) if fc == colour && tc != colour => {
                        let pos_moves = match fp {
                            crate::ply::Piece::Pawn => game.last_position().board.pawn_attacks(fr_sq.as_bb(), fc),
                            crate::ply::Piece::Knight => game.last_position().board.knight_attacks(fr_sq.as_bb(), fc),
                            crate::ply::Piece::Bishop => game.last_position().board.bishop_attacks(fr_sq.as_bb(), fc),
                            crate::ply::Piece::Rook => game.last_position().board.rook_attacks(fr_sq.as_bb(), fc),
                            crate::ply::Piece::Queen => game.last_position().board.queen_attacks(fr_sq.as_bb(), fc),
                            crate::ply::Piece::King => game.last_position().board.king_attacks(fr_sq.as_bb(), fc),
                        };

                        if to_sq.as_bb() & pos_moves != 0 {
                            (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, fr_sq)), Some(format!("{fp} takes {tp}")))
                        } else {
                            (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, fr_sq)), Some(format!("Not a valid attack")))
                        }

                    },
                    (Some((fp, fc)), Some((tp, tc))) if fc == colour && tc == colour => {
                        (Some(game.last_position().board.get_possible_moves_board(colour == Colour::White, fr_sq)), Some(format!("TODO: Check for castling")))
                    },
                    _ => (None, None)

                }


            },
        }
    } else {
        (None, None)
    }
}

fn validate_input(game: &Game, colour: Colour, input: &String) -> Option<String> {
    todo!()

    // * convert input to `Movement` *
    // game.validate_movement(movement)
}

pub fn get_input(game: &Game, colour: Colour, prompt: &ColoredString) -> Result<InputType, io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    
    write!(stdout, "{}\n\n", format!("{}", game.stringify_board(colour)))?;
    write!(stdout, "Move: {} Ply: {}\r\n", game.mov, game.ply)?;
    write!(stdout, "{}{}", prompt, cursor::BlinkingBar)?;
    stdout.flush().unwrap();
    
    let stdin = io::stdin();
    let mut inp_buf = InputBuffer::default();
    let mut board = game.stringify_board(colour);
    let mut response: Option<String> = None;

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Home => inp_buf.jump_start(),
            Key::End => inp_buf.jump_end(),
            Key::Left => inp_buf.left_char(),
            Key::Right => inp_buf.right_char(),
            Key::Backspace => {
                inp_buf.backspace();
                let (brd, resp) = update_response(game, colour, &inp_buf.peak_trimmed().to_lowercase());
                if let Some(b) = brd { board = b } else { board = game.stringify_board(colour) }
                response = resp;
            },
            Key::Delete => {
                inp_buf.delete();
                let (brd, resp) = update_response(game, colour, &inp_buf.peak_trimmed().to_lowercase());
                if let Some(b) = brd { board = b } else { board = game.stringify_board(colour) }
                response = resp;
            },
            Key::Char('\n') => if !inp_buf.buffer.is_empty() && !inp_buf.is_only_whitespace() {
                let (brd, resp) = update_response(game, colour, &inp_buf.peak_trimmed().to_lowercase());
                if let Some(b) = brd { board = b } else { board = game.stringify_board(colour) }
                response = resp;
                if response.is_none() {
                    write!(stdout, "\r\n")?;
                    return Ok(InputType::String(inp_buf.take_trimmed()))
                }
            },
            Key::Char(ch) => {
                inp_buf.insert_char(ch);
                let (brd, resp) = update_response(game, colour, &inp_buf.peak_trimmed().to_lowercase());
                if let Some(b) = brd { board = b } else { board = game.stringify_board(colour) }
                response = resp;
            },
            Key::Ctrl('c') => {
                write!(stdout, "^C\r\n").unwrap();
                return Ok(InputType::Termination)
            },
            _ => {}
        }

        inp_buf.render(&board, &prompt, &response, &mut stdout).unwrap();
        stdout.flush().unwrap();
    }

    Ok(InputType::Termination)
}

pub struct Match {
    game: Game,

}

impl Match {
    pub fn play_two_player(&mut self) {
        
    }

}

