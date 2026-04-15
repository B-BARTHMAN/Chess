use crate::piece::piece_type::PieceType;

#[repr(u16)]
pub enum PromotionType {
  Queen  = 0u16 << 14,
  Rook   = 1u16 << 14,
  Bishop = 2u16 << 14,
  Knight = 3u16 << 14,
}

impl PromotionType {
  pub const fn piece_type(self) -> PieceType {
    match self {
      PromotionType::Queen  => PieceType::Queen,
      PromotionType::Rook   => PieceType::Rook,
      PromotionType::Bishop => PieceType::Bishop,
      PromotionType::Knight => PieceType::Knight,
    }
  }
}