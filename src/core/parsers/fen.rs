use crate    :: {
    core     :: Square, 
    game     :: Game, 
    position :: Position 
};


pub fn parse_fen(fen: &str) -> Result<Game, String> {
    let mut game: Game = Game::default();
    let mut position = Position::new();

    let fen_parts = fen.split(' ').collect::<Vec<_>>(); // FIXME: We are doing this twice with Position::from_fen

    if fen_parts.len() != 6 {
        return Err("Malformed FEN".to_string()) // TODO: Return proper error
    }

    let mut file = 0;
    let mut rank = 7;
    // Piece Placement
    for c in fen_parts[0].chars() {
        match c {
            '/' => { file = 0; rank -= 1; },
            '0' ..= '9' => file += c.to_digit(10).unwrap(),
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
            _ => return Err("Unknown character in FEN placement data".to_string())
        }
    }
    // Active Colour
    match fen_parts[1] {
        // With this, the starting position will have technically be black's move
        // But it shouldn't affect game logic
        "w" => position.was_blacks_move = true,
        "b" => position.was_blacks_move = false,
        _ => return Err("Malformed FEN active colour".to_string())
    }
    // Castling Rights
    match fen_parts[2] {
        "-" => {},
        _ => {
            for c in fen_parts[2].chars() {
                match c {
                    'K' => { position.castling |= 1 << 7 },
                    'Q' => { position.castling |= 1 << 6 },
                    'k' => { position.castling |= 1 << 5 },
                    'q' => { position.castling |= 1 << 4 },
                    _  => return Err ("Unknown character in FEN castling rights".to_string())
                }
            }
        }
    }
    // En Passant Target
    match Square::str_to_bb(fen_parts[3]) {
        Some(np) => position.en_passant_targ = Some(np),
        None if fen_parts[3] == "-" => position.en_passant_targ = None,
        None => return Err ("Malformed En Passant Target. Can be either a square (i.e. 'A6') or a dash '-' denoting that there is no valid square.".to_string()),
    }
    // Halfmove (ply) clock (used for 50 move rule)
    match fen_parts[4].parse() {
        Ok(num) => position.ply_clock = num,
        _ => return Err ("Malformed FEN halfmove clock".to_string())
    }
    // Fullmove counter 
    match fen_parts[5].parse() {
        Ok(num) => game.mov = num,
        _ => return Err("Malformed FEN fullmoves".to_string())
    }
    // Set Ply
    game.ply = game.mov * 2;
    if position.was_blacks_move {
        game.ply -= 1;
    }

    game.history.push(position);
    Ok(game)
}