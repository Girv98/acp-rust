use crate::core::Square;


#[derive(Debug, Default, Clone, Copy)]
/// Represents a ply or turn
pub struct Ply {
    pub piece: u8,
    pub from_sq: Square,
    pub to_sq: Square,
    pub is_capture: bool,
    pub is_promotion: bool,

}

impl Ply {
    pub fn new(piece:u8, from_sq: Square, to_sq: Square, is_capture: bool, is_promotion: bool) -> Self {
        Self { piece, from_sq, to_sq, is_capture, is_promotion }
    }
}
