

use std::io;

use crate::Piece;
use crate::interface::print_piece;
use crate:: logic::Logic;


pub fn get_input()-> String{
    println!("\nPlease enter some input:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("You entered: {}", input.trim());
    return input.trim().to_string();
}


pub fn match_input(input: String, has_selected: bool) -> usize{

    let mut action: usize = 0;

    //see if piece has been selected 
    if has_selected {
        println!("{} before matched", action);
        match input.as_str(){
            "w" => action = 1,
            "a" => action = 2,
            "d" => action = 3,
            "s" => action = 4,
            "q" => action = 5,
            "e" => action = 6,
            "z" => action = 7,
            "x" => action = 8,
            _ => println!("_"),
        }
    }   else {
            match input.as_str(){
                "w" => action = 9,
                "a" => action = 10,
                "d" => action = 11,
                "e" => action = 12,
                _ => println!("_"),
            }
    }

    match action {
        1 =>  println!("{} <- this is the action", action),
        2 =>   println!("{} <- this is the action", action),
        3 =>  println!("{} <- this is the action", action),
        4 =>  println!("{} <- this is the action", action),
        5 =>  println!("{} <- this is the action", action),
        6 =>  println!("{} <- this is the action", action),
        7 =>  println!("{} <- this is the action", action),
        8 =>  println!("{} <- this is the action", action),
        _ => println!("_"),
    }    
    action
}



