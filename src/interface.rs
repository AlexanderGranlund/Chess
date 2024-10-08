
use crate::piece::Piece; 
use crate::logic::Logic;
use std::io::{self, Write};


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

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}


pub fn print_board_in_terminal(logic: &Logic){
    //clear_terminal();
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
                Piece::Piece { piece_type: 1, .. } => print_piece(*piece, 0, chess_pieces, logic),
                Piece::Piece { piece_type: 2, .. } => print_piece(*piece, 1, chess_pieces, logic),
                Piece::Piece { piece_type: 3, .. } => print_piece(*piece, 2, chess_pieces, logic),
                Piece::Piece { piece_type: 4, .. } => print_piece(*piece, 3, chess_pieces, logic),
                Piece::Piece { piece_type: 5, .. } => print_piece(*piece, 4, chess_pieces, logic),
                Piece::Piece { piece_type: 6, .. } => print_piece(*piece, 5, chess_pieces, logic),       
                Piece::Empty{position:_} => print_piece(*piece, 6, chess_pieces, logic),
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
                    Piece::Piece {piece_type: 1, .. } => print_piece(*piece, 0, chess_pieces, logic),
                    Piece::Piece {piece_type: 2, .. } => print_piece(*piece, 1, chess_pieces, logic),
                    Piece::Piece {piece_type: 3, .. } => print_piece(*piece, 2, chess_pieces, logic),
                    Piece::Piece {piece_type: 4, .. } => print_piece(*piece, 3, chess_pieces, logic),
                    Piece::Piece {piece_type: 5, .. } => print_piece(*piece, 4, chess_pieces, logic),
                    Piece::Piece {piece_type: 6, .. } => print_piece(*piece, 5, chess_pieces, logic),
                    Piece::Empty{position:_} => print_piece(*piece, 6, chess_pieces, logic),
                    _ => { println!("error")}
                }
                count += 1;
            }
    }
}


pub fn print_piece(piece: Piece, type_of_piece: usize, chess_pieces: [char; 12], logic: &Logic) {
    match piece {
        Piece::Piece { white, position,..} if type_of_piece == 0 => {
            if white {
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[5]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[5])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[5]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[5]);
                }
                else {
                    print!(" {} ", chess_pieces[5]);
                }
                

            } else {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[11]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[11])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[11]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[11]);
                }
                else {
                    print!(" {} ", chess_pieces[11]);
                }
                
             
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 1 || type_of_piece == 1 => {
            if white {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[2]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[2])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[2]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[2]);
                }
                else {
                    print!(" {} ", chess_pieces[2]);
                }
                
            } else {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[8]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[8])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[8]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[8]);
                }
                else {
                    print!(" {} ", chess_pieces[8]);
                }
                
            }
        }
        Piece::Piece { white, position,.. } if type_of_piece == 2 => {
            if white {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[4]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[4])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[4]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[4]);
                }
                else {
                    print!(" {} ", chess_pieces[4]);
                }
               
            } else {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[10]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[10])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[10]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[10]);
                }
                else {
                    print!(" {} ", chess_pieces[10]);
                }
            
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 3 => {
            if white {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[3]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[3])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[3]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[3]);
                }
                else {
                    print!(" {} ", chess_pieces[3]);
                }
             
            } else {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[9]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[9])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[9]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[9]);
                }
                else {
                    print!(" {} ", chess_pieces[9]);
                }
                
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 4 => {
            if white {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[1]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[1])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[1]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[1]);
                }
                else {
                    print!(" {} ", chess_pieces[1]);
                }
            
            } else {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[7]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[7])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[7]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[7]);
                }
                else {
                    print!(" {} ", chess_pieces[7]);
                }
               
            }
        }
        Piece::Piece { white, position ,..} if type_of_piece == 5 => {
            if white {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[0]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[0])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[0]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[0]);
                }
                else {
                    print!(" {} ", chess_pieces[0]);
                }
            
            } else {
                
                if position == logic.current_index && logic.has_selected && logic.current_index != logic.selected_index{
                    print!("\x1b[42m {} \x1b[0m", chess_pieces[6]);
                }
                else if position == logic.selected_index{
                    print!("\x1b[44m {} \x1b[0m", chess_pieces[6])
                }
                else if position == logic.current_index{
                    print!("\x1b[41m {} \x1b[0m", chess_pieces[6]);
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m {} \x1b[0m", chess_pieces[6]);
                }
                else {
                    print!(" {} ", chess_pieces[6]);
                }
                
            }
        }
        Piece::Empty {position ,..} if type_of_piece == 6 => {

                if position == logic.current_index{
                    print!("\x1b[41m _ \x1b[0m");
                }
                else if logic.valid_moves.contains(&position){
                    print!("\x1b[43m _ \x1b[0m");
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