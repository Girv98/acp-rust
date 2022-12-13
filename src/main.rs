mod board;
mod constants;
mod game;
mod position;
mod mov;

use std::collections::HashMap;

use board::*;
use constants::*;
use game::*;
use position::*;

fn main() {
    let mut game = Game::new();

    game.init("".to_string());


    let sq = (1u64 << 1) | (1u64 << 6) | (1u64 << 57) | (1u64 << 62);
    let sq = constants::DARK_SQS;
    // pretty_print_bb(get_all_knight_moves_bb(sq));
    //println!();
    pp_bb(knight_moves_bb(sq));
    pp_bb(sq);
    println!();
    
    let bb = game.position.board.get_all_blk_pawn_attacks();
    
    pp_bb(bb);
    println!();
}
