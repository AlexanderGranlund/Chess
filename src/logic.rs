use std::io::empty;
use std::ptr::null;

use crate::piece;
use crate::piece::get_core_moves;
use crate::piece::Piece;

//use std::collections::HashMap;

pub struct Logic {
    height: usize,
    width: usize,
    pub moves: usize,
    pub board: [Piece; 64],
    pub temp_board: [Piece; 64],
    pub taken_white_pieces: Vec<usize>,
    pub taken_black_pieces: Vec<usize>,
    pub all_white_moves: Vec<usize>,
    pub all_black_moves: Vec<usize>,
    pub taken_white_pieces_num: usize,
    pub taken_black_pieces_num: usize,
    current_move: [usize; 2],
    pub core_moves: Vec<Vec<usize>>,
    pub whites_turn: bool,
    pub current_index: usize,
    pub has_selected: bool,
    pub selected_index: usize,
    pub valid_moves: Vec<usize>,
    pub temp_valid_moves: Vec<usize>,
    pub white_king_position: usize,
    pub black_king_position: usize,
    pub white_in_check: bool,
    pub black_in_check: bool,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            height: 8,
            width: 8,
            moves: 0,
            board: create_starting_board(),
            temp_board: create_starting_board(),
            taken_white_pieces: vec![],
            taken_black_pieces: vec![],
            all_white_moves: vec![],
            all_black_moves: vec![],
            taken_white_pieces_num: 0,
            taken_black_pieces_num: 0,
            current_move: [0; 2],
            core_moves: get_core_moves(),
            whites_turn: true,
            current_index: 7,
            has_selected: false,
            selected_index: 100,
            valid_moves: vec![],
            temp_valid_moves: vec![],
            white_king_position: 3,
            black_king_position: 59,
            white_in_check: false,
            black_in_check: false,
        }
    }
    pub fn increment_moves(&mut self) {
        self.white_in_check = false;
        self.black_in_check = false;
        self.has_selected = false;
        self.selected_index = 100;
        self.moves += 1;
        if !self.whites_turn {
            self.current_index = 7;
            self.whites_turn = true;
        } else {
            self.current_index = 56;
            self.whites_turn = false;
        }

        self.get_valid_moves(100);
        self.clean_valid_moves();
    }

    pub fn view_core_moves(moves: &Vec<Vec<usize>>) {
        println!("\n");
        for vec in moves {
            println!("{:?}", vec);
        }
    }

    fn print_logic(&self) {
        println!("whites turn: {}", self.whites_turn);
        println!("has selected: {}", self.has_selected);
        println!("selected index: {}", self.selected_index);
        println!("current index: {}", self.current_index);
        println!("valid moves: {:?}", self.valid_moves);
    }

    pub fn select_new_piece(&mut self, direction: usize) {
        if self.whites_turn {
            match direction {
                //forward
                1 => {
                    if self.current_index + 8 < 64 {
                        self.current_index += 8;
                    }
                }
                //left
                2 => {
                    if (self.current_index + 1) % 8 != 0 {
                        self.current_index += 1;
                    }
                }
                //right
                3 => {
                    if (self.current_index) % 8 != 0 {
                        self.current_index -= 1;
                    }
                }
                //backward
                4 => {
                    if (self.current_index >= 8) {
                        self.current_index -= 8;
                    }
                }
                _ => println!("direction is: {}", direction),
            }
        } else {
            match direction {
                //backward
                4 => {
                    if self.current_index + 8 < 64 {
                        self.current_index += 8;
                    }
                }
                //right
                3 => {
                    if (self.current_index + 1) % 8 != 0 {
                        self.current_index += 1;
                    }
                }
                //left
                2 => {
                    if (self.current_index) % 8 != 0 {
                        self.current_index -= 1;
                    }
                }
                //forward
                1 => {
                    if (self.current_index >= 8) {
                        self.current_index -= 8;
                    }
                }
                _ => println!("direction is: {}", direction),
            }
        }

        self.get_valid_moves(100);
        self.clean_valid_moves();
    }

    fn select_and_lock(&mut self) {
        let is_white: bool = self.is_white(self.current_index);
        let is_piece: bool = self.is_piece(self.current_index);
        if self.has_selected && self.selected_index != self.current_index {
            if is_piece && ((is_white && self.whites_turn) || (!is_white && !self.whites_turn)) {
                return;
            }
            if self.valid_moves.contains(&self.current_index) {
                if self.is_piece(self.current_index) {
                    if self.is_white(self.current_index) {
                        self.taken_white_pieces_num += 1;
                        self.taken_white_pieces
                            .push(self.piece_type(self.current_index));
                        self.taken_white_pieces.sort();
                    } else {
                        self.taken_black_pieces_num += 1;
                        self.taken_black_pieces
                            .push(self.piece_type(self.current_index));
                        self.taken_black_pieces.sort();
                    }
                }

                if self.piece_type(self.selected_index) == 6 {
                    if self.is_white(self.selected_index) {
                        self.white_king_position = self.current_index;
                    } else {
                        self.black_king_position = self.current_index;
                    }
                }

                self.board[self.current_index] = self.board[self.selected_index];
                if let Piece::Piece {
                    ref mut position, ..
                } = self.board[self.current_index]
                {
                    *position = self.current_index;
                }
                if let Piece::Piece {
                    ref mut has_moved, ..
                } = self.board[self.current_index]
                {
                    *has_moved = true;
                }
                self.board[self.selected_index] = Piece::Empty {
                    position: self.selected_index,
                };
                self.increment_moves();
            }
        } else if !((!is_white && self.whites_turn) || (is_white && !self.whites_turn))
            && is_piece
            && self.valid_moves.len() > 0
        {
            self.selected_index = self.current_index;
            self.has_selected = true;
            self.get_valid_moves(100);
            self.clean_valid_moves();
        }
    }

    fn is_piece(&self, index: usize) -> bool {
        let is_piece = match self.board[index] {
            Piece::Empty { .. } => false,
            Piece::Start => false,
            _ => true,
        };
        return is_piece;
    }
    fn is_white(&self, index: usize) -> bool {
        if let Piece::Piece { white, .. } = self.board[index] {
            return white;
        }

        return false;
    }

    fn piece_type(&self, index: usize) -> usize {
        if self.is_piece(index) {
            if let Piece::Piece { piece_type, .. } = self.board[index] {
                return piece_type;
            }
        }
        return 200;
    }

    fn has_moved(&self, index: usize) -> bool {
        if let Piece::Piece { has_moved, .. } = self.board[index] {
            return has_moved;
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

    pub fn do_action(&mut self, action: usize) {
        match action {
            1 => self.select_new_piece(1), //w
            2 => self.select_new_piece(2), //a
            3 => self.select_new_piece(3), //d
            4 => self.select_new_piece(4), //s
            5 => self.select_and_lock(),   //e
            6 => println!("{} <- this is the action", action),
            7 => println!("{} <- this is the action", action),
            8 => println!("{} <- this is the action", action),
            _ => println!("action is: {}", action),
        }
    }

    pub fn get_valid_moves(&mut self, mut index: usize) {
        self.valid_moves.clear();
        if index == 100 {
            if self.has_selected {
                index = self.selected_index;
            } else {
                index = self.current_index;
            }
        }

        let piece: Piece = self.board[index];
        match piece {
            Piece::Piece { piece_type: 1, .. } => self.valid_moves_pawn(index),
            Piece::Piece { piece_type: 2, .. } => self.valid_moves_rook(index),
            Piece::Piece { piece_type: 3, .. } => self.valid_moves_knight(index),
            Piece::Piece { piece_type: 4, .. } => self.valid_moves_bishop(index),
            Piece::Piece { piece_type: 5, .. } => self.valid_moves_queen(index),
            Piece::Piece { piece_type: 6, .. } => self.valid_moves_king(index),
            Piece::Empty { .. } => return,
            _ => {
                println!("error")
            }
        }
    }

    pub fn valid_moves_pawn(&mut self, index: usize) {
        let to: usize;

        if !self.has_moved(index) {
            to = 3;
        } else {
            to = 2;
        }

        if self.is_white(index) {
            for i in 1..to {
                if (i * 8 + index) < 63 {
                    let is_piece: bool = self.is_piece(index + i * 8);
                    if is_piece {
                        break;
                    } else {
                        //check if in check first!!

                        self.valid_moves.push(index + i * 8);
                    }
                }
            }
            if !(index % 8 == 0) {
                let is_piece_dia_right: bool = self.is_piece(index + 8 - 1);
                let is_white_dia_right: bool = self.is_white(index + 8 - 1);
                if is_piece_dia_right && !is_white_dia_right {
                    self.valid_moves.push(index + 8 - 1);
                }
            }
            if !(index + 1 % 8 == 0) {
                let is_piece_dia_left: bool = self.is_piece(index + 8 + 1);
                let is_white_dia_left: bool = self.is_white(index + 8 + 1);
                if is_piece_dia_left && !is_white_dia_left {
                    self.valid_moves.push(index + 8 + 1);
                }
            }
        } else if !self.is_white(index) {
            for i in 1..to {
                if index > 7 {
                    let is_piece: bool = self.is_piece(index - i * 8);
                    if is_piece {
                        break;
                    } else {
                        //check if in check first!!
                        self.valid_moves.push(index - i * 8);
                    }
                }
            }
            if !((index + 1) % 8 == 0) {
                let is_piece_dia_right: bool = self.is_piece(index - 8 + 1);
                let is_white_dia_right: bool = self.is_white(index - 8 + 1);
                if is_piece_dia_right && is_white_dia_right {
                    self.valid_moves.push(index - 8 + 1);
                }
            }
            if !(index % 8 == 0) {
                let is_piece_dia_left: bool = self.is_piece(index - 8 - 1);
                let is_white_dia_left: bool = self.is_white(index - 8 - 1);
                if is_piece_dia_left && is_white_dia_left {
                    self.valid_moves.push(index - 8 - 1);
                }
            }
        }
    }

    pub fn valid_moves_rook(&mut self, index: usize) {
        let mut stop_north = false;
        let mut stop_west = false;
        let mut stop_east = false;
        let mut stop_south = false;
        let row = index / 8;

        for i in 1..8 {
            if self.is_white(index) {
                if !stop_north && (index + i * 8) < 64 {
                    let is_piece: bool = self.is_piece(index + i * 8);
                    let is_white: bool = self.is_white(index + i * 8);
                    if is_piece && is_white {
                        stop_north = true;
                    } else {
                        self.valid_moves.push(index + i * 8);
                        if is_piece {
                            stop_north = true;
                        }
                    }
                }
                if !stop_west && (index + i) / 8 == row && (index + i) < 64 {
                    let is_piece: bool = self.is_piece(index + i);
                    let is_white: bool = self.is_white(index + i);
                    if is_piece && is_white {
                        stop_west = true;
                    } else {
                        self.valid_moves.push(index + i);
                        if is_piece {
                            stop_west = true;
                        }
                    }
                }
                if !stop_east && index >= i && (index - i) / 8 == row {
                    let is_piece: bool = self.is_piece(index - i);
                    let is_white: bool = self.is_white(index - i);
                    if is_piece && is_white {
                        stop_east = true;
                    } else {
                        self.valid_moves.push(index - i);
                        if is_piece {
                            stop_east = true;
                        }
                    }
                }
                if !stop_south && index > i * 8 {
                    let is_piece: bool = self.is_piece(index - i * 8);
                    let is_white: bool = self.is_white(index - i * 8);
                    if is_piece && is_white {
                        stop_south = true;
                    } else {
                        self.valid_moves.push(index - i * 8);
                        if is_piece {
                            stop_south = true;
                        }
                    }
                }
            } else if !self.is_white(index) {
                if !stop_north && (index > i * 8) {
                    let is_piece: bool = self.is_piece(index - i * 8);
                    let is_white: bool = self.is_white(index - i * 8);
                    if is_piece && !is_white {
                        stop_north = true;
                    } else {
                        self.valid_moves.push(index - i * 8);
                        if is_piece {
                            stop_north = true;
                        }
                    }
                }
                if !stop_west && index >= i && (index - i) / 8 == row {
                    let is_piece: bool = self.is_piece(index - i);
                    let is_white: bool = self.is_white(index - i);
                    if is_piece && !is_white {
                        stop_west = true;
                    } else {
                        self.valid_moves.push(index - i);
                        if is_piece {
                            stop_west = true;
                        }
                    }
                }
                if !stop_east && (index + i) / 8 == row && (index + i) < 64 {
                    let is_piece: bool = self.is_piece(index + i);
                    let is_white: bool = self.is_white(index + i);
                    if is_piece && !is_white {
                        stop_east = true;
                    } else {
                        self.valid_moves.push(index + i);
                        if is_piece {
                            stop_east = true;
                        }
                    }
                }
                if !stop_south && (index + i * 8) < 64 {
                    let is_piece: bool = self.is_piece(index + i * 8);
                    let is_white: bool = self.is_white(index + i * 8);
                    if is_piece && !is_white {
                        stop_south = true;
                    } else {
                        self.valid_moves.push(index + i * 8);
                        if is_piece {
                            stop_south = true;
                        }
                    }
                }
            }
        }
    }
    pub fn valid_moves_knight(&mut self, index: usize) {
        if self.is_white(index) {
            if (index + 15) < 64 && index % 8 != 0 {
                let is_white: bool = self.is_white(index + 15);

                if !is_white {
                    self.valid_moves.push(index + 15);
                }
            }

            if (index + 17) < 64 && index % 8 != 7 {
                let is_white: bool = self.is_white(index + 17);

                if !is_white {
                    self.valid_moves.push(index + 17);
                }
            }

            if (index + 6) < 64 && index % 8 != 0 && index % 8 != 1 {
                let is_white: bool = self.is_white(index + 6);

                if !is_white {
                    self.valid_moves.push(index + 6);
                }
            }

            if (index + 10) < 64 && index % 8 != 7 && index % 8 != 6 {
                let is_white: bool = self.is_white(index + 10);

                if !is_white {
                    self.valid_moves.push(index + 10);
                }
            }

            if (index > 17) && index % 8 != 0 {
                let is_white: bool = self.is_white(index - 17);

                if !is_white {
                    self.valid_moves.push(index - 17);
                }
            }

            if (index > 15) && index % 8 != 7 {
                let is_white: bool = self.is_white(index - 15);

                if !is_white {
                    self.valid_moves.push(index - 15);
                }
            }

            if (index > 10) && index % 8 != 0 && index % 8 != 1 {
                let is_white: bool = self.is_white(index - 10);

                if !is_white {
                    self.valid_moves.push(index - 10);
                }
            }

            if (index > 6) && index % 8 != 7 && index % 8 != 6 {
                let is_white: bool = self.is_white(index - 6);

                if !is_white {
                    self.valid_moves.push(index - 6);
                }
            }
        } else if !self.is_white(index) {
            if (index + 15) < 64 && index % 8 != 0 {
                let is_white: bool = self.is_white(index + 15);
                let is_piece: bool = self.is_piece(index + 15);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 15);
                }
            }

            if (index + 17) < 64 && index % 8 != 7 {
                let is_white: bool = self.is_white(index + 17);
                let is_piece: bool = self.is_piece(index + 17);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 17);
                }
            }

            if (index + 6) < 64 && index % 8 != 0 && index % 8 != 1 {
                let is_white: bool = self.is_white(index + 6);
                let is_piece: bool = self.is_piece(index + 6);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 6);
                }
            }

            if (index + 10) < 64 && index % 8 != 7 && index % 8 != 6 {
                let is_white: bool = self.is_white(index + 10);
                let is_piece: bool = self.is_piece(index + 10);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 10);
                }
            }

            if (index > 17) && index % 8 != 0 {
                let is_white: bool = self.is_white(index - 17);
                let is_piece: bool = self.is_piece(index - 17);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 17);
                }
            }

            if (index > 15) && index % 8 != 7 {
                let is_white: bool = self.is_white(index - 15);
                let is_piece: bool = self.is_piece(index - 15);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 15);
                }
            }

            if (index > 10) && index % 8 != 0 && index % 8 != 1 {
                let is_white: bool = self.is_white(index - 10);
                let is_piece: bool = self.is_piece(index - 10);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 10);
                }
            }

            if (index > 6) && index % 8 != 7 && index % 8 != 6 {
                let is_white: bool = self.is_white(index - 6);
                let is_piece: bool = self.is_piece(index - 6);

                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 6);
                }
            }
        }
    }

    pub fn valid_moves_bishop(&mut self, index: usize) {
        let mut stop_NE = false;
        let mut stop_NW = false;
        let mut stop_SE = false;
        let mut stop_SW = false;

        for i in 1..8 {
            if self.is_white(index) {
                if (index + i * 7) > 63 || (index + i * 7) % 8 == 7 {
                    stop_NE = true;
                }
                if !stop_NE {
                    let is_piece: bool = self.is_piece(index + i * 7);
                    let is_white: bool = self.is_white(index + i * 7);
                    if is_piece && is_white {
                        stop_NE = true;
                    } else {
                        self.valid_moves.push(index + i * 7);
                        if is_piece {
                            stop_NE = true;
                        }
                    }
                }
                if (index + i * 9) > 63 || (index + i * 9) % 8 == 0 {
                    stop_NW = true;
                }
                if !stop_NW {
                    let is_piece: bool = self.is_piece(index + i * 9);
                    let is_white: bool = self.is_white(index + i * 9);
                    if is_piece && is_white {
                        stop_NW = true;
                    } else {
                        self.valid_moves.push(index + i * 9);
                        if is_piece {
                            stop_NW = true;
                        }
                    }
                }
                if (index < i * 9) || (index - i * 9) % 8 == 7 {
                    stop_SE = true;
                }
                if !stop_SE {
                    let is_piece: bool = self.is_piece(index - i * 9);
                    let is_white: bool = self.is_white(index - i * 9);
                    if is_piece && is_white {
                        stop_SE = true;
                    } else {
                        self.valid_moves.push(index - i * 9);
                        if is_piece {
                            stop_SE = true;
                        }
                    }
                }
                if (index < i * 7) || (index - i * 7) % 8 == 0 {
                    stop_SW = true;
                }
                if !stop_SW {
                    let is_piece: bool = self.is_piece(index - i * 7);
                    let is_white: bool = self.is_white(index - i * 7);
                    if is_piece && is_white {
                        stop_SW = true;
                    } else {
                        self.valid_moves.push(index - i * 7);
                        if is_piece {
                            stop_SW = true;
                        }
                    }
                }
            } else if !self.is_white(index) {
                if (index < i * 7) || (index - i * 7) % 8 == 0 {
                    stop_NE = true;
                }
                if !stop_NE {
                    let is_piece: bool = self.is_piece(index - i * 7);
                    let is_white: bool = self.is_white(index - i * 7);
                    if is_piece && !is_white {
                        stop_NE = true;
                    } else {
                        self.valid_moves.push(index - i * 7);
                        if is_piece {
                            stop_NE = true;
                        }
                    }
                }
                if (index < i * 9) || (index - i * 9) % 8 == 7 {
                    stop_NW = true;
                }
                if !stop_NW {
                    let is_piece: bool = self.is_piece(index - i * 9);
                    let is_white: bool = self.is_white(index - i * 9);
                    if is_piece && !is_white {
                        stop_NW = true;
                    } else {
                        self.valid_moves.push(index - i * 9);
                        if is_piece {
                            stop_NW = true;
                        }
                    }
                }
                if (index + i * 9) > 63 || (index + i * 9) % 8 == 0 {
                    stop_SE = true;
                }
                if !stop_SE {
                    let is_piece: bool = self.is_piece(index + i * 9);
                    let is_white: bool = self.is_white(index + i * 9);
                    if is_piece && !is_white {
                        stop_SE = true;
                    } else {
                        self.valid_moves.push(index + i * 9);
                        if is_piece {
                            stop_SE = true;
                        }
                    }
                }
                if (index + i * 7) > 63 || (index + i * 7) % 8 == 7 {
                    stop_SW = true;
                }
                if !stop_SW {
                    let is_piece: bool = self.is_piece(index + i * 7);
                    let is_white: bool = self.is_white(index + i * 7);
                    if is_piece && !is_white {
                        stop_SW = true;
                    } else {
                        self.valid_moves.push(index + i * 7);
                        if is_piece {
                            stop_SW = true;
                        }
                    }
                }
            }
        }
    }
    pub fn valid_moves_queen(&mut self, index: usize) {
        let mut stop_north = false;
        let mut stop_west = false;
        let mut stop_east = false;
        let mut stop_south = false;

        let mut stop_NE = false;
        let mut stop_NW = false;
        let mut stop_SE = false;
        let mut stop_SW = false;

        let row = index / 8;

        for i in 1..8 {
            if self.is_white(index) {
                //vertical & horisontal
                if !stop_north && (index + i * 8) < 64 {
                    let is_piece: bool = self.is_piece(index + i * 8);
                    let is_white: bool = self.is_white(index + i * 8);
                    if is_piece && is_white {
                        stop_north = true;
                    } else {
                        self.valid_moves.push(index + i * 8);
                        if is_piece {
                            stop_north = true;
                        }
                    }
                }
                if !stop_west && (index + i) / 8 == row && (index + i) < 64 {
                    let is_piece: bool = self.is_piece(index + i);
                    let is_white: bool = self.is_white(index + i);
                    if is_piece && is_white {
                        stop_west = true;
                    } else {
                        self.valid_moves.push(index + i);
                        if is_piece {
                            stop_west = true;
                        }
                    }
                }
                if !stop_east && index >= i && (index - i) / 8 == row {
                    let is_piece: bool = self.is_piece(index - i);
                    let is_white: bool = self.is_white(index - i);
                    if is_piece && is_white {
                        stop_east = true;
                    } else {
                        self.valid_moves.push(index - i);
                        if is_piece {
                            stop_east = true;
                        }
                    }
                }
                if !stop_south && index > i * 8 {
                    let is_piece: bool = self.is_piece(index - i * 8);
                    let is_white: bool = self.is_white(index - i * 8);
                    if is_piece && is_white {
                        stop_south = true;
                    } else {
                        self.valid_moves.push(index - i * 8);
                        if is_piece {
                            stop_south = true;
                        }
                    }
                }

                //diagonal
                if (index + i * 7) > 63 || (index + i * 7) % 8 == 7 {
                    stop_NE = true;
                }
                if !stop_NE {
                    let is_piece: bool = self.is_piece(index + i * 7);
                    let is_white: bool = self.is_white(index + i * 7);
                    if is_piece && is_white {
                        stop_NE = true;
                    } else {
                        self.valid_moves.push(index + i * 7);
                        if is_piece {
                            stop_NE = true;
                        }
                    }
                }
                if (index + i * 9) > 63 || (index + i * 9) % 8 == 0 {
                    stop_NW = true;
                }
                if !stop_NW {
                    let is_piece: bool = self.is_piece(index + i * 9);
                    let is_white: bool = self.is_white(index + i * 9);
                    if is_piece && is_white {
                        stop_NW = true;
                    } else {
                        self.valid_moves.push(index + i * 9);
                        if is_piece {
                            stop_NW = true;
                        }
                    }
                }
                if (index < i * 9) || (index - i * 9) % 8 == 7 {
                    stop_SE = true;
                }
                if !stop_SE {
                    let is_piece: bool = self.is_piece(index - i * 9);
                    let is_white: bool = self.is_white(index - i * 9);
                    if is_piece && is_white {
                        stop_SE = true;
                    } else {
                        self.valid_moves.push(index - i * 9);
                        if is_piece {
                            stop_SE = true;
                        }
                    }
                }
                if (index < i * 7) || (index - i * 7) % 8 == 0 {
                    stop_SW = true;
                }
                if !stop_SW {
                    let is_piece: bool = self.is_piece(index - i * 7);
                    let is_white: bool = self.is_white(index - i * 7);
                    if is_piece && is_white {
                        stop_SW = true;
                    } else {
                        self.valid_moves.push(index - i * 7);
                        if is_piece {
                            stop_SW = true;
                        }
                    }
                }
            } else if !self.is_white(index) {
                //vertical & horisontal
                if !stop_north && (index > i * 8) {
                    let is_piece: bool = self.is_piece(index - i * 8);
                    let is_white: bool = self.is_white(index - i * 8);
                    if is_piece && !is_white {
                        stop_north = true;
                    } else {
                        self.valid_moves.push(index - i * 8);
                        if is_piece {
                            stop_north = true;
                        }
                    }
                }
                if !stop_west && index >= i && (index - i) / 8 == row {
                    let is_piece: bool = self.is_piece(index - i);
                    let is_white: bool = self.is_white(index - i);
                    if is_piece && !is_white {
                        stop_west = true;
                    } else {
                        self.valid_moves.push(index - i);
                        if is_piece {
                            stop_west = true;
                        }
                    }
                }
                if !stop_east && (index + i) / 8 == row && (index + i) < 64 {
                    let is_piece: bool = self.is_piece(index + i);
                    let is_white: bool = self.is_white(index + i);
                    if is_piece && !is_white {
                        stop_east = true;
                    } else {
                        self.valid_moves.push(index + i);
                        if is_piece {
                            stop_east = true;
                        }
                    }
                }
                if !stop_south && (index + i * 8) < 64 {
                    let is_piece: bool = self.is_piece(index + i * 8);
                    let is_white: bool = self.is_white(index + i * 8);
                    if is_piece && !is_white {
                        stop_south = true;
                    } else {
                        self.valid_moves.push(index + i * 8);
                        if is_piece {
                            stop_south = true;
                        }
                    }
                }

                //diagonal
                if (index < i * 7) || (index - i * 7) % 8 == 0 {
                    stop_NE = true;
                }
                if !stop_NE {
                    let is_piece: bool = self.is_piece(index - i * 7);
                    let is_white: bool = self.is_white(index - i * 7);
                    if is_piece && !is_white {
                        stop_NE = true;
                    } else {
                        self.valid_moves.push(index - i * 7);
                        if is_piece {
                            stop_NE = true;
                        }
                    }
                }
                if (index < i * 9) || (index - i * 9) % 8 == 7 {
                    stop_NW = true;
                }
                if !stop_NW {
                    let is_piece: bool = self.is_piece(index - i * 9);
                    let is_white: bool = self.is_white(index - i * 9);
                    if is_piece && !is_white {
                        stop_NW = true;
                    } else {
                        self.valid_moves.push(index - i * 9);
                        if is_piece {
                            stop_NW = true;
                        }
                    }
                }
                if (index + i * 9) > 63 || (index + i * 9) % 8 == 0 {
                    stop_SE = true;
                }
                if !stop_SE {
                    let is_piece: bool = self.is_piece(index + i * 9);
                    let is_white: bool = self.is_white(index + i * 9);
                    if is_piece && !is_white {
                        stop_SE = true;
                    } else {
                        self.valid_moves.push(index + i * 9);
                        if is_piece {
                            stop_SE = true;
                        }
                    }
                }
                if (index + i * 7) > 63 || (index + i * 7) % 8 == 7 {
                    stop_SW = true;
                }
                if !stop_SW {
                    let is_piece: bool = self.is_piece(index + i * 7);
                    let is_white: bool = self.is_white(index + i * 7);
                    if is_piece && !is_white {
                        stop_SW = true;
                    } else {
                        self.valid_moves.push(index + i * 7);
                        if is_piece {
                            stop_SW = true;
                        }
                    }
                }
            }
        }
    }

    pub fn valid_moves_king(&mut self, index: usize) {
        let row = index / 8;

        if self.is_white(index) {
            //vertical & horisontal
            if (index + 8) < 64 {
                let is_piece: bool = self.is_piece(index + 8);
                let is_white: bool = self.is_white(index + 8);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index + 8);
                }
            }
            if (index + 1) / 8 == row && (index + 1) < 64 {
                let is_piece: bool = self.is_piece(index + 1);
                let is_white: bool = self.is_white(index + 1);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index + 1);
                }
            }
            if index >= 1 && (index - 1) / 8 == row {
                let is_piece: bool = self.is_piece(index - 1);
                let is_white: bool = self.is_white(index - 1);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index - 1);
                }
            }
            if index > 8 {
                let is_piece: bool = self.is_piece(index - 8);
                let is_white: bool = self.is_white(index - 8);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index - 8)
                }
            }

            //diagonal
            if !((index + 7) > 63 || (index + 7) % 8 == 7) {
                let is_piece: bool = self.is_piece(index + 7);
                let is_white: bool = self.is_white(index + 7);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index + 7);
                }
            }
            if !((index + 9) > 63 || (index + 9) % 8 == 0) {
                let is_piece: bool = self.is_piece(index + 9);
                let is_white: bool = self.is_white(index + 9);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index + 9);
                }
            }
            if !((index < 9) || (index - 9) % 8 == 7) {
                let is_piece: bool = self.is_piece(index - 9);
                let is_white: bool = self.is_white(index - 9);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index - 9);
                }
            }
            if !((index < 7) || (index - 7) % 8 == 0) {
                let is_piece: bool = self.is_piece(index - 7);
                let is_white: bool = self.is_white(index - 7);
                if !(is_piece && is_white) {
                    self.valid_moves.push(index - 7);
                }
            }
        } else if !self.is_white(index) {
            //vertical & horisontal
            if index > 8 {
                let is_piece: bool = self.is_piece(index - 8);
                let is_white: bool = self.is_white(index - 8);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 8);
                }
            }
            if index >= 1 && (index - 1) / 8 == row {
                let is_piece: bool = self.is_piece(index - 1);
                let is_white: bool = self.is_white(index - 1);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 1);
                }
            }
            if (index + 1) / 8 == row && (index + 1) < 64 {
                let is_piece: bool = self.is_piece(index + 1);
                let is_white: bool = self.is_white(index + 1);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 1);
                }
            }
            if (index + 8) < 64 {
                let is_piece: bool = self.is_piece(index + 8);
                let is_white: bool = self.is_white(index + 8);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 8);
                }
            }

            //diagonal
            if !((index < 7) || (index - 7) % 8 == 0) {
                let is_piece: bool = self.is_piece(index - 7);
                let is_white: bool = self.is_white(index - 7);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 7);
                }
            }
            if !((index < 9) || (index - 9) % 8 == 7) {
                let is_piece: bool = self.is_piece(index - 9);
                let is_white: bool = self.is_white(index - 9);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index - 9);
                }
            }
            if !((index + 9) > 63 || (index + 9) % 8 == 0) {
                let is_piece: bool = self.is_piece(index + 9);
                let is_white: bool = self.is_white(index + 9);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 9);
                }
            }
            if !((index + 7) > 63 || (index + 7) % 8 == 7) {
                let is_piece: bool = self.is_piece(index + 7);
                let is_white: bool = self.is_white(index + 7);
                if !(is_piece && !is_white) {
                    self.valid_moves.push(index + 7);
                }
            }
        }
    }

    fn find_checks(&mut self) {
        self.all_white_moves.clear();
        self.all_black_moves.clear();
        for i in 0..64 {
            if self.is_piece(i) {
                self.get_valid_moves(i);
                if self.whites_turn && !self.is_white(i) {
                    self.all_black_moves.append(&mut self.valid_moves);
                } else if !self.whites_turn && self.is_white(i) {
                    self.all_white_moves.append(&mut self.valid_moves);
                }
            }
        }
        if self.all_black_moves.contains(&self.white_king_position) {
            self.white_in_check = true;
        } else {
            self.white_in_check = false;
        }
        if self.all_white_moves.contains(&self.black_king_position) {
            self.black_in_check = true;
        } else {
            self.black_in_check = false;
        }
    }

    fn clean_valid_moves(&mut self) {
        self.temp_valid_moves = self.valid_moves.clone();
        self.temp_board = self.board;
        self.find_checks();
        self.board = self.temp_board;
        self.valid_moves = self.temp_valid_moves.clone();

        if self.valid_moves.is_empty() {
            return;
        }
        let index: usize;
        if self.has_selected {
            index = self.selected_index;
        } else {
            index = self.current_index;
        }

        if (self.is_piece(index) && self.is_white(index) && !self.whites_turn)
            || (self.is_piece(index) && !self.is_white(index) && self.whites_turn)
        {
            self.valid_moves.clear();
        }

        let mut moves_to_remove: Vec<usize> = vec![];

        if self.piece_type(index) == 6 {
            if self.is_white(index) {
                moves_to_remove.append(&mut self.all_black_moves);
            } else {
                moves_to_remove.append(&mut self.all_white_moves);
            }
        } else {
            for target_index in 0..self.valid_moves.len() {
                self.white_in_check = false;
                self.black_in_check = false;
                self.temp_valid_moves = self.valid_moves.clone();
                self.temp_board = self.board;
                self.board[self.temp_valid_moves[target_index]] = self.board[index];
                self.board[index] = Piece::Empty { position: (index) };
                self.find_checks();
                self.board = self.temp_board;
                self.valid_moves = self.temp_valid_moves.clone();
                if self.whites_turn && self.white_in_check {
                    moves_to_remove.push(self.valid_moves[target_index]);
                } else if !self.whites_turn && self.black_in_check {
                    moves_to_remove.push(self.valid_moves[target_index]);
                }
            }

            self.temp_valid_moves = self.valid_moves.clone();
            self.temp_board = self.board;
            self.find_checks();
            self.board = self.temp_board;
            self.valid_moves = self.temp_valid_moves.clone();
        }

        for val in 0..moves_to_remove.len() {
            self.valid_moves.retain(|&x| x != moves_to_remove[val]);
        }
    }
}

