use crate::board::square::Square;
use crate::piece::piece_type::PieceType;
use crate::position::castling::CastlingRights;

// P P M M T T T T T T F F F F F F
#[derive(Copy, Clone)]
pub struct Move(pub u16);
#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
  Normal = 0u16 << 12,
  Promotion = 1u16 << 12,
  EnPassant = 2u16 << 12,
  Castle = 3u16 << 12,
}
#[repr(u16)]
pub enum PromotionType {
  Queen = 0u16 << 14,
  Rook = 1u16 << 14,
  Bishop = 2u16 << 14,
  Knight = 3u16 << 14,
}

impl Move {
  #[inline]
  pub const fn normal(from: Square, to: Square) -> Self {
    Move((from as u16) | ((to as u16) << 6) | (MoveType::Normal as u16))
  }
  #[inline]
  pub const fn promotion(from: Square, to: Square, pt: PromotionType) -> Self {
    let from = from as u16;
    let to = to as u16;
    Move(from | (to << 6) | (MoveType::Promotion as u16) | (pt as u16))
  }
  #[inline]
  pub const fn enpassant(from: Square, to: Square) -> Self {
    let from = from as u16;
    let to = to as u16;
    Move(from | (to << 6) | (MoveType::EnPassant as u16))
  }
  #[inline]
  pub const fn castle(rights: CastlingRights) -> Self {
    match rights {
      CastlingRights::WhiteKingside  => Move((Square::E1 as u16) | ((Square::G1 as u16) << 6) | (MoveType::Castle as u16)),
      CastlingRights::WhiteQueenside => Move((Square::E1 as u16) | ((Square::C1 as u16) << 6) | (MoveType::Castle as u16)),
      CastlingRights::BlackKingside  => Move((Square::E8 as u16) | ((Square::G8 as u16) << 6) | (MoveType::Castle as u16)),
      CastlingRights::BlackQueenside => Move((Square::E8 as u16) | ((Square::C8 as u16) << 6) | (MoveType::Castle as u16)),
      _ => panic!("Weird castling rights encountered")
    }
  }
  
  #[inline]
  pub const fn from(&self) -> Square {
    Square::from_index((self.0 & 0b111111) as i8)
  }
  #[inline]
  pub const fn to(self) -> Square {
    Square::from_index(((self.0 >> 6) & 0b111111) as i8)
  }
  #[inline]
  pub const fn mt(self) -> MoveType {
    match (self.0 >> 12) & 0b11 {
      0b00 => MoveType::Normal,
      0b01 => MoveType::Promotion,
      0b10 => MoveType::EnPassant,
      0b11 => MoveType::Castle,
      _ => unreachable!()
    }
  }
  #[inline]
  pub const fn pt(self) -> PromotionType {
    match (self.0 >> 14) & 0b11 {
      0b00 => PromotionType::Queen,
      0b01 => PromotionType::Rook,
      0b10 => PromotionType::Bishop,
      0b11 => PromotionType::Knight,
      _ => unreachable!()
    }
  }
}

impl PromotionType {
  pub const fn pt(self) -> PieceType {
    match self {
      PromotionType::Queen => PieceType::Queen,
      PromotionType::Rook => PieceType::Rook,
      PromotionType::Bishop => PieceType::Bishop,
      PromotionType::Knight => PieceType::Knight,
    }
  }
}
impl std::fmt::Display for Move {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let from = (self.0 & 0x3F) as u8;
    let to = ((self.0 >> 6) & 0x3F) as u8;

    fn square_to_str(sq: u8) -> String {
      let file = (sq % 8) as u8 + b'A';
      let rank = (sq / 8) as u8 + b'1';
      format!("{}{}", file as char, rank as char)
    }

    write!(f, "{}-{}", square_to_str(from), square_to_str(to))
  }
}