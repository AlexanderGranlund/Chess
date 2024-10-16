use std::collections::HashMap;

/*

   Integer sets the move direction:
       1, North (opposite opponent)
       2, East
       3, Diagonal North East
       4, Diagonal South East
       5, North North East
       6, North East East
       7, North North West
       8, South West West

*/

pub fn get_core_moves() -> Vec<Vec<usize>> {
    let mut core_moves: Vec<Vec<usize>> = vec![];

    //pawn
    let mut temp_pawn_vec: Vec<usize> = vec![];
    temp_pawn_vec.push(1);
    temp_pawn_vec.push(3);
    temp_pawn_vec.push(4);
    core_moves.push(temp_pawn_vec);

    //rook
    let mut temp_rook_vec: Vec<usize> = vec![];
    temp_rook_vec.push(1);
    temp_rook_vec.push(2);
    core_moves.push(temp_rook_vec);

    //knight
    let mut temp_knight_vec: Vec<usize> = vec![];
    temp_knight_vec.push(5);
    temp_knight_vec.push(6);
    temp_knight_vec.push(7);
    temp_knight_vec.push(8);
    core_moves.push(temp_knight_vec);

    //bishop
    let mut temp_bishop_vec: Vec<usize> = vec![];
    temp_bishop_vec.push(3);
    temp_bishop_vec.push(4);
    core_moves.push(temp_bishop_vec);

    //queen
    let mut temp_queen_vec: Vec<usize> = vec![];
    temp_queen_vec.push(1);
    temp_queen_vec.push(2);
    temp_queen_vec.push(3);
    temp_queen_vec.push(4);
    core_moves.push(temp_queen_vec);

    //king
    let mut temp_king_vec: Vec<usize> = vec![];
    temp_king_vec.push(1);
    temp_king_vec.push(2);
    temp_king_vec.push(3);
    temp_king_vec.push(4);
    core_moves.push(temp_king_vec);

    return core_moves;
}

#[derive(Copy, Clone)]
pub enum Piece {
    Piece {
        piece_type: usize, //pawn = 1, rook = 2, knight = 3, bishop = 4, queen = 5, king = 6
        white: bool,
        position: usize,
        has_moved: bool,
    },
    Empty {
        position: usize,
    },
    Start,
}
