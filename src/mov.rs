

#[derive(Debug, Default, Clone, Copy)]
pub struct Move {
    pub piece: u8,
    pub from_sq: u64,   // IMPL: Only single bit should be set
    pub to_sq: u64,     // IMPL: 〃
    pub is_capture: bool, 
    pub is_promotion: bool,

}

impl Move {
    fn from_sq_sanity_check(&self) {
        if self.from_sq.count_ones() != 1 {
            panic!("Move.from_sq contains more than one square")
        }
    }
     
    fn to_sq_sanity_check(&self) {
        if self.to_sq.count_ones() != 1 {
            panic!("Move.from_sq contains more than one square")
        }
    }



}
