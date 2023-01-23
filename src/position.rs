use super::board::*;
use super::mov::Move;

#[derive(Debug, Default, Copy, Clone)]
pub enum InCheck {
    #[default]
    None,
    Black,
    White
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
    pub board: Board,
    pub blacks_move: bool,
    pub castling_rights: u8,        // lowest 4 bits used KQkq   uppercase = White lower = Black 
    pub castling_restrictions: u8,  // ^^
    pub ply_clock: u8,
    pub en_passant_targ: u64,
    pub check: InCheck,
    pub last_move: Option<Move>     // is None if start of a game and/or if unknowable from FEN input
}

impl Position {
    pub fn new() -> Self {
        Position::default()
    }
}
