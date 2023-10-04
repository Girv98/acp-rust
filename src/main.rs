mod board;
mod game;
mod position;
mod ply;
mod core;

// use board::*;
use crate::core::utils::pp_bb;
use game::*;
// use position::*;

fn main() {
    let game = Game::new("".to_string());
    let bb = game.current_position().board.occupied_bb();
    println!("{}\n", pp_bb(bb));
    game.current_position().board.print_board();
    println!();
}
