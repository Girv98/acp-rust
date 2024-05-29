use crate::core::Square;

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum PromotablePiece {
    Knight,
    Bishop,
    Rook, 
    Queen
}

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    White,
    Black
}

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    pub player: Colour,
    pub piece: Piece,
    pub from_sq: Square,
    pub to_sq: Square,
}

impl Movement {
    pub fn new(player: Colour, piece: Piece, from_sq: Square, to_sq: Square) -> Self {
        Self { player, piece, from_sq, to_sq }
    }
}

#[derive(Debug, Clone, Copy)]
/// Represents a ply or turn
pub struct Ply {
    pub mov: Movement,
    pub is_capture: bool,
    pub promotion: Option<PromotablePiece>,

}

impl Ply {
    pub fn new(player: Colour, piece: Piece, from_sq: Square, to_sq: Square, is_capture: bool, is_promotion: Option<PromotablePiece>) -> Self {
        Self { mov: Movement::new(player, piece, from_sq, to_sq), is_capture, promotion: is_promotion }
    }

    pub fn from_move(mov: Movement, is_capture: bool, promotion: Option<PromotablePiece>) -> Self {
        Self { mov, is_capture, promotion }
    }
}
