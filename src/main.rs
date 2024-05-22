mod board;
mod game;
mod position;
mod ply;
mod core;

// use board::*;
use crate::core::{utils::pp_bb, Square};
use game::*;
// use position::*;

fn main() {
    let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    let bb = game.current_position().board.occupied_bb();
    println!("{}\n", pp_bb(bb));

    game.current_position().board.print_board(true);
    println!();
    game.current_position().board.print_board(false);
    println!();

    println!("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    println!("{}", game.as_fen());

    println!("{:?}", Square::from_bb(game.current_position().en_passant_targ.unwrap()));
}
