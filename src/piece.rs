

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
