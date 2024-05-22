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
pub enum Colour {
    White,
    Black
}

#[derive(Debug, Clone, Copy)]
/// Represents a ply or turn
pub struct Ply {
    pub player: Colour,
    pub piece: Piece,
    pub from_sq: Square,
    pub to_sq: Square,
    pub is_capture: bool,
    pub is_promotion: bool,

}

#[allow(dead_code)]
impl Ply {
    pub fn new(player: Colour, piece: Piece, from_sq: Square, to_sq: Square, is_capture: bool, is_promotion: bool) -> Self {
        Self { player, piece, from_sq, to_sq, is_capture, is_promotion }
    }
}
