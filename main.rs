
//use input::get_input;

use input::{get_input, match_input};
use std::io::{self, Write};

pub use crate::piece::Piece;
pub use crate::logic::Logic;
pub use crate::interface::print_board_in_terminal;

mod piece;
mod logic;
mod interface;
mod input;


fn main(){
    while true {
        std::thread::sleep(std::time::Duration::from_secs(2));
        //clear_terminal(); 
        let mut logic = Logic::new();
        print_board_in_terminal(logic.board, true);
        //Logic::view_core_moves(logic.core_moves);
        match_input(get_input(), logic.has_selected); 
        
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}


/*
change background color:

    use std::io::{self, Write};
    se termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    fn main() -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        // Set background to red and write text
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Red)))?;
        write!(&mut stdout, "This is red background")?;

        // Reset to default color
        stdout.reset()?;
        writeln!(&mut stdout, " Back to normal")?;

        // Set background to green and write text
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::White)))?;
        write!(&mut stdout, "This is green background")?;

        // Reset to default color
        stdout.reset()?;
        writeln!(&mut stdout, " Back to normal")?;

        Ok(())
    }
    
 */