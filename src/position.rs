use crate :: { 
    board :: *, core::{Square, RANK_1, RANK_8}, 
    ply   :: { Colour, Movement, Piece, Ply }
};

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
    pub check: Option<Colour>,          // is None if neither king is in check
    pub last_ply: Option<Ply>           // is None if start of a game or if unknowable from FEN input
}

impl Position {
    pub fn new() -> Self {
        Position::default()
    }

    pub fn colour_can_long_castle(&self, colour: Colour) -> bool {
        match colour {
            Colour::White => 1 << 6 & self.castling != 0 && 1 << 2 & self.castling == 0,
            Colour::Black => 1 << 4 & self.castling != 0 && 1 << 0 & self.castling == 0,
        }
    }

    pub fn colour_can_short_castle(&self, colour: Colour) -> bool {
        match colour {
            Colour::White => 1 << 7 & self.castling != 0 && 1 << 3 & self.castling == 0,
            Colour::Black => 1 << 5 & self.castling != 0 && 1 << 1 & self.castling == 0,
        }
    }

    pub fn colour_is_in_check(&self, colour: Colour) -> bool {
        if let Some(c) = self.check {
            colour == c
        } else {
            false
        }
    }

    pub fn analyse_move(ply: Movement) -> Ply {
        todo!()
    }

    pub fn move_is_promotion(ply: Movement) -> bool {
        if ply.piece != Piece::Pawn {
            return false  
        }
        match ply.player {
            Colour::White => ply.to_sq.as_bb() & RANK_8 != 0,
            Colour::Black => ply.to_sq.as_bb() & RANK_1 != 0,
        }
    }

    pub fn move_is_checkmate(&self, ply: Movement) -> bool {
        todo!()
    }

    pub fn move_is_check(&self, ply: Movement) -> bool {
        // both direct attack and discovery
        todo!()
    }
    
    pub fn move_is_discovery_check(&self, ply: Movement) -> bool {
        todo!()
    }

    pub fn piece_is_pinned(&self, sq: Square) -> bool {
        // find pieces that's 
        todo!()
    }

    pub fn move_is_capture(&self, ply: Movement) -> bool {
        // NOTE: does not check if move is legal. e.g. if capturing king or if pinned
        match ply.player {
            Colour::White => self.board.black_bb() & ply.to_sq.as_bb() != 0,
            Colour::Black => self.board.white_bb() & ply.to_sq.as_bb() != 0
        }
    }
}
