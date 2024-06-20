use std::collections::HashMap;

/*
    HashMap containing legal moves is structured as follows:
        key sets the move direction:
            1, North (opposite opponent)
            2, East
            3, West 
            4, South 
            5, North East
            6, North West 
            7, South East 
            8, South West 

            { Knight specific keys:

            9, North North East
            10, North North West 

            11, East North East
            12, East South East

            13, West North West
            14, West South West

            15, South South East 
            16, South South West }

        Value Meaning:
            x = maximum amount of moves in the Keys direction 
        

 */

pub fn get_core_moves() -> Vec<HashMap<String, usize>> {
    let mut core_moves: Vec<HashMap<String, usize>> = vec![];
    // contains the maps each containing the core moves for a pieve 
    // [0]pawn, [1]rook, [2]knight, [3]bishop, [4]queen, [5]king

    //pawn
    let mut temp_pawn_map: HashMap<String, usize> = HashMap::new();
    temp_pawn_map.insert("north".to_string(), 2);
    temp_pawn_map.insert("north_east".to_string(), 1);
    temp_pawn_map.insert("north_west".to_string(), 1);
    let pawn_map = temp_pawn_map;
    core_moves.push(pawn_map);

    //rook
    let mut temp_rook_map: HashMap<String, usize> = HashMap::new();
    temp_rook_map.insert("north".to_string(), 7);
    temp_rook_map.insert("east".to_string(), 7);
    temp_rook_map.insert("west".to_string(), 7);
    temp_rook_map.insert("south".to_string(), 7);
    let rook_map = temp_rook_map;
    core_moves.push(rook_map);

    //knight
    let mut temp_knight_map: HashMap<String, usize> = HashMap::new();
    temp_knight_map.insert("north_north_east".to_string(), 1);
    temp_knight_map.insert("north_north_west".to_string(), 1);

    temp_knight_map.insert("east_north_east".to_string(), 1);
    temp_knight_map.insert("wets_north_west".to_string(), 1);

    temp_knight_map.insert("east_south_east".to_string(), 1);
    temp_knight_map.insert("west_south_west".to_string(), 1);

    temp_knight_map.insert("south_south_east".to_string(), 1);
    temp_knight_map.insert("south_south_west".to_string(), 1);
 
    let knight_map = temp_knight_map;
    core_moves.push(knight_map);

    //bishop
    let mut temp_bishop_map: HashMap<String, usize> = HashMap::new();
    temp_bishop_map.insert("north_east".to_string(), 7);
    temp_bishop_map.insert("north_west".to_string(), 7);
    temp_bishop_map.insert("south_east".to_string(), 7);
    temp_bishop_map.insert("south_west".to_string(), 7);
    let bishop_map = temp_bishop_map;
    core_moves.push(bishop_map);

    //queen
    let mut temp_queen_map: HashMap<String, usize> = HashMap::new();
    temp_queen_map.insert("north".to_string(), 7);
    temp_queen_map.insert("east".to_string(), 7);
    temp_queen_map.insert("west".to_string(), 7);
    temp_queen_map.insert("south".to_string(), 7);
    temp_queen_map.insert("north_east".to_string(), 7);
    temp_queen_map.insert("north_west".to_string(), 7);
    temp_queen_map.insert("south_east".to_string(), 7);
    temp_queen_map.insert("south_west".to_string(), 7);
    let queen_map = temp_queen_map;
    core_moves.push(queen_map);

    //king
    let mut temp_king_map: HashMap<String, usize> = HashMap::new();
    temp_king_map.insert("north".to_string(), 1);
    temp_king_map.insert("east".to_string(), 1);
    temp_king_map.insert("west".to_string(), 1);
    temp_king_map.insert("south".to_string(), 1);
    temp_king_map.insert("north_east".to_string(), 1);
    temp_king_map.insert("north_west".to_string(), 1);
    temp_king_map.insert("south_east".to_string(), 1);
    temp_king_map.insert("south_west".to_string(), 1);
    let king_map = temp_king_map;
    core_moves.push(king_map);
    return core_moves;
}

#[derive(Copy, Clone)]
pub enum Piece {
    Pawn{
        //legal_moves: HashMap<usize, usize>,
        white: bool,
        position: [usize;2],
        has_moved: bool,
    },
    Rook{
        //legal_moves: HashMap<usize, usize>,
        white: bool,
        position: [usize;2],
        has_moved: bool,
    },
    Knight{
        //legal_moves: HashMap<usize, usize>,
        white: bool,
        position: [usize;2],
        has_moved: bool,
    },
    Bishop{
        //legal_moves: HashMap<usize, usize>,
        white: bool,
        position: [usize;2],
        has_moved: bool,
    },
    Queen{
        //legal_moves: HashMap<usize, usize>,
        white: bool,
        position: [usize;2],
        has_moved: bool,
    },
    King{
        //legal_moves: HashMap<usize, usize>,
        white: bool,
        position: [usize;2],
        has_moved: bool,
    },
    Empty,
}
