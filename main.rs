
pub use crate::piece::Piece;
pub use crate::logic::Logic;
pub use crate::interface::print_board_in_terminal;
mod piece;
mod logic;
mod interface;


fn main(){
    let mut logic = Logic::new();
    print_board_in_terminal(logic.board);

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