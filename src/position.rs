use crate::types::{Color, Piece, PieceType, Square, Bitboard};
struct Position {
    board: [Piece; Square::SquareNb as usize],
    by_type: [Bitboard; PieceType::PieceTypeNb as usize],
    by_color: [Bitboard; Color::ColorNb as usize],
}

impl Position {
    fn pieces(&self, us: Color, pt: PieceType) -> Bitboard {
        return self.by_type[pt as usize] & self.by_color[us as usize];
    }
}