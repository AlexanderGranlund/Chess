

use std::io;

use crate::Piece;
use crate::interface::print_piece;
use crate:: logic::Logic;


pub fn get_input()-> String{
    //println!("\nPlease enter some input:");
    println!("\n");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    //println!("You entered: {}", input);
    return input;
}


pub fn match_input(input: String, has_selected: bool) -> usize{

    let mut action: usize = 0;

    //see if piece has been selected 
    if has_selected {
        println!("{} before matched", action);
        action = match input.as_str(){
            "w" => 1,
            "a" => 2,
            "d" => 3,
            "s" => 4,
            "q" => 5,
            "e" => 6,
            "z" => 7,
            "x" => 8,
            _ => 200,
        }
    }   else {
            action = match input.as_str(){
                "w" => 9,
                "a" => 10,
                "d" => 11,
                "s" => 12,
                "e" => 13,
                _ => 200,
            }
    }

    action
}







