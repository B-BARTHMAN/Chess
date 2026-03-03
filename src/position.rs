use crate::types::{Color, Piece, PieceType, Square, Bitboard};
pub struct Position {
    board: [Piece; Square::SquareNb as usize],
    by_type: [Bitboard; PieceType::PieceTypeNb as usize],
    by_color: [Bitboard; Color::ColorNb as usize],
}

impl Position {
    #[inline(always)]
    pub fn pieces<const PT: u8, const US: u8>(&self) -> Bitboard {
        self.by_type[PT as usize] & self.by_color[US as usize]
    }
}