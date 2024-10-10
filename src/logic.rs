use std::io::empty;
use std::ptr::null;

use crate::piece::Piece;
use crate::piece::get_core_moves;

//use std::collections::HashMap;

pub struct Logic{
    height: usize,
    width: usize,
    pub moves: usize,
    pub board: [Piece;64],
    taken_white_pieces: [Piece;16],
    taken_black_pieces: [Piece;16],
    taken_white_pieces_num: usize,
    taken_black_pieces_num: usize,
    current_move: [usize;2],
    pub core_moves: Vec<Vec<usize>>,
    pub whites_turn: bool,
    pub current_index: usize,
    pub has_selected: bool,
    pub selected_index: usize,
    pub valid_moves: Vec<usize>,
    white_in_check: bool,
    black_in_check: bool,

}

impl Logic {
    pub fn new() -> Logic{
        Logic{
            height: 8,
            width: 8,
            moves: 0,
            board: create_starting_board(),
            taken_white_pieces: [Piece::Empty { position: (0) };16],
            taken_black_pieces: [Piece::Empty { position: (0) };16],
            taken_white_pieces_num: 0,
            taken_black_pieces_num: 0,
            current_move: [0;2],
            core_moves: get_core_moves(),
            whites_turn: true,
            current_index: 7,
            has_selected: false,
            selected_index: 100,
            valid_moves: vec![],
            white_in_check: false,
            black_in_check: false,
        }
    }
    pub fn increment_moves(&mut self){
        println!("\nfrom increment moves (1) ");
        self.print_logic();
        self.has_selected = false;
        self.selected_index = 100;
        self.moves += 1;
        if !self.whites_turn{
            self.current_index = 7;
            self.whites_turn = true;
        } 
        else{
            self.current_index = 56;
            self.whites_turn = false;
        }
        println!("\nfrom increment moves (2) ");
        self.print_logic();
        self.get_valid_moves();
        
    }

    pub fn view_core_moves(moves: &Vec<Vec<usize>>){
        println!("\n");
        for vec in moves{
            println!("{:?}", vec);
        }
    }

    fn print_logic(& self){
        println!("whites turn: {}", self.whites_turn);
        println!("has selected: {}", self.has_selected);
        println!("selected index: {}", self.selected_index);
        println!("current index: {}", self.current_index);
        println!("valid moves: {:?}", self.valid_moves);
    }

