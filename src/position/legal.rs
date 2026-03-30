use crate::bitboard::ops::lsb;
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

impl Position {
    pub fn is_legal(&self) -> bool {
        self.attackers_to(
            lsb(self.pieces(PieceType::King, self.side_to_move.other())),
            self.side_to_move,
        ) == 0
    }
}
