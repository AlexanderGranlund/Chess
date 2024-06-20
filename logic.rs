use crate::piece::Piece;
use crate::piece::get_core_moves;

use std::collections::HashMap;

pub struct Logic{
    height: usize,
    width: usize,
    moves: usize,
    pub board: [Piece;64],
    current_move: [usize;2],
    core_moves: Vec<HashMap<String, usize>>
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
        }
    }
    pub fn increment_moves(&mut self){
        self.moves += 1;
    }

}

pub fn is_valid_move(board: &[Option<Piece>;64], current_move: [usize;2]) -> bool{
    return true;
}

fn create_starting_board() -> [Piece;64]{
    let mut board: [Piece;64] = [Piece::Empty;64];
    //white pawns
    for i in 0..8{
        board[i+8] = Piece::Pawn {/*legal_moves: HashMap::new(),*/ white: true, position: [i,1], has_moved: false};
    }
    //black pawns
    for i in 0..8{
        board[55-(i)] = Piece::Pawn {/*legal_moves: HashMap::new(),*/ white: false, position: [i,6], has_moved: false};
    }
    //white rooks
    board[0] =  Piece::Rook {/*legal_moves: HashMap::new(),*/ white: true, position: [0,0], has_moved: false};
    board[7] =  Piece::Rook {/*legal_moves: HashMap::new(),*/ white: true, position: [7,0], has_moved: false};

    //white knights 
    board[1] =  Piece::Knight {/*legal_moves: HashMap::new(),*/ white: true, position: [1,0], has_moved: false};
    board[6] =  Piece::Knight {/*legal_moves: HashMap::new(),*/ white: true, position: [6,0], has_moved: false};

    //white bishops
    board[2] =  Piece::Bishop {/*legal_moves: HashMap::new(),*/ white: true, position: [2,0], has_moved: false};
    board[5] =  Piece::Bishop {/*legal_moves: HashMap::new(),*/ white: true, position: [5,0], has_moved: false};

    //white queen
    board[3] =  Piece::Queen {/*legal_moves: HashMap::new(),*/ white: true, position: [3,0], has_moved: false};
    
    //white king
    board[4] =  Piece::King {/*legal_moves: HashMap::new(),*/ white: true, position: [4,0], has_moved: false};

    //black rooks
    board[63] =  Piece::Rook {/*legal_moves: HashMap::new(),*/ white: false, position: [63,7], has_moved: false};
    board[56] =  Piece::Rook {/*legal_moves: HashMap::new(),*/ white: false, position: [56,7], has_moved: false};

    //black knights
    board[62] =  Piece::Knight {/*legal_moves: HashMap::new(),*/ white: false, position: [62,7], has_moved: false};
    board[57] =  Piece::Knight {/*legal_moves: HashMap::new(),*/ white: false, position: [57,7], has_moved: false};

    //black bishops
    board[61] =  Piece::Bishop {/*legal_moves: HashMap::new(),*/ white: false, position: [61,7], has_moved: false};
    board[58] =  Piece::Bishop {/*legal_moves: HashMap::new(),*/ white: false, position: [58,7], has_moved: false};

    //black queen
    board[59] =  Piece::Queen {/*legal_moves: HashMap::new(),*/ white: false, position: [59,7], has_moved: false};
    
    //black king
    board[60] =  Piece::Knight {/*legal_moves: HashMap::new(),*/ white: false, position: [60,7], has_moved: false};

    return board;
}

pub fn get_position_on_board(position_array: &[usize;2]) -> usize{
    let board_positiion = position_array[0] + (position_array[1] * 8);
    if board_positiion < 0 || board_positiion > 63{
        return 100;
    }
    return board_positiion;
}