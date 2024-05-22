use crate::core::Square;

use super::board::*;
use super::ply::Ply;

#[derive(Debug, Default, Copy, Clone)]
pub enum InCheck {
    #[default]
    None,
    White,
    Black,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
    pub board: Board,
    pub was_blacks_move: bool,
    // Upper nibble used for castling rights
    // Lower nibble used for temp restrictions TODO(James): check if this is even necessary 
    // KQkqKQkq :  uppercase = White, lower = Black 
    pub castling: u8,
    pub ply_clock: u8,                  // For 50 move rule
    pub en_passant_targ: Option<u64>,   // is None if last ply was not a double push
    pub check: InCheck,
    pub last_ply: Option<Ply>           // is None if start of a game or if unknowable from FEN input
}

impl Position {
    pub fn new() -> Self {
        Position::default()
    }

    pub fn from_fen(fen: &str) -> Self {
        // TODO(James): evaluate checks and temporary castling restrictions
        let mut position = Self::new();
        
        let fen_parts = fen.split(' ').collect::<Vec<_>>();
        
        if fen_parts.len() != 6 {
            panic!("Malformed FEN") // TODO: Return Error
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
                // Black Pieces
                'p' => { position.board.b_p_bb += 1 << (rank * 8 + file); file += 1; },
                'r' => { position.board.b_r_bb += 1 << (rank * 8 + file); file += 1; },
                'n' => { position.board.b_n_bb += 1 << (rank * 8 + file); file += 1; },
                'b' => { position.board.b_b_bb += 1 << (rank * 8 + file); file += 1; },
                'q' => { position.board.b_q_bb += 1 << (rank * 8 + file); file += 1; },
                'k' => { position.board.b_k_bb += 1 << (rank * 8 + file); file += 1; },
                // White Pieces
                'P' => { position.board.w_p_bb += 1 << (rank * 8 + file); file += 1; },
                'R' => { position.board.w_r_bb += 1 << (rank * 8 + file); file += 1; },
                'N' => { position.board.w_n_bb += 1 << (rank * 8 + file); file += 1; },
                'B' => { position.board.w_b_bb += 1 << (rank * 8 + file); file += 1; },
                'Q' => { position.board.w_q_bb += 1 << (rank * 8 + file); file += 1; },
                'K' => { position.board.w_k_bb += 1 << (rank * 8 + file); file += 1; },
                _ => panic!("Unknown character in FEN placement data")
            }
        }
        // Active Colour
        match fen_parts[1] {
            // With this, the starting position will have technically be black's move
            // But it shouldn't affect game logic
            "w" => position.was_blacks_move = true,
            "b" => position.was_blacks_move = false,
            _ => panic!("Malformed FEN active colour")
        }
        // Castling Rights
        match fen_parts[2] {
            "-" => {},
            _ => {
                for c in fen_parts[2].chars() {
                    match c {
                        'K' => { position.castling |= 1 << 7 },
                        'Q' => { position.castling |= 1 << 6 },
                        'k' => { position.castling |= 1 << 5 },
                        'q' => { position.castling |= 1 << 4 },
                        _  => panic!("Unknown character in FEN castling rights")
                    }
                }
            }
        }
        // En Passant Target
        match Square::str_to_u8(fen_parts[3]) {
            Some(np) => position.en_passant_targ = Some(np),
            None if fen_parts[3] == "-" => position.en_passant_targ = None,
            None => panic!("Malformed En Passant Target. Can be either a square (i.e. 'A6') or a dash '-' denoting that there is no valid square."),
        }
        // Halfmove (ply) clock (used for 50 move rule)
        match fen_parts[4].parse() {
            Ok(num) => position.ply_clock = num,
            _ => panic!("Malformed FEN halfmove clock")
        }

        position
    }
}
