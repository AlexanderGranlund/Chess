use crate::piece::Piece; 


pub fn print_board_in_terminal(board: [Piece;64], rev: bool){
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
    if rev{
        for piece in board.iter().rev(){
            if count % 8 == 0 {
                println!("");
            }
            match piece {
                Piece::Piece { piece_type: 1, .. } => print_piece(*piece, 0, chess_pieces),
                Piece::Piece { piece_type: 2, .. } => print_piece(*piece, 1, chess_pieces),
                Piece::Piece { piece_type: 3, .. } => print_piece(*piece, 2, chess_pieces),
                Piece::Piece { piece_type: 4, .. } => print_piece(*piece, 3, chess_pieces),
                Piece::Piece { piece_type: 5, .. } => print_piece(*piece, 4, chess_pieces),
                Piece::Piece { piece_type: 6, .. } => print_piece(*piece, 5, chess_pieces),       
                Piece::Empty => print!("_"),
                _ => { println!("error")}
            }
            count += 1;
        }
    }   else{
            for piece in board.iter(){
                if count % 8 == 0 {
                    println!("");
                }
                match piece {
                    Piece::Piece {piece_type: 1, .. } => print_piece(*piece, 0, chess_pieces),
                    Piece::Piece {piece_type: 2, .. } => print_piece(*piece, 1, chess_pieces),
                    Piece::Piece {piece_type: 3, .. } => print_piece(*piece, 2, chess_pieces),
                    Piece::Piece {piece_type: 4, .. } => print_piece(*piece, 3, chess_pieces),
                    Piece::Piece {piece_type: 5, .. } => print_piece(*piece, 4, chess_pieces),
                    Piece::Piece {piece_type: 6, .. } => print_piece(*piece, 5, chess_pieces),
                    Piece::Empty => println!("_"),
                    _ => { println!("error")}
                }
                count += 1;
            }
    }
}

pub fn print_piece(piece: Piece, type_of_piece: usize, chess_pieces: [char; 12]) {
    match piece {
        Piece::Piece { white,..} if type_of_piece == 0 => {
            if white {
                print!("{}", chess_pieces[5]);
            } else {
                print!("{}", chess_pieces[11]);
            }
        }
        Piece::Piece { white ,..} if type_of_piece == 1 || type_of_piece == 5 => {
            if white {
                print!("{}", chess_pieces[2]);
            } else {
                print!("{}", chess_pieces[8]);
            }
        }
        Piece::Piece { white,.. } if type_of_piece == 2 => {
            if white {
                print!("{}", chess_pieces[4]);
            } else {
                print!("{}", chess_pieces[10]);
            }
        }
        Piece::Piece { white ,..} if type_of_piece == 3 => {
            if white {
                print!("{}", chess_pieces[3]);
            } else {
                print!("{}", chess_pieces[9]);
            }
        }
        Piece::Piece { white ,..} if type_of_piece == 4 => {
            if white {
                print!("{}", chess_pieces[1]);
            } else {
                print!("{}", chess_pieces[7]);
            }
        }
        Piece::Piece { white ,..} if type_of_piece == 5 => {
            if white {
                print!("{}", chess_pieces[0]);
            } else {
                print!("{}", chess_pieces[6]);
            }
        }
        _ => {
            println!("Unknown piece type or type_of_piece does not match any variant.");
        }
    }
}