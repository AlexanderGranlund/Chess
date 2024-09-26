use crate::clear_terminal;
use crate::piece::Piece; 
use crate::logic::Logic;


/*
// Set background to red and write text
        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Red)))?;
        write!(&mut stdout, "This is red background")?;

        // Reset to default color
        stdout.reset()?;
        writeln!(&mut stdout, " Back to normal")?;
*/


pub fn print_board_in_terminal(logic: &Logic){
    clear_terminal();
    let chess_pieces = [
    '\u{2654}', // White King    0
    '\u{2655}', // White Queen   1
    '\u{2656}', // White Rook    2
    '\u{2657}', // White Bishop  3
    '\u{2658}', // White Knight  4
    '\u{2659}', // White Pawn    5
    '\u{265A}', // Black King    6
    '\u{265B}', // Black Queen   7
    '\u{265C}', // Black Rook    8
    '\u{265D}', // Black Bishop  9
    '\u{265E}', // Black Knight  10
    '\u{265F}', // Black Pawn    11
];
    let mut count = 0;
    if logic.whites_turn{
        for piece in logic.board.iter().rev(){
            if count % 8 == 0 {
                println!("");
            }
            match piece {
                Piece::Piece { piece_type: 1, .. } => print_piece(*piece, 0, chess_pieces, logic.current_index),
                Piece::Piece { piece_type: 2, .. } => print_piece(*piece, 1, chess_pieces, logic.current_index),
                Piece::Piece { piece_type: 3, .. } => print_piece(*piece, 2, chess_pieces, logic.current_index),
                Piece::Piece { piece_type: 4, .. } => print_piece(*piece, 3, chess_pieces, logic.current_index),
                Piece::Piece { piece_type: 5, .. } => print_piece(*piece, 4, chess_pieces, logic.current_index),
                Piece::Piece { piece_type: 6, .. } => print_piece(*piece, 5, chess_pieces, logic.current_index),       
                Piece::Empty{position:_} => print_piece(*piece, 6, chess_pieces, logic.current_index),
                _ => { println!("error")}
            }
            count += 1;
        }
    }   else{
            for piece in logic.board.iter(){
                if count % 8 == 0 {
                    println!("");
                }
                match piece {
                    Piece::Piece {piece_type: 1, .. } => print_piece(*piece, 0, chess_pieces, logic.current_index),
                    Piece::Piece {piece_type: 2, .. } => print_piece(*piece, 1, chess_pieces, logic.current_index),
                    Piece::Piece {piece_type: 3, .. } => print_piece(*piece, 2, chess_pieces, logic.current_index),
                    Piece::Piece {piece_type: 4, .. } => print_piece(*piece, 3, chess_pieces, logic.current_index),
                    Piece::Piece {piece_type: 5, .. } => print_piece(*piece, 4, chess_pieces, logic.current_index),
                    Piece::Piece {piece_type: 6, .. } => print_piece(*piece, 5, chess_pieces, logic.current_index),
                    Piece::Empty{position:_} => print_piece(*piece, 6, chess_pieces, logic.current_index),
                    _ => { println!("error")}
                }
                count += 1;
            }
    }
}


pub fn print_piece(piece: Piece, type_of_piece: usize, chess_pieces: [char; 12], current_index: usize) {
    match piece {
        Piece::Piece { white, position,..} if type_of_piece == 0 => {
            if white {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[5]);
                }
                else {
                    print!(" {} ", chess_pieces[5]);
                }
                

            } else {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[11]);
                }
                else {
                    print!(" {} ", chess_pieces[11]);
                }
               
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 1 || type_of_piece == 5 => {
            if white {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[2]);
                }
                else {
                    print!(" {} ", chess_pieces[2]);
                }
                
            } else {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[8]);
                }
                else {
                    print!(" {} ", chess_pieces[8]);
                }
                
            }
        }
        Piece::Piece { white, position,.. } if type_of_piece == 2 => {
            if white {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[4]);
                }
                else {
                    print!(" {} ", chess_pieces[4]);
                }
               
            } else {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[10]);
                }
                else {
                    print!(" {} ", chess_pieces[10]);
                }
            
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 3 => {
            if white {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[3]);
                }
                else {
                    print!(" {} ", chess_pieces[3]);
                }
             
            } else {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[9]);
                }
                else {
                    print!(" {} ", chess_pieces[9]);
                }
                
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 4 => {
            if white {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[1]);
                }
                else {
                    print!(" {} ", chess_pieces[1]);
                }
            
            } else {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[7]);
                }
                else {
                    print!(" {} ", chess_pieces[7]);
                }
               
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 5 => {
            if white {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[0]);
                }
                else {
                    print!(" {} ", chess_pieces[0]);
                }
            
            } else {
                if position == current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[6]);
                }
                else {
                    print!(" {} ", chess_pieces[6]);
                }
                
            }
        }
        Piece::Empty {position ,..} if type_of_piece == 6 => {
            
                if position == current_index{
                    print!("\x1b[41m _ \x1b[0m");
                }
                else {
                    print!(" _ ");
                }
            
            
        }
        _ => {
            println!("Unknown piece type or type_of_piece does not match any variant.");
        }
    }
}