use crate    ::{
    position :: Position, 
    core     ::{ Square, INITIAL_FEN } 
};

#[derive(Debug, Default)]
pub struct Game {
    pub history: Vec<Position>,
    pub ply: u16,
    pub mve: u16,
}

impl Game {
    pub fn new(fen: &str) -> Self {
        let mut game = Game::default();
        game.init(fen);
        game
    }

    fn init(&mut self, fen: &str) {
        self.from_fen(if fen.is_empty() { INITIAL_FEN } else { fen });
    }

    pub fn as_fen(&self) -> String {
        
        let mut fen = String::new();


        { // Piece Placement
        }
        { // Active Colour
            match self.current_position().blacks_move {
                true => fen.push_str("b "),
                false => fen.push_str("w "),
            }
        }
        { // Castling Rights
            
        }
        { // En Passant Target
            match self.current_position().en_passant_targ {
                Some(x) => {
                    let sq = Square::bb_to_str(x).expect("TODO");
                    fen.push_str(sq);
                    fen.push(' ');

                },
                None => {
                    fen.push_str("- ");
                }
            }
        }
        { // PLy Clock
            fen.push_str(&self.current_position().ply_clock.to_string());
            fen.push(' ');
        }
        { // Move Counter
            fen.push_str(&self.mve.to_string());
            fen.push(' ');
        }
        fen
    }

    pub fn from_fen(&mut self, fen: &str) {
        let fen_parts = fen.split(' ').collect::<Vec<_>>();

        if fen_parts.len() != 6 {
            panic!("Malformed FEN") // Return Error
        }

        let mut position = Position::new();

        let mut file = 0;
        let mut rank = 7;
        // Piece Placement
        for c in fen_parts[0].chars() {
            match c {
                '/' => {
                    file = 0;
                    rank -= 1;
                },
                '0' ..= '9' => {
                    file += c.to_digit(10).unwrap();
                },
                // Black Pieces
                'p' => { position.board.b_p_bb += 1 << (rank * 8 + file); file += 1; },
                'r' => { position.board.b_r_bb += 1 << (rank * 8 + file); file += 1; },
                'n' => { position.board.b_n_bb += 1 << (rank * 8 + file); file += 1; },
                'b' => { position.board.b_b_bb += 1 << (rank * 8 + file); file += 1; },
                'q' => { position.board.b_q_bb += 1 << (rank * 8 + file); file += 1; },
                'k' => { position.board.b_k_bb += 1 << (rank * 8 + file); file += 1; },
                // White Pieces
                'P' => { position.board.w_p_bb += 1 << (rank * 8 + file); file += 1; },
                'R' => { position.board.w_r_bb += 1 << (rank * 8 + file); file += 1; },
                'N' => { position.board.w_n_bb += 1 << (rank * 8 + file); file += 1; },
                'B' => { position.board.w_b_bb += 1 << (rank * 8 + file); file += 1; },
                'Q' => { position.board.w_q_bb += 1 << (rank * 8 + file); file += 1; },
                'K' => { position.board.w_k_bb += 1 << (rank * 8 + file); file += 1; },
                _ => panic!("Unknown character in FEN placement data")
            }
        }
        // Active Colour
        match fen_parts[1] {
            "w" => position.blacks_move = false,
            "b" => position.blacks_move = true,
            _ => panic!("Malformed FEN active colour")
        }
        // Castling Rights
        match fen_parts[2] {
            "-" => {},
            _ => {
                for c in fen_parts[2].chars() {
                    match c {
                        'K' => { position.castling += 1 << 7 },
                        'Q' => { position.castling += 1 << 6 },
                        'k' => { position.castling += 1 << 5 },
                        'q' => { position.castling += 1 << 4 },
                        _  => panic!("Unknown character in FEN castling rights")
                    }
                }
            }
        }
        // En Passant Target
        match Square::str_to_u8(fen_parts[3]) {
            Some(np) => position.en_passant_targ = Some(np),
            None if fen_parts[3] == "-" => position.en_passant_targ = None,
            None => panic!("Malformed En Passant Target. Can be either a square (i.e. 'A6') or a dash '-' denoting that there is no valid square."),
        }
        // Halfmove (ply) clock (used for 50 move rule)
        match fen_parts[4].parse() {
            Ok(num) => position.ply_clock = num,
            _ => panic!("Malformed FEN halfmove clock")
        }
        // Fullmove counter 
        match fen_parts[5].parse() {
            Ok(num) => self.mve = num,
            _ => panic!("Malformed FEN fullmoves")
        }
        // Set Ply
        self.ply = self.mve * 2;
        if !position.blacks_move {
            self.ply -= 1;
        }

        // TODO(James): evaluate checks and temporary castling restrictions

        self.history.push(position);
    }

    pub fn as_pgn(&self) -> String {
        todo!()
    }

    pub fn from_pgn(&mut self) {
        todo!()
    }

    pub fn current_position(&self) -> &Position {
        self.history.last().expect("Error: Game has no previous position. This is a bug!")
    }
}