    pub fn select_new_piece(&mut self, direction: usize){
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
            3 =>  {if (self.current_index) % 8 != 0{
                self.current_index -= 1;
            }},
            //backward
            4 =>  {if !(self.current_index < 8) {
                self.current_index -= 8;
            }},
            _ => println!("direction is: {}", direction),
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
                2 =>  {if (self.current_index) % 8 != 0{
                    self.current_index -= 1;
                }},
                //forward
                1 =>  {if !(self.current_index < 8){
                    self.current_index -= 8;
                }},
                _ => println!("direction is: {}", direction),
            }   
        }

        self.get_valid_moves();
       
        
    }

    fn select_and_lock(&mut self){
        let is_white:bool = self.is_white(self.current_index);
        let is_piece:bool = self.is_piece(self.current_index);
        if self.has_selected && self.selected_index != self.current_index{
            
            if is_piece && ((is_white && self.whites_turn) || (!is_white && !self.whites_turn)) {
                return;
            }
            if self.valid_moves.contains(&self.current_index){
                self.board[self.current_index] = self.board[self.selected_index];
                if let Piece::Piece { ref mut position, .. } = self.board[self.current_index] {
                    *position = self.current_index;
                }
                if let Piece::Piece { ref mut has_moved, .. } = self.board[self.current_index] {
                    *has_moved = true;
                }
                self.board[self.selected_index] = Piece::Empty { position: self.selected_index };
                self.increment_moves();
            }
            
        }
        else if !((!is_white && self.whites_turn) || (is_white && !self.whites_turn)) && is_piece{
            self.selected_index = self.current_index;
            self.has_selected = true;
            self.get_valid_moves();
        
        }
       
       
    }

    fn is_piece(&mut self, index:usize)-> bool{
        let is_piece = match self.board[index] {
            Piece::Empty { .. } => false,
            Piece::Start => false,
            _ => true,
        };
        return is_piece
    }
    fn is_white(&mut self ,index:usize)-> bool{
        if let Piece::Piece { white, .. } = self.board[index] {
            return white;
        }
        else {
            print!("is_white() was used on a non piece");
        }
        return false;

    }

    fn has_moved(&mut self ,index:usize)-> bool{
        if let Piece::Piece { has_moved, .. } = self.board[index] {
            return has_moved;
        }
        else {
            print!("has_moved() was used on a non piece");
        }
        return false;

    }

    /*
    "w" => 1, 
    "a" => 2,
    "d" => 3,
    "s" => 4,
    "q" => 5,
    "e" => 6,
    "z" => 7,
    "x" => 8,
    _ => 200, 
    */

    pub fn do_action(&mut self,  action: usize){
        match action {
            1 =>   self.select_new_piece(1), //w
            2 =>   self.select_new_piece(2), //a
            3 =>   self.select_new_piece(3), //d
            4 =>   self.select_new_piece(4), //s
            5 =>  self.select_and_lock(),              //e
            6 =>  println!("{} <- this is the action", action),
            7 =>  println!("{} <- this is the action", action),
            8 =>  println!("{} <- this is the action", action),
            _ => println!("action is: {}", action),
        }    
    }


    pub fn get_valid_moves(&mut self){
        self.valid_moves.clear();
        let index:usize;
        if self.has_selected{
            index = self.selected_index;
            
        }
        else{
            index = self.current_index;
        }
        let piece: Piece = self.board[index];
        match piece {
            Piece::Piece { piece_type: 1, .. } => self.valid_moves_pawn(),
            Piece::Piece { piece_type: 2, .. } => self.valid_moves_rook(),
            Piece::Piece { piece_type: 3, .. } => self.valid_moves_knight(),
            Piece::Piece { piece_type: 4, .. } => self.valid_moves_bishop(),
            Piece::Piece { piece_type: 5, .. } => self.valid_moves_queen(),
            Piece::Piece { piece_type: 6, .. } => self.valid_moves_king(),   
            Piece::Empty{ .. } => return,    
            _ => { println!("error")}
        }
    
    }

    pub fn valid_moves_pawn(&mut self){
        let to:usize;
        let index:usize;
        if self.has_selected{
            index = self.selected_index;
            
        }
        else{
            index = self.current_index;
        }
        self.print_logic();
        if !self.has_moved(index){
           
            to= 3;
        }
        else {
            
            to= 2;
        }

        if self.whites_turn && self.is_white(index){
           
            for i in 1..to{
                if (i*8 + index) < 63{
                    let is_piece: bool = self.is_piece(index + i*8);
                    if is_piece{
                        break;
                    }
                    else {
                        //check if in check first!!
                        
                        self.valid_moves.push(index + i*8);
                    }
                }
            }
            if !(index % 8 == 0){
                let is_piece_dia_right: bool = self.is_piece(index + 8 - 1);
                let is_white_dia_right: bool = self.is_white(index + 8 - 1);
                if is_piece_dia_right && !is_white_dia_right{
                    self.valid_moves.push(index + 8 - 1);
                }
                
            }
            if !(index + 1 % 8 == 0){
                let is_piece_dia_left: bool = self.is_piece(index + 8 + 1);
                let is_white_dia_left: bool = self.is_white(index + 8 + 1);
                if is_piece_dia_left && !is_white_dia_left{
                    self.valid_moves.push(index + 8 + 1);
                }
            }
        }
        else if !self.whites_turn && !self.is_white(index){
            
            for i in 1..to{
                if index > 7{
                    let is_piece: bool = self.is_piece(index - i*8);
                    if is_piece{
                        break;
                    }
                    else {
                        //check if in check first!!
                        self.valid_moves.push(index - i*8);
                    }
                }
            }
            if !((index + 1) % 8 == 0){
                let is_piece_dia_right: bool = self.is_piece(index - 8 + 1);
                let is_white_dia_right: bool = self.is_white(index - 8 + 1);
                if is_piece_dia_right && is_white_dia_right{
                    self.valid_moves.push(index - 8 + 1);
                }
                
            }
            if !(index % 8 == 0){
                let is_piece_dia_left: bool = self.is_piece(index - 8 - 1);
                let is_white_dia_left: bool = self.is_white(index - 8 - 1);
                if is_piece_dia_left && is_white_dia_left{
                    self.valid_moves.push(index - 8 - 1);
                }
            }
        }
        
    }
    pub fn valid_moves_rook(&mut self){
       
    
    }
    pub fn valid_moves_knight(&mut self){
       
    
    }
    pub fn valid_moves_bishop(&mut self){
       
    
    }
    pub fn valid_moves_queen(&mut self){
       
    
    }
    pub fn valid_moves_king(&mut self){
       
    
    }


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
    board[4] =  Piece::Piece {piece_type: 5, white: true, position: 4, has_moved: false};
    
    //white king
    board[3] =  Piece::Piece {piece_type: 6, white: true, position: 3, has_moved: false};

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
    board[60] =  Piece::Piece {piece_type: 5, white: false, position: 60, has_moved: false};
    
    //black king
    board[59] =  Piece::Piece {piece_type: 6, white: false, position: 59, has_moved: false};

    return board;
}



