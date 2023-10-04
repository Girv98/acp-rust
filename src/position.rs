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
    pub blacks_move: bool,
    pub castling_rights: u8,            // lowest 4 bits used KQkq   uppercase = White lower = Black 
    pub castling_restrictions: u8,      // ^^
    pub ply_clock: u8,                  // For 50 move rule
    pub en_passant_targ: Option<u64>,   // is None if last ply was not a double push
    pub check: InCheck,
    pub last_ply: Option<Ply>           // is None if start of a game or if unknowable from FEN input
}

impl Position {
    pub fn new() -> Self {
        Position::default()
    }
}
