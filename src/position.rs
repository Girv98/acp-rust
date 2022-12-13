use super::board::*;
use super::mov::Move;

#[derive(Debug, Default, Copy, Clone)]
pub struct Position {
    pub board: Board,
    pub blacks_move: bool,
    pub castle_rights: u8, // KQkq   WB 
    pub ply_clock: u8,
    pub en_passant_targ: u64,
    pub check: Option<bool>, // White In Check = Some(false) Black in Check = Some(True), Neither = None,
    pub last_move: Option<Move> //is None if start of a game and/or if unknowable from FEN input
}

impl Position {
    pub fn new() -> Self {
        Position::default()
    }
}
