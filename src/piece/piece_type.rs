pub const PIECE_TYPE_COUNT: usize = 6;
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceType {
  None = 255,
  Pawn = 0,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

pub const PIECE_TYPES: [PieceType; 6] = [
  PieceType::Pawn,
  PieceType::Knight,
  PieceType::Bishop,
  PieceType::Rook,
  PieceType::Queen,
  PieceType::King,
];

impl PieceType {
  #[inline]
  pub fn from_fen(fen: char) -> Self {
    debug_assert!("pPnNbBrRqQkK".contains(fen));
    match fen {
      'p' | 'P' => PieceType::Pawn,
      'n' | 'N' => PieceType::Knight,
      'b' | 'B' => PieceType::Bishop,
      'r' | 'R' => PieceType::Rook,
      'q' | 'Q' => PieceType::Queen,
      'k' | 'K' => PieceType::King,
      _ => PieceType::None,
    }
  }
}