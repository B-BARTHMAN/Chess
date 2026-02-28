use crate::types::{Piece, PieceType};
struct Position {
    board: [Piece; 64],
    by_type: [u64; PieceType::PieceTypeNB as usize],

}