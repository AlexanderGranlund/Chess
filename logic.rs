use crate::piece::Piece;
use crate::piece::get_core_moves;

//use std::collections::HashMap;

pub struct Logic{
    height: usize,
    width: usize,
    moves: usize,
    pub board: [Piece;64],
    current_move: [usize;2],
    pub core_moves: Vec<Vec<usize>>,
    pub whites_turn: bool,
    pub current_index: usize,
    pub has_selected: bool,
    pub selected_index: usize,

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
            current_index: 7,
            has_selected: false,
            selected_index: 100,
        }
    }
    pub fn increment_moves(&mut self){
        self.has_selected = false;
        self.moves += 1;
        if self.moves % 2 == 0{
            self.current_index = 7;
            self.whites_turn = true;
        } 
        else{
            self.current_index = 55;
            self.whites_turn = false;
        }
    }

    pub fn view_core_moves(moves: Vec<Vec<usize>>){
        println!("\n");
        for vec in moves{
            println!("{:?}", vec);
        }
    }


    pub fn select_new_piece(&mut self, direction: usize){
        //println!("index enter: {}", self.current_index);
        if self.whites_turn{
            match direction {
            //forward
            1 => {if self.current_index + 8 < 64{
                        self.current_index += 8;
            }},
            //left
            2 =>   {if (self.current_index + 1) % 8 != 0{
                self.current_index += 1;
            }},
            //right
            3 =>  {if (self.current_index - 1) % 8 != 0{
                self.current_index -= 1;
            }},
            //backward
            4 =>  {if !(self.current_index < 8) {
                self.current_index -= 8;
            }},
            _ => println!("_"),
            }   
        } else{
            match direction {
                //backward
                4 => {if self.current_index + 8 < 64{
                            self.current_index += 8;
                }},
                //right
                3 =>   {if (self.current_index + 1) % 8 != 0{
                    self.current_index += 1;
                }},
                //left
                2 =>  {if (self.current_index - 1) % 8 != 0{
                    self.current_index -= 1;
                }},
                //forward
                1 =>  {if !(self.current_index < 8){
                    self.current_index -= 8;
                }},
                _ => println!("_"),
                }   
        }
        //println!("index leave: {}", self.current_index);
        
    }

    pub fn do_action(&mut self,  action: usize){
        match action {
            1 =>  println!("{} <- this is the action", action),
            2 =>   println!("{} <- this is the action", action),
            3 =>  println!("{} <- this is the action", action),
            4 =>  println!("{} <- this is the action", action),
            5 =>  println!("{} <- this is the action", action),
            6 =>  println!("{} <- this is the action", action),
            7 =>  println!("{} <- this is the action", action),
            8 =>  println!("{} <- this is the action", action),
            9 =>    self.select_new_piece(1), //w
            10 =>   self.select_new_piece(2), //a
            11 =>   self.select_new_piece(3), //d
            12 =>   self.select_new_piece(4), //s
            13 =>  println!("{} <- this is the action", action),
            _ => println!("_"),
        }    
    }

}


pub fn is_valid_move(board: &[Option<Piece>;64], current_move: [usize;2]) -> bool{
    return true;
}

fn create_starting_board() -> [Piece;64]{
    let mut board: [Piece;64] = [Piece::Start;64];
    for i in 0..64{
        board[i] = Piece::Empty { position: i};
    }
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



