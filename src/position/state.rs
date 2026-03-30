use crate::board::square::Square;
use crate::position::castling::CastlingRights;
use crate::piece::piece::Piece;

pub struct State {
    pub ep_square: Square,
    pub castling_rights: CastlingRights,
    pub captured: Option<Piece>,
}
