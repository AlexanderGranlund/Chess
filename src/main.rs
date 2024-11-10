use input::{get_input, match_input};

pub use crate::interface::print_board_in_terminal;
pub use crate::logic::Logic;
pub use crate::piece::Piece;

mod input;
mod interface;
mod logic;
mod piece;

fn main() {
    let mut logic = Logic::new();

    loop {
        print_board_in_terminal(&logic);
        if logic.game_state > 0 {
            match logic.game_state {
                1 => {
                    println!("\n###        Game Over        ###\n### Black Won by Checkmate  ###");
                }
                2 => {
                    println!("\n###        Game Over        ###\n###    Draw by Stalemate    ###");
                }
                3 => {
                    println!("\n###        Game Over        ###\n### White Won by Check Mate ###");
                }
                4 => {
                    println!("\n###        Game Over        ###\n###    Draw by Stalemate   ###");
                }
                _ => {}
            }
            let input_vec: Vec<String> = get_input();
            for input in input_vec {
                logic.do_action(match_input(input));
            }
        } else {
            let input_vec: Vec<String> = get_input();
            for input in input_vec {
                logic.do_action(match_input(input));
            }
        }
    }
}
