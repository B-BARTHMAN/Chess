use crate::piece::color::Color;
use crate::piece::piece_type::PieceType;
use crate::util::indexable::Indexable;

pub const PIECE_COUNT: usize = 12;
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Piece {
  NoPiece = 255,
  WPawn = 0,
  WKnight,
  WBishop,
  WRook,
  WQueen,
  WKing,
  BPawn,
  BKnight,
  BBishop,
  BRook,
  BQueen,
  BKing,
}

impl Indexable for Piece {
  fn idx(self) -> usize {
    self as usize
  }
}

impl Piece {
  #[inline]
  pub const fn new(color: Color, piece_type: PieceType) -> Self {
    match color {
      Color::White => {
        match piece_type {
          PieceType::Pawn => Piece::WPawn,
          PieceType::Knight => Piece::WKnight,
          PieceType::Bishop => Piece::WBishop,
          PieceType::Rook => Piece::WRook,
          PieceType::Queen => Piece::WQueen,
          PieceType::King => Piece::WKing,
          _ => Piece::NoPiece,
        }
      }
      Color::Black => {
        match piece_type {
          PieceType::Pawn => Piece::BPawn,
          PieceType::Knight => Piece::BKnight,
          PieceType::Bishop => Piece::BBishop,
          PieceType::Rook => Piece::BRook,
          PieceType::Queen => Piece::BQueen,
          PieceType::King => Piece::BKing,
          _ => Piece::NoPiece,
        }
      }
    }
  }
  #[inline]
  pub const fn piece_type(&self) -> PieceType {
    match self {
      Piece::NoPiece => PieceType::None,
      Piece::WPawn => PieceType::Pawn,
      Piece::WKnight => PieceType::Knight,
      Piece::WBishop => PieceType::Bishop,
      Piece::WRook => PieceType::Rook,
      Piece::WQueen => PieceType::Queen,
      Piece::WKing => PieceType::King,
      Piece::BPawn => PieceType::Pawn,
      Piece::BKnight => PieceType::Knight,
      Piece::BBishop => PieceType::Bishop,
      Piece::BRook => PieceType::Rook,
      Piece::BQueen => PieceType::Queen,
      Piece::BKing => PieceType::King,
    }
  }
  #[inline]
  pub const fn color(&self) -> Color {
    match self {
      Piece::NoPiece => panic!("No piece has no color"),
      Piece::WPawn => Color::White,
      Piece::WKnight => Color::White,
      Piece::WBishop => Color::White,
      Piece::WRook => Color::White,
      Piece::WQueen => Color::White,
      Piece::WKing => Color::White,
      Piece::BPawn => Color::Black,
      Piece::BKnight => Color::Black,
      Piece::BBishop => Color::Black,
      Piece::BRook => Color::Black,
      Piece::BQueen => Color::Black,
      Piece::BKing => Color::Black,
    }
  }
  #[inline]
  pub fn from_fen(fen: char) -> Self {
    debug_assert!("pPnNbBrRqQkK".contains(fen));
    match fen {
      'p' => Piece::BPawn,
      'P' => Piece::WPawn,
      'n' => Piece::BKnight,
      'N' => Piece::WKnight,
      'b' => Piece::BBishop,
      'B' => Piece::WBishop,
      'r' => Piece::BRook,
      'R' => Piece::WRook,
      'q' => Piece::BQueen,
      'Q' => Piece::WQueen,
      'k' => Piece::BKing,
      'K' => Piece::WKing,
      _ => unreachable!(),
    }
  }
}