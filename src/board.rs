
use colored::Colorize;

use crate::core::Square;
use crate::core;
use crate::ply::{Colour, Piece};

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
        // Gets a mask of all the squares that black's pieces can capture 
        // and then sees if the given square is in that mask
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
        let _king = if is_white { self.w_k_bb } else { self.b_k_bb };
        let _square = sq.to_bb();
        todo!()
    } 

    fn piece_at(&self, offset: u64) -> Option<(Piece, Colour)> {
        use Piece::*;
        use Colour::*;
             if (self.w_p_bb & offset) == offset { Some((Pawn,   White)) }
        else if (self.w_r_bb & offset) == offset { Some((Rook,   White)) }
        else if (self.w_n_bb & offset) == offset { Some((Knight, White)) }
        else if (self.w_b_bb & offset) == offset { Some((Bishop, White)) }
        else if (self.w_q_bb & offset) == offset { Some((Queen,  White)) }
        else if (self.w_k_bb & offset) == offset { Some((King,   White)) }
        // Black Pieces
        else if (self.b_p_bb & offset) == offset { Some((Pawn,   Black)) }
        else if (self.b_r_bb & offset) == offset { Some((Rook,   Black)) }
        else if (self.b_n_bb & offset) == offset { Some((Knight, Black)) }
        else if (self.b_b_bb & offset) == offset { Some((Bishop, Black)) }
        else if (self.b_q_bb & offset) == offset { Some((Queen,  Black)) }
        else if (self.b_k_bb & offset) == offset { Some((King,   Black)) }
        else { None }
    }

    pub fn stringify(&self) -> String {
        let mut str = String::new();

        let mut rank = String::new();

        let mut counter = 0u8;
        for i in 0..64 {
            let offset = 1 << (63-i);
            let piece = match self.piece_at(offset) {
                Some(piece) => match piece {
                    // White Pieces
                    (Piece::Pawn,   Colour::White) => 'P',
                    (Piece::Rook,   Colour::White) => 'R',
                    (Piece::Knight, Colour::White) => 'N',
                    (Piece::Bishop, Colour::White) => 'B',
                    (Piece::Queen,  Colour::White) => 'Q',
                    (Piece::King,   Colour::White) => 'K',
                    // Black Pieces
                    (Piece::Pawn,   Colour::Black) => 'p',
                    (Piece::Rook,   Colour::Black) => 'r',
                    (Piece::Knight, Colour::Black) => 'n',
                    (Piece::Bishop, Colour::Black) => 'b',
                    (Piece::Queen,  Colour::Black) => 'q',
                    (Piece::King,   Colour::Black) => 'k',
                },
                None => '\0',
            };

            if piece == '\0' {
                counter += 1;
                debug_assert!(counter < 9 );
            } else {
                if counter > 0 {
                    let c = (48 + counter) as char;
                    rank.push(c);
                    counter = 0;
                }
                rank.push(piece);
            }

            if i % 8 == 7 {
                if counter > 0 {
                    let c = (48 + counter) as char;
                    rank.push(c);
                    counter = 0;
                }
                rank.chars().rev().for_each(|c| str.push(c));
                str.push('/');
                rank = String::new();
            }

        }

        str.pop();
        str
    }

    pub fn print_board(&self, white_pov: bool) {
        let mut rank = String::new();
        let mut letters = if white_pov {vec!['8', '7', '6', '5', '4', '3', '2', '1']} else {vec!['1', '2', '3', '4', '5', '6', '7', '8']}.into_iter();
        
        for i in 0..64 {
            let offset = if white_pov { 1 << (63-i) } else { 1 << i };
            let prev = rank;
            // NOTE: i would have liked to use ♙♖♘♗♕♔ ♟︎♞♝♜♛♚ but they don't seem to line up properly with certain fonts/terminals 🙃
            rank = match self.piece_at(offset) {
                Some(piece) => match piece {
                    // White Pieces
                    (Piece::Pawn,   Colour::White) => format!("{}{}", " P".bright_blue(), &prev),
                    (Piece::Rook,   Colour::White) => format!("{}{}", " R".bright_blue(), &prev),
                    (Piece::Knight, Colour::White) => format!("{}{}", " N".bright_blue(), &prev),
                    (Piece::Bishop, Colour::White) => format!("{}{}", " B".bright_blue(), &prev),
                    (Piece::Queen,  Colour::White) => format!("{}{}", " Q".bright_blue(), &prev),
                    (Piece::King,   Colour::White) => format!("{}{}", " K".bright_blue(), &prev),
                    // Black Pieces
                    (Piece::Pawn,   Colour::Black) => format!("{}{}", " P".bright_red(),  &prev),
                    (Piece::Rook,   Colour::Black) => format!("{}{}", " R".bright_red(),  &prev),
                    (Piece::Knight, Colour::Black) => format!("{}{}", " N".bright_red(),  &prev),
                    (Piece::Bishop, Colour::Black) => format!("{}{}", " B".bright_red(),  &prev),
                    (Piece::Queen,  Colour::Black) => format!("{}{}", " Q".bright_red(),  &prev),
                    (Piece::King,   Colour::Black) => format!("{}{}", " K".bright_red(),  &prev),
                },
                None => format!("{}{}", " ·", &prev),
            };
            if i % 8 == 7 {
                print!("{}", letters.next().unwrap());
                println!("{}", rank.as_str());
                rank = "".to_string();
            }
        }
        if white_pov {
            println!("~ A B C D E F G H");
        } else {
            println!("~ H G F E D C B A");
        }

    }    
}
