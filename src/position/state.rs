use crate::board::square::Square;
use crate::piece::piece_type::PieceType;
use crate::position::castling::CastlingRights;

pub struct State {
  pub ep_square: Square,
  pub castling_rights: CastlingRights,
  pub captured: Option<PieceType>,
}