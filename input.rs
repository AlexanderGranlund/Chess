

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
   
        println!("{} before matched", action);
        action = match input.as_str(){
            "w" => 1, 
            "a" => 2,
            "d" => 3,
            "s" => 4,
            "e" => 5,
            "q" => 6,
            "z" => 7,
            "x" => 8,
            _ => 200,
        };
   
    

    action
}







