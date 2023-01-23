mod board;
mod constants;
mod game;
mod position;
mod mov;



// use board::*;
use constants::*;
use game::*;
// use position::*;

fn main() {
    let game = Game::new("".to_string());

    let _sq = (1u64 << 1) | (1u64 << 6) | (1u64 << 57) | (1u64 << 62);
    let sq = constants::DARK_SQS;
    // pretty_print_bb(get_all_knight_moves_bb(sq));
    //println!();
    pp_bb(knight_moves_bb(sq));
    pp_bb(sq);
    
    let bb = game.current_position().board.get_all_occupied_bb();
    
    pp_bb(bb);

    game.current_position().board.print_board()
}
