use colored::Colorize;

use crate    :: {
    core     :: { repl::{self, InputType}, Square, INITIAL_FEN }, 
    ply      :: Colour, 
    position :: Position 
};

#[derive(Debug, Default)]
pub struct Game {
    pub history: Vec<Position>,
    pub ply: u16,
    pub mve: u16,
}

impl Game {
    pub fn new() -> Self {
        Self::from_fen(INITIAL_FEN)
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
            fen.push_str(&self.mve.to_string());
        }
        fen
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut game: Game = Game::default();
        let position = Position::from_fen(fen);

        let fen_parts = fen.split(' ').collect::<Vec<_>>(); // FIXME: We are doing this twice with Position::from_fen

        // Fullmove counter 
        match fen_parts[5].parse() {
            Ok(num) => game.mve = num,
            _ => panic!("Malformed FEN fullmoves")
        }
        // Set Ply
        game.ply = game.mve * 2;
        if position.was_blacks_move {
            game.ply -= 1;
        }

        game.history.push(position);

        game
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

    pub fn print_board(&self, colour: Colour) {
        match colour {
            Colour::White => self.last_position().board.print_board(true),
            Colour::Black => self.last_position().board.print_board(false)
        }
    }


    pub fn play_two_player(&mut self) {
        loop {
            let (player, prompt) = match self.last_position().was_blacks_move {
                true  => (Colour::White, "White to play: ".bright_blue()),
                false => (Colour::Black, "Black to play: ".bright_red()),
            };
                
            self.print_board(player);
            println!("Move: {} Ply: {}\r\n", self.mve, self.ply);

            match repl::get_input(&prompt).unwrap() {
                InputType::String(inp) => match inp.as_str() {
                    "quit" | "exit "=> return,
                    _ => todo!("parse input\r\n")
                },
                InputType::Termination => return,
            }
        }
    }

    // pub fn play_one_player(&mut self, player_colour: Colour) {
    //     let _is_players_turn = match player_colour {
    //         Colour::White => self.last_position().was_blacks_move,
    //         Colour::Black => !self.last_position().was_blacks_move,
    //     };
    //     todo!()
    // }

    // fn analyse_user_input(&self, buffer: String) -> Ply {
    //     todo!()
    // }

    // pub fn play_two_player(&mut self) {
    //     loop {
    //         let (player, prompt) = match self.last_position().was_blacks_move {
    //             true  => (Colour::White, "White to play: ".bright_blue()),
    //             false => (Colour::Black, "Black to play: ".bright_red()),
    //         };

    //         let mut user_buffer = String::new(); { 
    //             self.print_board(player);
    //             println!("\nMove: {} Ply: {}", self.mve, self.ply);
    //             print!("{prompt}");
    //             io::stdout().flush().unwrap();
    //             io::stdin().read_line(&mut user_buffer).unwrap();
    //         }

    //         println!("{:?}", user_buffer.chars().collect::<Vec<_>>());
    //         println!("You typed: {user_buffer}");

    //         let next_move = self.analyse_user_input(user_buffer);
    //         // TODO: Do things with move
            
    //         let mut next_pos = *self.last_position();
    //         next_pos.was_blacks_move = !next_pos.was_blacks_move;
    //         // Update Position
    //         // Update Castling
    //         // Update Ply_Clock
    //         // Update En_Passant_Targ
    //         // Update Check
    //         // Update Last_Ply
    //         self.history.push(next_pos);
    //         // Update Move & Ply
    //         self.mve = if next_pos.was_blacks_move { self.mve + 1} else {self.mve };
    //         self.ply = (self.ply % 2) + 1
    //     }
    // }
}