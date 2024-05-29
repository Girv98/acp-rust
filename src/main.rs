mod board;
mod game;
mod position;
mod ply;
mod core;

// use crate::core::{repl, utils::pp_bb, Square};
use game::*;

fn main() {
    // let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    // let bb = game.last_position().board.occupied_bb();
    // println!("{}\n", pp_bb(bb));
    // game.print_board(ply::Colour::White);
    // println!();
    // game.print_board(ply::Colour::Black);
    // println!();

    // println!("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    // println!("{}", game.as_fen());

    // println!("{:?}", Square::from_bb(game.last_position().en_passant_targ.unwrap()));
    println!();
    let mut new_game = Game::new();
    new_game.play_two_player();
}
