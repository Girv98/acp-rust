use crate::constants::Squares;



#[derive(Debug, Default, Clone, Copy)]
pub struct Move {
    pub piece: u8,
    pub from_sq: Squares,   // NOTE: Only a single bit should be set
    pub to_sq: Squares,     // NOTE: 〃
    pub is_capture: bool, 
    pub is_promotion: bool,

}

impl Move {
    pub fn new(piece:u8, from_sq: Squares, to_sq: Squares, is_capture: bool, is_promotion: bool) -> Self {
        Self { piece, from_sq, to_sq, is_capture, is_promotion }
    }
}
