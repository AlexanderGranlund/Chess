
//use input::get_input;

use input::{get_input, match_input};

pub use crate::piece::Piece;
pub use crate::logic::Logic;
pub use crate::interface::print_board_in_terminal;

mod piece;
mod logic;
mod interface;
mod input;


fn main(){
    let mut logic = Logic::new();
    while true {
        //std::thread::sleep(std::time::Duration::from_secs(2));
        
        
        print_board_in_terminal(&logic);
        print!("\n\nmoves: {}", logic.moves);
        //Logic::view_core_moves(&logic.core_moves);
        //println!("index enter: {}", logic.current_index);
        let input_vec: Vec<String> = get_input();
        for input in input_vec{
            logic.do_action(match_input(input));
        }
        //println!("index enter: {}", logic.current_index);
        //do_action(&mut self, piece: Piece,  action: usize)
    }
}