fn create_starting_board() -> [Piece; 64] {
    let mut board: [Piece; 64] = [Piece::Start; 64];
    for i in 0..64 {
        board[i] = Piece::Empty { position: i };
    }
    //white pawns
    for i in 0..8 {
        board[i + 8] = Piece::Piece {
            piece_type: 1,
            white: true,
            position: (i + 8),
            has_moved: false,
        };
    }
    //black pawns
    for i in 0..8 {
        board[55 - (i)] = Piece::Piece {
            piece_type: 1,
            white: false,
            position: (55 - i),
            has_moved: false,
        };
    }
    //white rooks
    board[0] = Piece::Piece {
        piece_type: 2,
        white: true,
        position: 0,
        has_moved: false,
    };
    board[7] = Piece::Piece {
        piece_type: 2,
        white: true,
        position: 7,
        has_moved: false,
    };

    //white knights
    board[1] = Piece::Piece {
        piece_type: 3,
        white: true,
        position: 1,
        has_moved: false,
    };
    board[6] = Piece::Piece {
        piece_type: 3,
        white: true,
        position: 6,
        has_moved: false,
    };

    //white bishops
    board[2] = Piece::Piece {
        piece_type: 4,
        white: true,
        position: 2,
        has_moved: false,
    };
    board[5] = Piece::Piece {
        piece_type: 4,
        white: true,
        position: 5,
        has_moved: false,
    };

    //white queen
    board[4] = Piece::Piece {
        piece_type: 5,
        white: true,
        position: 4,
        has_moved: false,
    };

    //white king
    board[3] = Piece::Piece {
        piece_type: 6,
        white: true,
        position: 3,
        has_moved: false,
    };

    //black rooks
    board[63] = Piece::Piece {
        piece_type: 2,
        white: false,
        position: 63,
        has_moved: false,
    };
    board[56] = Piece::Piece {
        piece_type: 2,
        white: false,
        position: 56,
        has_moved: false,
    };

    //black knights
    board[62] = Piece::Piece {
        piece_type: 3,
        white: false,
        position: 62,
        has_moved: false,
    };
    board[57] = Piece::Piece {
        piece_type: 3,
        white: false,
        position: 57,
        has_moved: false,
    };

    //black bishops
    board[61] = Piece::Piece {
        piece_type: 4,
        white: false,
        position: 61,
        has_moved: false,
    };
    board[58] = Piece::Piece {
        piece_type: 4,
        white: false,
        position: 58,
        has_moved: false,
    };

    //black queen
    board[60] = Piece::Piece {
        piece_type: 5,
        white: false,
        position: 60,
        has_moved: false,
    };

    //black king
    board[59] = Piece::Piece {
        piece_type: 6,
        white: false,
        position: 59,
        has_moved: false,
    };

    return board;
}
