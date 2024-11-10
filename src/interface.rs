use crate::logic::Logic;
use crate::piece::Piece;
//use std::io::{self, Write};
use std::process::Command;

/*
    println!("\x1b[41m Red Background \x1b[0m");
    println!("\x1b[42m Green Background \x1b[0m");
    println!("\x1b[43m Yellow Background \x1b[0m");
    println!("\x1b[44m Blue Background \x1b[0m");
    println!("\x1b[45m Magenta Background \x1b[0m");
    println!("\x1b[46m Cyan Background \x1b[0m");
    println!("\x1b[47m White Background \x1b[0m");
    println!("\x1b[40m Black Background \x1b[0m");
*/

const chess_pieces: [char; 12] = [
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

/*
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
 */

pub fn clear_terminal() {
    let _ = Command::new("clear").status(); // For Unix-based systems
                                            // For Windows, use:
                                            // let _ = Command::new("cmd").args(&["/C", "cls"]).status();
}

pub fn print_board_in_terminal(logic: &Logic) {
    clear_terminal();
    println!("\n\nMoves: {}", logic.moves);
    let mut count = 0;

    // Create a single iterator and reverse it conditionally
    let board_iter: Box<dyn Iterator<Item = &Piece>> = if logic.whites_turn {
        print_taken_pieces(true, logic);
        println!("\n");
        Box::new(logic.board.iter().rev()) // Reverse for White's turn
    } else {
        print_taken_pieces(false, logic);
        println!("\n");
        Box::new(logic.board.iter()) // Normal order for Black's turn
    };

    for piece in board_iter {
        if count % 8 == 0 {
            println!("");
        }

        match piece {
            Piece::Piece { piece_type: 1, .. } => print_piece_prep(*piece, 0, logic),
            Piece::Piece { piece_type: 2, .. } => print_piece_prep(*piece, 1, logic),
            Piece::Piece { piece_type: 3, .. } => print_piece_prep(*piece, 2, logic),
            Piece::Piece { piece_type: 4, .. } => print_piece_prep(*piece, 3, logic),
            Piece::Piece { piece_type: 5, .. } => print_piece_prep(*piece, 4, logic),
            Piece::Piece { piece_type: 6, .. } => print_piece_prep(*piece, 5, logic),
            Piece::Empty { position: _ } => print_piece_prep(*piece, 6, logic),
            _ => {
                println!("error")
            }
        }
        count += 1;
    }
    //print_piece(position: usize, piece_index: usize, logic: &Logic)

    if logic.whites_turn {
        println!("");
        print_taken_pieces(false, logic);
    } else {
        println!("");
        print_taken_pieces(true, logic);
    }
}

pub fn print_piece_prep(piece: Piece, type_of_piece: usize, logic: &Logic) {
    match piece {
        Piece::Piece {
            white, position, ..
        } if type_of_piece == 0 => {
            if white {
                print_piece(position, 5, logic);
            } else {
                print_piece(position, 11, logic);
            }
        }
        Piece::Piece {
            white, position, ..
        } if type_of_piece == 1 || type_of_piece == 1 => {
            if white {
                print_piece(position, 2, logic);
            } else {
                print_piece(position, 8, logic);
            }
        }
        Piece::Piece {
            white, position, ..
        } if type_of_piece == 2 => {
            if white {
                print_piece(position, 4, logic);
            } else {
                print_piece(position, 10, logic);
            }
        }
        Piece::Piece {
            white, position, ..
        } if type_of_piece == 3 => {
            if white {
                print_piece(position, 3, logic);
            } else {
                print_piece(position, 9, logic);
            }
        }
        Piece::Piece {
            white, position, ..
        } if type_of_piece == 4 => {
            if white {
                print_piece(position, 1, logic);
            } else {
                print_piece(position, 7, logic);
            }
        }
        Piece::Piece {
            white, position, ..
        } if type_of_piece == 5 => {
            if white {
                print_piece(position, 0, logic);
            } else {
                print_piece(position, 6, logic);
            }
        }
        Piece::Empty { position, .. } if type_of_piece == 6 => {
            if position == logic.current_index {
                print!("\x1b[41m _ \x1b[0m");
            } else if logic.valid_moves.contains(&position) {
                print!("\x1b[43m _ \x1b[0m");
            } else {
                print!(" _ ");
            }
        }
        _ => {
            println!("Unknown piece type or type_of_piece does not match any variant.");
        }
    }
}

fn print_piece(position: usize, piece_index: usize, logic: &Logic) {
    //println!("from interface valid moves: {:?}", logic.valid_moves);
    if position == logic.current_index
        && logic.has_selected
        && logic.current_index != logic.selected_index
    {
        print!("\x1b[42m {} \x1b[0m", chess_pieces[piece_index]);
    } else if position == logic.selected_index {
        print!("\x1b[44m {} \x1b[0m", chess_pieces[piece_index])
    } else if position == logic.current_index {
        print!("\x1b[41m {} \x1b[0m", chess_pieces[piece_index]);
    } else if ((position == logic.white_king_position) && logic.white_in_check)
        || ((position == logic.black_king_position) && logic.black_in_check)
    {
        print!("\x1b[45m {} \x1b[0m", chess_pieces[piece_index]);
    } else if logic.valid_moves.contains(&position) {
        print!("\x1b[43m {} \x1b[0m", chess_pieces[piece_index]);
    } else {
        print!(" {} ", chess_pieces[piece_index]);
    }
}

fn print_taken_pieces(white_pieces: bool, logic: &Logic) {
    if white_pieces {
        println!("\n\n{} Taken white piece(s):", logic.taken_white_pieces_num);
        for piece_type in logic.taken_white_pieces.iter() {
            match piece_type {
                1 => print!("{} ", chess_pieces[5]), //pawn
                2 => print!("{} ", chess_pieces[2]), //rook
                3 => print!("{} ", chess_pieces[4]), //knight
                4 => print!("{} ", chess_pieces[3]), //bishop
                5 => print!("{} ", chess_pieces[1]), //queen
                6 => print!("{} ", chess_pieces[0]), //king
                _ => {
                    println!("error")
                }
            }
        }
    } else {
        println!("\n\n{} Taken black piece(s):", logic.taken_black_pieces_num);
        for piece_type in logic.taken_black_pieces.iter() {
            match piece_type {
                1 => print!("{} ", chess_pieces[11]),
                2 => print!("{} ", chess_pieces[8]),
                3 => print!("{} ", chess_pieces[10]),
                4 => print!("{} ", chess_pieces[9]),
                5 => print!("{} ", chess_pieces[7]),
                6 => print!("{} ", chess_pieces[6]),
                _ => {
                    println!("error")
                }
            }
        }
    }
}

pub fn print_promotion_choices(white: bool) {
    println!("Please choose piece to promote to.");
    if white {
        //queen             knight             rook             bishop
        println!(
            "1: {}, 2: {}, 3: {}, 4: {},",
            chess_pieces[1], chess_pieces[4], chess_pieces[2], chess_pieces[3]
        );
    } else {
        println!(
            "1: {}, 2: {}, 3: {}, 4: {},",
            chess_pieces[7], chess_pieces[10], chess_pieces[8], chess_pieces[9]
        );
    }
}
