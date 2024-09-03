use crate::piece::Piece;
use crate::piece::get_core_moves;

use std::collections::HashMap;

pub struct Logic{
    height: usize,
    width: usize,
    moves: usize,
    pub board: [Piece;64],
    current_move: [usize;2],
    pub core_moves: Vec<Vec<usize>>,
    whites_turn: bool,
    selected_index: usize,
    pub has_selected: bool,

}

impl Logic {
    pub fn new() -> Logic{
        Logic{
            height: 8,
            width: 8,
            moves: 0,
            board: create_starting_board(),
            current_move: [0;2],
            core_moves: get_core_moves(),
            whites_turn: true,
            selected_index: 0,
            has_selected: false,
        }
    }
    pub fn increment_moves(&mut self){
        self.selected_index = 0;
        self.has_selected = false;
        self.moves += 1;
    }

    pub fn view_core_moves(moves: Vec<Vec<usize>>){
        println!("\n");
        for vec in moves{
            println!("{:?}", vec);
        }
    }


    fn select_new_piece(&mut self, direction: usize){
        match direction {
            1 => {if self.selected_index + 8 < 64{
                        self.selected_index += 8;
            }},
            2 =>   {if (self.selected_index - 1) % 7 != 0{
                self.selected_index -= 1;
            }},
            3 =>  {if (self.selected_index + 1) % 8 != 0{
                self.selected_index += 1;
            }},
            4 =>  {if self.selected_index - 8 >= 0{
                self.selected_index -= 8;
            }},
            _ => println!("_"),
        }    
    }}

    /*
     forward    =>  action = 1,
    left        =>   action = 2,
    right       =>  action = 3,
    backward    =>  action = 3,
    forward_left_diagonal   =>  action = 4,
    forward_right_diagonal  =>  action = 5,
    backward_left_diagonal  =>  action = 6,
    backward_right_diagonal =>  action = 7,
     */
    pub fn do_action(piece: Piece,  action: usize){
        match action {
            1 =>  println!("{} <- this is the action", action),
            2 =>   println!("{} <- this is the action", action),
            3 =>  println!("{} <- this is the action", action),
            4 =>  println!("{} <- this is the action", action),
            5 =>  println!("{} <- this is the action", action),
            6 =>  println!("{} <- this is the action", action),
            7 =>  println!("{} <- this is the action", action),
            8 =>  println!("{} <- this is the action", action),
            9 =>  println!("{} <- this is the action", action),
            10 =>  println!("{} <- this is the action", action),
            11 =>  println!("{} <- this is the action", action),
            12 =>  println!("{} <- this is the action", action),
            _ => println!("_"),
        }    
        
    }


pub fn is_valid_move(board: &[Option<Piece>;64], current_move: [usize;2]) -> bool{
    return true;
}

fn create_starting_board() -> [Piece;64]{
    let mut board: [Piece;64] = [Piece::Empty;64];
    //white pawns
    for i in 0..8{
        board[i+8] = Piece::Piece {piece_type: 1, white: true, position: (i+8), has_moved: false};
    }
    //black pawns
    for i in 0..8{
        board[55-(i)] = Piece::Piece {piece_type: 1, white: false, position: (55-i), has_moved: false};
    }
    //white rooks
    board[0] =  Piece::Piece {piece_type: 2, white: true, position: 0, has_moved: false};
    board[7] =  Piece::Piece {piece_type: 2, white: true, position: 7, has_moved: false};

    //white knights 
    board[1] =  Piece::Piece {piece_type: 3, white: true, position: 1, has_moved: false};
    board[6] =  Piece::Piece {piece_type: 3, white: true, position: 6, has_moved: false};

    //white bishops
    board[2] =  Piece::Piece {piece_type: 4, white: true, position: 2, has_moved: false};
    board[5] =  Piece::Piece {piece_type: 4, white: true, position: 5, has_moved: false};

    //white queen
    board[3] =  Piece::Piece {piece_type: 5, white: true, position: 3, has_moved: false};
    
    //white king
    board[4] =  Piece::Piece {piece_type: 5, white: true, position: 4, has_moved: false};

    //black rooks
    board[63] =  Piece::Piece {piece_type: 2, white: false, position: 63, has_moved: false};
    board[56] =  Piece::Piece {piece_type: 2, white: false, position: 56, has_moved: false};

    //black knights
    board[62] =  Piece::Piece {piece_type: 3, white: false, position: 62, has_moved: false};
    board[57] =  Piece::Piece {piece_type: 3, white: false, position: 57, has_moved: false};

    //black bishops
    board[61] =  Piece::Piece {piece_type: 4, white: false, position: 61, has_moved: false};
    board[58] =  Piece::Piece {piece_type: 4, white: false, position: 58, has_moved: false};

    //black queen
    board[59] =  Piece::Piece {piece_type: 5, white: false, position: 59, has_moved: false};
    
    //black king
    board[60] =  Piece::Piece {piece_type: 6, white: false, position: 60, has_moved: false};

    return board;
}

