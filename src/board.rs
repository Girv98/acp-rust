
use colored::Colorize;

use crate::constants;

#[derive(Debug, Default, Copy, Clone)]
pub struct Board {
    pub w_p_bb: u64, // white pawn bitboard
    pub w_r_bb: u64, // white rook bitboard
    pub w_n_bb: u64, // white knight bitboard
    pub w_b_bb: u64, // white bishop bitboard
    pub w_q_bb: u64, // white queen bitboard
    pub w_k_bb: u64, // white king bitboard

    pub b_p_bb: u64, // black pawn bitboard
    pub b_r_bb: u64, // black rook bitboard
    pub b_n_bb: u64, // black knight bitboard
    pub b_b_bb: u64, // black bishop bitboard
    pub b_q_bb: u64, // black queen bitboard
    pub b_k_bb: u64, // black king bitboard
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    pub fn get_all_occupied_bb(&self) -> u64 {
        // Bitwise-ORs all piece bitboards
        self.get_wht_bb() | self.get_blk_bb()
    }

    pub fn get_blk_bb(&self) -> u64 {
        // Bitwise-ORs all black piece bitboards
        self.b_p_bb | self.b_r_bb | self.b_n_bb | self.b_b_bb | self.b_q_bb | self.b_k_bb
    }

    pub fn get_wht_bb(&self) -> u64 {
        // Bitwise-ORs all black piece bitboards
        self.w_p_bb | self.w_r_bb | self.w_n_bb | self.w_b_bb | self.w_q_bb | self.w_k_bb
    }

    pub fn get_all_blk_pawn_attacks(&self) -> u64 {
        constants::black_pawn_attacks_bb(self.b_p_bb)
    }

    pub fn get_all_wht_pawn_attacks(&self) -> u64 {
        constants::black_pawn_attacks_bb(self.w_p_bb)
    }

    pub fn get_all_blk_knight_moves(&self) -> u64 {
        constants::knight_moves_bb(self.b_n_bb)
    }

    pub fn get_all_wht_knight_moves(&self) -> u64 {
        constants::knight_moves_bb(self.w_n_bb)
    }

    pub fn print_board(&self) {

        let mut rank = String::with_capacity(88);
        let mut letters = vec!['8', '7', '6', '5', '4', '3', '2', '1'].into_iter();

        for i in 0..64 {
            let offset = 1 << (63-i);

            let prev = rank;
            // NOTE: ♙♖♘♗♕♔ ♟︎♞♝♜♛♚ don't line up properly 🙃
                 if self.w_p_bb & offset == offset { rank = format!("{}{}", " P".bright_blue(), &prev); } 
            else if self.w_r_bb & offset == offset { rank = format!("{}{}", " R".bright_blue(), &prev); } 
            else if self.w_n_bb & offset == offset { rank = format!("{}{}", " N".bright_blue(), &prev); } 
            else if self.w_b_bb & offset == offset { rank = format!("{}{}", " B".bright_blue(), &prev); }
            else if self.w_q_bb & offset == offset { rank = format!("{}{}", " Q".bright_blue(), &prev); }
            else if self.w_k_bb & offset == offset { rank = format!("{}{}", " K".bright_blue(), &prev); }
            else if self.b_p_bb & offset == offset { rank = format!("{}{}", " P".bright_red(),  &prev); } 
            else if self.b_r_bb & offset == offset { rank = format!("{}{}", " R".bright_red(),  &prev); } 
            else if self.b_n_bb & offset == offset { rank = format!("{}{}", " N".bright_red(),  &prev); } 
            else if self.b_b_bb & offset == offset { rank = format!("{}{}", " B".bright_red(),  &prev); } 
            else if self.b_q_bb & offset == offset { rank = format!("{}{}", " Q".bright_red(),  &prev); } 
            else if self.b_k_bb & offset == offset { rank = format!("{}{}", " K".bright_red(),  &prev); } 
            else { rank = format!("{}{}", " ·", &prev);}

            // println!("{}", rank.chars().count());
            if i % 8 == 7 {
                print!("{}", letters.next().unwrap());
                println!("{}", rank.as_str());
                rank = "".to_string();
            }
        }

        println!("~ A B C D E F G H");

    }
    
}
