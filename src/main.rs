mod board;
mod game;
mod position;
mod ply;
mod core;

use core::Square;

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
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 1 2";
    let mut new_game = Game::try_from_fen(fen).unwrap();

    // let square = Square::from_str("d4").unwrap();

    // println!("{}", new_game.last_position().board.get_possible_moves_board(true, square));

    // let mut new_game = Game::new();
    new_game.play_two_player();
}
