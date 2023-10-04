
use colored::Colorize;

use crate::core::Square;
use crate::core;

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
    /// Gets bitboard of all unoccupied squares
    pub fn empty_bb(&self) -> u64 {
        !self.occupied_bb()
    }
    /// Gets bitboard of all occupied squares
    pub fn occupied_bb(&self) -> u64 {
        self.white_bb() | self.black_bb()
    }
    /// Gets bitboard of all white occupied squares
    pub fn white_bb(&self) -> u64 {
        self.w_p_bb | self.w_r_bb | self.w_n_bb | self.w_b_bb | self.w_q_bb | self.w_k_bb
    }
    /// Gets bitboard of all black occupied squares
    pub fn black_bb(&self) -> u64 {
        self.b_p_bb | self.b_r_bb | self.b_n_bb | self.b_b_bb | self.b_q_bb | self.b_k_bb
    }
    /// Gets bitboard of all black occupied squares that are under threat from white's pawns
    pub fn white_pawn_attacks(&self) -> u64 {
        core::pawn_attacks_bb(self.w_p_bb, true) & self.black_bb()
    }
    /// Gets bitboard of all white occupied squares that are under threat from black's pawns
    pub fn black_pawn_attacks(&self) -> u64 {
        core::pawn_attacks_bb(self.b_p_bb, false) & self.white_bb()
    }
    /// Gets bitboard of all white push squares
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_single_pushes(&self) -> u64 {
        core::pawn_single_pushes_bb(self.w_p_bb, self.empty_bb(), true)
    }
    /// Gets bitboard of all black push squares
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_single_pushes(&self) -> u64 {
        core::pawn_single_pushes_bb(self.b_p_bb, self.empty_bb(), false)
    }
    /// Gets bitboard of all white double push squares
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_double_pushes(&self) -> u64 {
        core::pawn_double_pushes_bb(self.w_p_bb, self.empty_bb(), true)
    }
    /// Gets bitboard of all black double push squares
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_double_pushes(&self) -> u64 {
        core::pawn_double_pushes_bb(self.b_p_bb, self.empty_bb(), false)
    }
    /// Gets bitboard of all possible white knight moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_knight_moves(&self) -> u64 {
        core::knight_moves_bb(self.w_n_bb) & self.empty_bb()
    }
    /// Gets bitboard of all possible black knight moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_knight_moves(&self) -> u64 {
        core::knight_moves_bb(self.b_n_bb) & self.empty_bb()
    }
    /// Gets bitboard of all black occupied squares that are under threat from white's knights
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_knight_attacks(&self) -> u64 {
        core::knight_moves_bb(self.w_n_bb) & self.black_bb()
    }
    /// Gets bitboard of all white occupied squares that are under threat from black's knights
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_knight_attacks(&self) -> u64 {
        core::knight_moves_bb(self.b_n_bb) & self.white_bb()
    }
    /// Gets bitboard of all possible white king moves
    /// NOTE: Doesn't account for illegal moves
    pub fn white_king_moves(&self) -> u64 {
        core::king_moves_bb(self.w_k_bb) & self.empty_bb()
    }
    /// Gets bitboard of all possible black king moves
    /// NOTE: Doesn't account for illegal moves
    pub fn black_king_moves(&self) -> u64 {
        core::king_moves_bb(self.b_k_bb) & self.empty_bb()
    }
    /// Gets bitboard of all black occupied squares that are under threat from white's king
    /// NOTE: Doesn't account for illegal moves 
    pub fn white_king_attacks(&self) -> u64 {
        core::king_moves_bb(self.w_k_bb) & self.black_bb()
    }
    /// Gets bitboard of all white occupied squares that are under threat from black's king
    /// NOTE: Doesn't account for illegal moves
    pub fn black_king_attacks(&self) -> u64 {
        core::king_moves_bb(self.b_k_bb) & self.black_bb()
    }
    /// Gets bitboard of all possible white bishop moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_bishop_moves(&self) -> u64 {
        core::bish_moves_bb(self.w_b_bb, self.empty_bb()) & self.empty_bb()
    }
    /// Gets bitboard of all possible black bishop moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_bishop_moves(&self) -> u64 {
        core::bish_moves_bb(self.b_b_bb, self.empty_bb()) & self.empty_bb()
    }
    /// Gets bitboard of all possible white bishop moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_bishop_attacks(&self) -> u64 {
        core::bish_moves_bb(self.w_b_bb, self.empty_bb()) & self.black_bb()
    }
    /// Gets bitboard of all possible black bishop moves.
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_bishop_attacks(&self) -> u64 {
        core::bish_moves_bb(self.b_b_bb, self.empty_bb()) & self.white_bb()
    }
    /// Gets bitboard of all possible white rook moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_rook_moves(&self) -> u64 {
        core::rook_moves_bb(self.w_r_bb, self.empty_bb()) & self.empty_bb()
    }
    /// Gets bitboard of all possible black rook moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_rook_moves(&self) -> u64 {
        core::rook_moves_bb(self.b_r_bb, self.empty_bb()) & self.empty_bb()
    }
    /// Gets bitboard of all possible white rook moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_rook_attacks(&self) -> u64 {
        core::rook_moves_bb(self.w_r_bb, self.empty_bb()) & self.black_bb()
    }
    /// Gets bitboard of all possible black rook moves.
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_rook_attacks(&self) -> u64 {
        core::rook_moves_bb(self.b_r_bb, self.empty_bb()) & self.white_bb()
    }
    /// Gets bitboard of all possible white queen moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_queen_moves(&self) -> u64 {
          core::rook_moves_bb(self.w_q_bb, self.empty_bb())
        | core::bish_moves_bb(self.w_q_bb, self.empty_bb()) 
        & self.empty_bb()
    }
    /// Gets bitboard of all possible black queen moves
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_queen_moves(&self) -> u64 {
          core::rook_moves_bb(self.b_q_bb, self.empty_bb())
        | core::bish_moves_bb(self.b_q_bb, self.empty_bb()) 
        & self.empty_bb()
    }
    /// Gets bitboard of all possible white queen attacks
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn white_queen_attacks(&self) -> u64 {
          core::rook_moves_bb(self.w_q_bb, self.empty_bb())
        | core::bish_moves_bb(self.w_q_bb, self.empty_bb()) 
        & self.black_bb()
    }
    /// Gets bitboard of all possible black queen attacks
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn black_queen_attacks(&self) -> u64 {
          core::rook_moves_bb(self.b_q_bb, self.empty_bb())
        | core::bish_moves_bb(self.b_q_bb, self.empty_bb()) 
        & self.white_bb()
    }
    /// Sees if a given square is under attack by any white piece
    pub fn sq_is_attacked_by_white(&self, sq: Square) -> bool {
        let sq = sq.to_bb();
        ( 
            self.white_pawn_attacks() 
          | self.white_knight_attacks()
          | self.white_king_attacks()
          | self.white_bishop_attacks()
          | self.white_rook_attacks()
          | self.white_queen_attacks()
        ) & sq == sq
    }
    /// Sees if a given square is under attack by any black piece
    pub fn sq_is_attacked_by_black(&self, sq: Square) -> bool {
        let sq = sq.to_bb();
        ( 
            self.black_pawn_attacks() 
          | self.black_knight_attacks()
          | self.black_king_attacks()
          | self.black_bishop_attacks()
          | self.black_rook_attacks()
          | self.black_queen_attacks()
        ) & sq == sq
    }

    pub fn is_white_king_in_check(&self) -> bool {
        self.sq_is_attacked_by_black(Square::from_bb(self.w_k_bb))
    } 

    pub fn is_black_king_in_check(&self) -> bool {
        self.sq_is_attacked_by_white(Square::from_bb(self.b_k_bb))
    } 

    pub fn is_piece_pinned_to_king(&self, sq: Square, is_white: bool) -> bool {
        let king = if is_white { self.w_k_bb } else { self.b_k_bb };
        let square = sq.to_bb();
        todo!()
    } 

    pub fn print_board(&self) {
        let mut rank = String::with_capacity(88);
        let mut letters = vec!['8', '7', '6', '5', '4', '3', '2', '1'].into_iter();

        for i in 0..64 {
            let offset = 1 << (63-i);
            let prev = rank;
            // NOTE: i would have liked to use ♙♖♘♗♕♔ ♟︎♞♝♜♛♚ but they don't seem to line up properly with certain fonts/terminals 🙃
            //White Pieces
                 if (self.w_p_bb & offset) == offset { rank = format!("{}{}", " P".bright_blue(), &prev); } 
            else if (self.w_r_bb & offset) == offset { rank = format!("{}{}", " R".bright_blue(), &prev); } 
            else if (self.w_n_bb & offset) == offset { rank = format!("{}{}", " N".bright_blue(), &prev); } 
            else if (self.w_b_bb & offset) == offset { rank = format!("{}{}", " B".bright_blue(), &prev); }
            else if (self.w_q_bb & offset) == offset { rank = format!("{}{}", " Q".bright_blue(), &prev); }
            else if (self.w_k_bb & offset) == offset { rank = format!("{}{}", " K".bright_blue(), &prev); }
            // Black pieces
            else if (self.b_p_bb & offset) == offset { rank = format!("{}{}", " P".bright_red(),  &prev); } 
            else if (self.b_r_bb & offset) == offset { rank = format!("{}{}", " R".bright_red(),  &prev); } 
            else if (self.b_n_bb & offset) == offset { rank = format!("{}{}", " N".bright_red(),  &prev); } 
            else if (self.b_b_bb & offset) == offset { rank = format!("{}{}", " B".bright_red(),  &prev); } 
            else if (self.b_q_bb & offset) == offset { rank = format!("{}{}", " Q".bright_red(),  &prev); } 
            else if (self.b_k_bb & offset) == offset { rank = format!("{}{}", " K".bright_red(),  &prev); } 
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
