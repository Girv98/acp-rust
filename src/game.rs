
use crate::constants::{INITIAL_FEN, squares};

use super::position::Position;


#[derive(Debug, Default)]
pub struct Game {
    pub history: Vec<Position>,
    pub position: Position,
    pub ply: u16,
    pub mve: u16,

}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn init(&mut self, f: String) {
        
        let fen = match f.as_str() {
            "" => INITIAL_FEN.to_string(),
            _ => f
        };

        let fen_parts = fen.split(' ').collect::<Vec<_>>();

        if fen_parts.len() != 6 {
            panic!("Malformed FEN")
        }

        let mut file = 0;
        let mut rank = 7;
        // Piece Placement
        for c in fen_parts[0].chars() {
            match c {
                '/' => {
                    file = 0;
                    rank -= 1;
                },
                '0' ..= '9' => {
                    file += c.to_digit(10).unwrap();
                },
                'p' => { self.position.board.b_p_bb += 1 << (rank * 8 + file); file += 1; },
                'r' => { self.position.board.b_r_bb += 1 << (rank * 8 + file); file += 1; },
                'n' => { self.position.board.b_n_bb += 1 << (rank * 8 + file); file += 1; },
                'b' => { self.position.board.b_b_bb += 1 << (rank * 8 + file); file += 1; },
                'q' => { self.position.board.b_q_bb += 1 << (rank * 8 + file); file += 1; },
                'k' => { self.position.board.b_k_bb += 1 << (rank * 8 + file); file += 1; },

                'P' => { self.position.board.w_p_bb += 1 << (rank * 8 + file); file += 1; },
                'R' => { self.position.board.w_r_bb += 1 << (rank * 8 + file); file += 1; },
                'N' => { self.position.board.w_n_bb += 1 << (rank * 8 + file); file += 1; },
                'B' => { self.position.board.w_b_bb += 1 << (rank * 8 + file); file += 1; },
                'Q' => { self.position.board.w_q_bb += 1 << (rank * 8 + file); file += 1; },
                'K' => { self.position.board.w_k_bb += 1 << (rank * 8 + file); file += 1; },
                _ => panic!("Unknown character in FEN placement data")
            }
        }
        // Active Colour
        match fen_parts[1] {
            "w" => self.position.blacks_move = false,
            "b" => self.position.blacks_move = true,
            _ => panic!("Malformed FEN active colour")
        }
        // Castling Rights
        match fen_parts[2] {
            "-" => {},
            _ => {
                for c in fen_parts[2].chars() {
                    match c {
                        'K' => { self.position.castle_rights += 1 << 3 },
                        'Q' => { self.position.castle_rights += 1 << 2 },
                        'k' => { self.position.castle_rights += 1 << 1 },
                        'q' => { self.position.castle_rights += 1 << 0 },
                        _  => panic!("Unknown character in FEN castling rights")
                    }
                }
            }
        }

        match squares(fen_parts[3]) {
            Some(np) => self.position.en_passant_targ = np,
            None => if fen_parts[3] != "-" {panic!("Malformed En Passant Target. Can be either a square (i.e. 'A6') or a dash '-' denoting that there is no valid square.")},
        }
        
        // Halfmove (ply) clock (use for 50 move rule)
        match fen_parts[4].parse() {
            Ok(num) => self.position.ply_clock = num,
            _ => panic!("Malformed FEN halfmove clock")
        }
        // Fullmove counter 
        match fen_parts[5].parse() {
            Ok(num) => self.mve = num,
            _ => panic!("Malformed FEN fullmoves")
        }

        self.ply = self.mve * 2;
        
        if !self.position.blacks_move {
            self.ply -= 1;
        }


        // TODO: evaluate checks

        self.history.push(self.position);
    }

    pub fn generate_fen(&self) -> String {
        todo!()
    }
}