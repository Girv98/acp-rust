use std::collections::HashMap;

use colored::Colorize;

use crate    :: {
    position :: Position, 
    ply      :: { Colour, Piece, Ply, Movement, PromotablePiece }, 
    core     :: { 
        repl :: { self, InputType }, 
        parsers, Square, INITIAL_FEN,
    }, 
};

#[derive(Debug)]
pub enum DrawKind {
    Stalemate,
    Mutual,
    ThreeFold,
    FiveFold,
    FiftyMove,
}

#[derive(Debug)]
pub enum WinKind {
    Surrender,
    Checkmate
}

#[derive(Debug)]
pub enum State {
    White(WinKind),
    Black(WinKind),
    Draw(DrawKind),
    Terminated
}

#[derive(Debug, Default)]
pub struct Game {
    pub history: Vec<Position>,
    pub ply: u16,
    pub mov: u16,
    pub state: Option<State>
}

impl Game {
    pub fn new() -> Self {
        Self::try_from_fen(INITIAL_FEN).expect("INITIAL_FEN is not malformed")
    }

    pub fn as_fen(&self) -> String {
        let mut fen = String::new();
        let last_pos = self.last_position();

        { // Piece Placement
            fen.push_str(&last_pos.board.stringify());
            fen.push(' ');
        }
        { // Active Colour
            match last_pos.was_blacks_move {
                true  => fen.push_str("w "),
                false => fen.push_str("b "),
            }
        }
        { // Castling Rights
            let mut buf = String::with_capacity(5);

            if last_pos.castling >> 7     == 1 { buf.push('K'); }
            if last_pos.castling >> 6 & 1 == 1 { buf.push('Q'); }
            if last_pos.castling >> 5 & 1 == 1 { buf.push('k'); }
            if last_pos.castling >> 4 & 1 == 1 { buf.push('q'); }
            
            if buf.is_empty() {
                buf.push('-');
            }
            buf.push(' ');
            fen.push_str(&buf);
        }
        { // En Passant Target
            match last_pos.en_passant_targ {
                Some(x) => {
                    let sq = Square::bb_to_str(x);
                    fen.push_str(sq);
                    fen.push(' ');
                },
                None => {
                    fen.push_str("- ");
                }
            }
        }
        { // PLy Clock
            fen.push_str(&last_pos.ply_clock.to_string());
            fen.push(' ');
        }
        { // Move Counter
            fen.push_str(&self.mov.to_string());
        }
        fen
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, String> {
        parsers::fen::parse_fen(fen)
    }

    pub fn as_pgn(&self) -> String {
        todo!()
    }

    pub fn from_pgn(pgn: &str) -> Self {
        todo!()
    }

    pub fn last_position(&self) -> &Position {
        self.history.last().expect("Game History is not empty")
    }

    pub fn stringify_board(&self, colour: Colour) -> String {
        match colour {
            Colour::White => self.last_position().board.get_board(true),
            Colour::Black => self.last_position().board.get_board(false)
        }
    }

    pub fn print_help() {
        println!("TODO\n");
    }

    pub fn play_two_player(&mut self) {

        while self.state.is_none() {
            let (player, prompt) = match self.last_position().was_blacks_move {
                true  => (Colour::White, "White to play: ".bright_blue()),
                false => (Colour::Black, "Black to play: ".bright_red()),
            };
                
            // self.get_board(player);
            // println!("Move: {} Ply: {}\r\n", self.mov, self.ply);

            let usr_input = match repl::get_input(&self, player, &prompt).unwrap() {
                InputType::String(inp) => match inp.to_lowercase().trim() {
                    "" => unreachable!(),
                    "quit" | "exit" => return,
                    "help" | "-h" => {
                        Self::print_help();
                        continue
                    },
                    "draw" => { 
                        // let (_nxt_player, prompt) = match player {
                        //     Colour::White => (Colour::Black, "Black: Do you except a draw? (y/n) ".bright_red()),
                        //     Colour::Black => (Colour::White, "White: Do you except a draw? (y/n) ".bright_blue()),
                        // };

                        // match repl::get_input(&prompt).unwrap() {
                        //     InputType::String(inp) => match inp.to_lowercase().trim() {
                        //         "y" | "yes" => println!("Draw"),
                        //         "n" | "no"  => continue,
                        //         _ => continue
                        //     },
                        //     InputType::Termination => return
                        // }
                        // return
                        unreachable!()
                    },
                    "surrender" | "surr" | "sur" => { 
                        match player {
                            Colour::White => {
                                println!("Black wins by surrender");
                                self.state = Some(State::Black(WinKind::Surrender));
                            },
                            Colour::Black => {
                                println!("White wins by surrender");
                                self.state = Some(State::White(WinKind::Surrender));
                            },
                        }
                        break
                    },
                    _ => inp,
                },
                InputType::Termination => return,
            };

            let Some(mov) = self.parse_user_input(player, &usr_input).unwrap() else {
                println!("Move not valid");
                continue;
            };
            if self.validate_movement(mov).is_none() {
                println!("Move not possible");
                continue;
            }

            if self.update(mov).is_err() {
                break
            }
        }
    }

    pub fn promote(&mut self, ply: Ply) -> Option<Ply> {
        let mut ply = ply;

        let prompt = match ply.mov.player {
            Colour::White => "Promote to what? (q/b/n/r): ".bright_blue(),
            Colour::Black => "Promote to what? (q/b/n/r): ".bright_red(),
        };

        match repl::get_input(&self, ply.mov.player, &prompt).unwrap() {
            InputType::String(inp) => match inp.to_lowercase().as_str() {
                "quit" | "exit" => return None,
                "q" => {
                    ply.promotion = Some(PromotablePiece::Queen);
                    // todo
                },
                "b" => {
                    ply.promotion = Some(PromotablePiece::Bishop);
                    // todo
                }
                "n" => {
                    ply.promotion = Some(PromotablePiece::Knight);
                    // todo
                },
                "r" => {
                    ply.promotion = Some(PromotablePiece::Rook);
                    // todo
                },
                _ => todo!(),
            },
            InputType::Termination => return None,
        }
        Some(ply)
    }

    fn update(&mut self, next_move: Movement) -> Result<(),()> {
        // Note: Assumes move has been validated
        let mut pos = self.last_position().clone();
        // Update Position, promote PLyMove to Ply by checking is_capture/is_promotion
        let ply = Ply::from_move(next_move, false, None);
        // check capture
        let ply = self.promote(ply).ok_or(())?;
        // Update Castling
        // Update Ply_Clock
        // Update En_Passant_Targ
        // Update Check
        
        // Update Last_Ply
        pos.last_ply = Some(ply);

        self.mov = if pos.was_blacks_move { self.mov + 1} else { self.mov };
        self.ply = (self.ply % 2) + 1;
        self.history.push(pos);

        Ok(())
    }

    fn parse_user_input(&self, player: Colour, inp: &str) -> Result<Option<Movement>, String> {
        // parse input i.e. e4
        // find corresponding piece i.e. e-pawn
        // return 

        // Pe4  => E Pawn to e4
        // Pexd => E Pawn captures d side
        // K b1 c3 => Knight B1 to C3

        let inp = inp.to_lowercase();
        let mut chars = inp.chars().peekable();

        let piece = match chars.next().expect("inp has at least one char") {
            'n' => Piece::Knight,
            'b' => match chars.peek() {
                Some('1'..='8') => Piece::Pawn,
                Some(_) => Piece::Bishop,
                _ => Piece::Bishop
            },
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            'k' => Piece::King,
            'a'..='h' => Piece::Pawn,
            c => return Err(format!("Unknown char: '{}'", c))
        };

        match chars.peek() {
            Some('x') => {
                chars.next();
            },
            None => return Err(format!("No more chars")),
            _ => {},
        }

        let pos = match chars.next() {
            Some(file @ 'a'..='h') => match chars.next() {
                Some(rank @ '1'..='8') => {
                    format!("{file}{rank}")
                },
                None => return Err(format!("No more chars")),
                Some(c) => return Err(format!("aUnknown char: '{}'", c))
            },
            None => return Err(format!("No more chars")),
            Some(c) => return Err(format!("bUnknown char: '{}'", c))
        };

        println!("P: {piece:?}\nM: {pos}");

        // if 

        // for c in inp.chars() {

        // }

        todo!()
    }

    pub fn validate_movement(&self, inp: Movement) -> Option<Ply> {
        todo!()
    }
}