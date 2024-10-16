use std::io;

use crate::logic::Logic;
use crate::Piece;

fn remove_empty_strings(vec: &mut Vec<String>) {
    vec.retain(|s| !s.is_empty());
}

pub fn get_input() -> Vec<String> {
    //println!("\nPlease enter some input:");
    println!("\n");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_vec_str: Vec<char> = input.trim().chars().collect();
    let mut input_vec: Vec<String> = input_vec_str.iter().map(|&s| s.to_string()).collect();
    //println!("You entered: {}", input);
    remove_empty_strings(&mut input_vec);
    println!("input vector: {:?}", input_vec);
    return input_vec;
}

pub fn match_input(input: String) -> usize {
    let action: usize;

    action = match input.as_str() {
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
