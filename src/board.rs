
use colored::Colorize;

use crate::core::Square;
use crate::core;
use crate::ply::{Colour, Piece, Ply};

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
    pub fn unoccupied_bb(&self) -> u64 {
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

    /// Gets bitboard of all push squares of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_pawn_single_pushes(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.pawn_single_push(sq, colour)
    }
    /// Gets bitboard of all double push squares of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_pawn_double_pushes(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.pawn_double_push(sq, colour)
    }
    /// Gets bitboard of all occupied squares that are under threat from a given colour's pawns
    pub fn all_pawn_attacks(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.pawn_attacks(sq, colour)
    }
    pub fn pawn_single_push(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::pawn_single_pushes_bb(sq, self.unoccupied_bb(), true),
            Colour::Black => core::pawn_single_pushes_bb(sq, self.unoccupied_bb(), false),
        }
    }
    pub fn pawn_double_push(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::pawn_double_pushes_bb(sq, self.unoccupied_bb(), true),
            Colour::Black => core::pawn_double_pushes_bb(sq, self.unoccupied_bb(), false),
        }
    }
    pub fn pawn_attacks(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::pawn_attacks_bb(sq, true) & self.black_bb(),
            Colour::Black => core::pawn_attacks_bb(sq, false) & self.white_bb(),
        }
    }
    
    /// Gets bitboard of all possible knight moves of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_knight_moves(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.knight_moves(sq)
    }
    /// Gets bitboard of all occupied squares that are under threat from a given colour's knights
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_knight_attacks(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.knight_attacks(sq, colour)
    }
    pub fn knight_moves(&self, sq: u64) -> u64 {
        core::knight_moves_bb(sq) & self.unoccupied_bb()
    }
    pub fn knight_attacks(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::knight_moves_bb(sq) & self.black_bb(),
            Colour::Black => core::knight_moves_bb(sq) & self.white_bb(),
        }
    }

    /// Gets bitboard of all possible king moves of a given colour
    /// NOTE: Doesn't account for illegal moves
    pub fn all_king_moves(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.king_moves(sq)
    }
    /// Gets bitboard of all occupied squares that are under threat from a given colour's king
    /// NOTE: Doesn't account for illegal moves 
    pub fn all_king_attacks(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.king_attacks(sq, colour)
    }
    pub fn king_moves(&self, sq: u64) -> u64 {
       core::king_moves_bb(sq) & self.unoccupied_bb()
    }
    pub fn king_attacks(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::king_moves_bb(sq) & self.black_bb(),
            Colour::Black => core::king_moves_bb(sq) & self.white_bb(),
        }
    }

    /// Gets bitboard of all possible bishop moves of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_bishop_moves(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.bishop_moves(sq)
    }
    /// Gets bitboard of all possible bishop attacks of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_bishop_attacks(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.bishop_attacks(sq, colour)
    }
    pub fn bishop_moves(&self, sq: u64) -> u64 {
        core::bish_moves_bb(sq, self.unoccupied_bb()) & self.unoccupied_bb()  
    }
    pub fn bishop_attacks(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::bish_moves_bb(sq, self.unoccupied_bb()) & self.black_bb(),
            Colour::Black => core::bish_moves_bb(sq, self.unoccupied_bb()) & self.white_bb(),
        }
    }

    /// Gets bitboard of all possible rook moves of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_rook_moves(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.rook_moves(sq)

    }
    /// Gets bitboard of all possible rook attacks of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_rook_attacks(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.rook_attacks(sq, colour)
    }
    pub fn rook_moves(&self, sq: u64) -> u64 {
        core::rook_moves_bb(sq, self.unoccupied_bb()) & self.unoccupied_bb()
    }
    pub fn rook_attacks(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::rook_moves_bb(sq, self.unoccupied_bb()) & self.black_bb(),
            Colour::Black => core::rook_moves_bb(sq, self.unoccupied_bb()) & self.white_bb(),
        }
    }

    /// Gets bitboard of all queen moves of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_queen_moves(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.queen_moves(sq)
    }
    /// Gets bitboard of all possible queen attacks of a given colour
    /// NOTE: Doesn't account for illegal moves such as when the piece is pinned
    pub fn all_queen_attacks(&self, colour: Colour) -> u64 {
        let sq = match colour {
            Colour::White => self.w_q_bb,
            Colour::Black => self.b_q_bb,
        };
        self.queen_attacks(sq, colour)
    }
    pub fn queen_moves(&self, sq: u64) -> u64 {
          core::rook_moves_bb(sq, self.unoccupied_bb())
        | core::bish_moves_bb(sq, self.unoccupied_bb()) 
        & self.unoccupied_bb()
    }
    pub fn queen_attacks(&self, sq: u64, colour: Colour) -> u64 {
        match colour {
            Colour::White => core::rook_moves_bb(sq, self.unoccupied_bb())
                           | core::bish_moves_bb(sq, self.unoccupied_bb()) 
                           & self.black_bb(),
            Colour::Black => core::rook_moves_bb(sq, self.unoccupied_bb())
                           | core::bish_moves_bb(sq, self.unoccupied_bb()) 
                           & self.white_bb(),
        }
    }

    /// Gets a bitboard of all squares that are threatened by white
    /// NOTE: Doesn't include en-passant target square
    pub fn all_white_attacks(&self) -> u64 {
          self.all_pawn_attacks(Colour::White) 
        | self.all_knight_attacks(Colour::White)
        | self.all_king_attacks(Colour::White)
        | self.all_bishop_attacks(Colour::White)
        | self.all_rook_attacks(Colour::White)
        | self.all_queen_attacks(Colour::White)
    }
    /// Gets a bitboard of all squares that are threatened by black
    /// NOTE: Doesn't include en-passant target square
    pub fn all_black_attacks(&self) -> u64 {
          self.all_pawn_attacks(Colour::Black) 
        | self.all_knight_attacks(Colour::Black)
        | self.all_king_attacks(Colour::Black)
        | self.all_bishop_attacks(Colour::Black)
        | self.all_rook_attacks(Colour::Black)
        | self.all_queen_attacks(Colour::Black)
    }
    /// Sees if a given (black-occupied) square is under attack by any white piece
    pub fn piece_is_attacked_by_white(&self, sq: Square) -> bool {
        let sq = sq.to_bb();
        (self.all_white_attacks() & sq) == sq
    }
    /// Sees if a given (white-occupied) square is under attack by any black piece
    pub fn piece_is_attacked_by_black(&self, sq: Square) -> bool {
        // Gets a mask of all the squares that black's pieces can capture 
        // and then sees if the given square is in that mask
        let sq = sq.to_bb();
        (self.all_black_attacks() & sq) == sq
    }
    /// Checks if white's king is in check. Panics if white somehow has more than one king
    pub fn white_king_is_in_check(&self) -> bool {
        debug_assert_eq!(self.w_k_bb.count_ones(), 1);
        self.piece_is_attacked_by_black(Square::from_bb(self.w_k_bb))
    } 
    /// Checks if black's king is in check. Panics if black somehow has more than one king
    pub fn black_king_is_in_check(&self) -> bool {
        debug_assert_eq!(self.b_k_bb.count_ones(), 1);
        self.piece_is_attacked_by_white(Square::from_bb(self.b_k_bb))
    } 

    pub fn piece_is_pinned_to_king(&self, sq: Square, is_white: bool) -> bool {
        let _king = if is_white { self.w_k_bb } else { self.b_k_bb };
        let _square = sq.to_bb();
        todo!()
    } 

    pub fn piece_at(&self, offset: u64) -> Option<(Piece, Colour)> {
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
                println!("{}\r", rank.as_str());
                rank = "".to_string();
            }
        }
        if white_pov {
            println!("~ A B C D E F G H\r")
        } else {
            println!("~ H G F E D C B A\r")
        }

    }    

    pub fn get_board(&self, white_pov: bool) -> String {
        let mut board = String::new();

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
                board.push(letters.next().unwrap());
                board.push_str(rank.as_str());
                board.push_str("\r\n");
                rank = "".to_string();
            }
        }
        if white_pov {
            board.push_str("~ A B C D E F G H\r");
        } else {
            board.push_str("~ H G F E D C B A\r");
        }

        board
    }

    pub fn get_move_board(&self, white_pov: bool, ply: Ply) -> String {
        todo!()
    } 

    pub fn get_possible_moves_board(&self, white_pov: bool, sq: Square) -> String {
        // FIXME: Doesn't (currently) filter illegal moves or pinned pieces
        // NOTE: Assumes sq is valid

        let sq = sq.as_bb();
        
        let (attacks, moves) = match self.piece_at(sq) {
            Some((p, c)) => match p {
                Piece::Pawn =>(self.pawn_attacks(sq, c), self.pawn_single_push(sq, c) | self.pawn_double_push(sq, c)),
                Piece::Knight => (self.knight_attacks(sq, c), self.knight_moves(sq)),
                Piece::Bishop => (self.bishop_attacks(sq, c), self.bishop_moves(sq)),
                Piece::Rook   => (self.rook_attacks(sq, c),   self.rook_moves(sq)),
                Piece::Queen  => (self.queen_attacks(sq, c),  self.queen_moves(sq)),
                Piece::King   => (self.king_attacks(sq, c),   self.king_moves(sq)),
            },
            None => (0, 0),
        };
        
        let mut board = String::new();

        let mut rank = String::new();
        let mut letters = if white_pov {vec!['8', '7', '6', '5', '4', '3', '2', '1']} else {vec!['1', '2', '3', '4', '5', '6', '7', '8']}.into_iter();
        
        for i in 0..64 {
            let offset = if white_pov { 1 << (63-i) } else { 1 << i };
            let prev = rank;
            // NOTE: i would have liked to use ♙♖♘♗♕♔ ♟︎♞♝♜♛♚ but they don't seem to line up properly with certain fonts/terminals 🙃
            rank = match self.piece_at(offset) {
                Some(piece) => match piece {
                    // White Pieces
                    (Piece::Pawn,   Colour::White) => format!("{}{}", if offset & attacks != 0 {" P".black().on_bright_red()} else {" P".bright_blue()}, &prev),
                    (Piece::Rook,   Colour::White) => format!("{}{}", if offset & attacks != 0 {" R".black().on_bright_red()} else {" R".bright_blue()}, &prev),
                    (Piece::Knight, Colour::White) => format!("{}{}", if offset & attacks != 0 {" N".black().on_bright_red()} else {" N".bright_blue()}, &prev),
                    (Piece::Bishop, Colour::White) => format!("{}{}", if offset & attacks != 0 {" B".black().on_bright_red()} else {" B".bright_blue()}, &prev),
                    (Piece::Queen,  Colour::White) => format!("{}{}", if offset & attacks != 0 {" Q".black().on_bright_red()} else {" Q".bright_blue()}, &prev),
                    (Piece::King,   Colour::White) => format!("{}{}", if offset & attacks != 0 {" K".black().on_bright_red()} else {" K".bright_blue()}, &prev),
                    // Black Pieces
                    (Piece::Pawn,   Colour::Black) => format!("{}{}", if offset & attacks != 0 {" P".black().on_bright_blue()} else {" P".bright_red()},  &prev),
                    (Piece::Rook,   Colour::Black) => format!("{}{}", if offset & attacks != 0 {" R".black().on_bright_blue()} else {" R".bright_red()},  &prev),
                    (Piece::Knight, Colour::Black) => format!("{}{}", if offset & attacks != 0 {" N".black().on_bright_blue()} else {" N".bright_red()},  &prev),
                    (Piece::Bishop, Colour::Black) => format!("{}{}", if offset & attacks != 0 {" B".black().on_bright_blue()} else {" B".bright_red()},  &prev),
                    (Piece::Queen,  Colour::Black) => format!("{}{}", if offset & attacks != 0 {" Q".black().on_bright_blue()} else {" Q".bright_red()},  &prev),
                    (Piece::King,   Colour::Black) => format!("{}{}", if offset & attacks != 0 {" K".black().on_bright_blue()} else {" K".bright_red()},  &prev),
                },
                None => if offset & moves != 0  {
                    format!("{}{}", " ·".on_green().black(), &prev)
                } else {
                    format!("{}{}", " ·", &prev)
                },
            };
            if i % 8 == 7 {
                board.push(letters.next().unwrap());
                board.push_str(rank.as_str());
                board.push_str("\r\n");
                rank = "".to_string();
            }
        }
        if white_pov {
            board.push_str("~ A B C D E F G H\r");
        } else {
            board.push_str("~ H G F E D C B A\r");
        }

        board

    }

}
