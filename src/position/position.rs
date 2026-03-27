use crate::movegen::normal::attacks_bb;
use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::masks::square_bb;
use crate::bitboard::ops::{lsb, pop_lsb};
use crate::board::square::Square;
use crate::chess_move::chess_move::Move;
use crate::movegen::precompute::PAWN_BB;
use crate::piece::color::{Color, COLOR_COUNT};
use crate::piece::piece_type::{PieceType, PIECE_TYPES, PIECE_TYPE_COUNT};
use crate::position::castling::{CastlingRights, BLACK_KINGSIDE, BLACK_KINGSIDE_PATH, BLACK_QUEENSIDE, BLACK_QUEENSIDE_PATH, WHITE_KINGSIDE, WHITE_KINGSIDE_PATH, WHITE_QUEENSIDE, WHITE_QUEENSIDE_PATH};
use crate::position::state::State;

pub struct Position {
  pub by_type: [Bitboard; PIECE_TYPE_COUNT],
  pub by_color: [Bitboard; COLOR_COUNT],
  pub states: Vec<State>,
  pub side_to_move: Color,

}

impl Position {
  #[inline]
  pub const fn pieces(&self, pt: PieceType, c: Color) -> Bitboard {
    self.by_type[pt as usize] & self.by_color[c as usize]
  }
  #[inline]
  pub const fn all_pieces(&self) -> Bitboard {
    self.by_color[Color::White as usize] | self.by_color[Color::Black as usize]
  }
  #[inline]
  pub const fn by_color(&self, c: Color) -> Bitboard {
    self.by_color[c as usize]
  }
  #[inline]
  pub const fn enemies(&self, c: Color) -> Bitboard {
    self.by_color[1 - c as usize]
  }
  #[inline]
  pub const fn moved_piece(&self, m: Move) -> PieceType {
    let from_bb = square_bb(m.from());

    let mut i = 0;
    while i < 6 {
      if self.by_type[i] & from_bb != 0 {
        return PIECE_TYPES[i];
      }
      i += 1;
    }

    panic!()
  }
  #[inline]
  pub fn state(&self) -> &State { self.states.last().unwrap() }
  #[inline]
  pub fn ep_square(&self) -> Square {
    self.state().ep_square
  }
  
  #[inline]
  pub fn can_castle(&self, rights: CastlingRights) -> bool {
    self.castle_allowed(rights) && !self.castle_blocked(rights) && !self.castle_attacked(rights)
  }
  #[inline]
  fn castle_allowed(&self, rights: CastlingRights) -> bool {
    self.state().castling_rights & rights != CastlingRights::empty()
  }
  #[inline]
  const fn castle_blocked(&self, rights: CastlingRights) -> bool {
    let pieces = self.all_pieces();
    match rights {
      CastlingRights::WhiteKingside => {(WHITE_KINGSIDE_PATH & pieces) != 0},
      CastlingRights::WhiteQueenside => {(WHITE_QUEENSIDE_PATH & pieces) != 0},
      CastlingRights::BlackKingside => {(BLACK_KINGSIDE_PATH & pieces) != 0},
      CastlingRights::BlackQueenside => {(BLACK_QUEENSIDE_PATH & pieces) != 0},
      _ => panic!("Encountered weird castling rights type")
    }
  }
  #[inline]
  fn castle_attacked(&self, rights: CastlingRights) -> bool {
    let enemy = Color::other(self.side_to_move);
    let mut castle_squares = match rights {
      CastlingRights::WhiteKingside => WHITE_KINGSIDE,
      CastlingRights::WhiteQueenside => WHITE_QUEENSIDE,
      CastlingRights::BlackKingside => BLACK_KINGSIDE,
      CastlingRights::BlackQueenside => BLACK_QUEENSIDE,
      _ => panic!("Encountered weird castling rights type")
    };
    while castle_squares != 0 {
      let sq = pop_lsb(&mut castle_squares);
      if self.attackers_to(sq, enemy) != 0 { return true; }
    }
    false
  }
  #[inline]
  pub fn attackers_to(&self, sq: Square, enemy: Color) -> Bitboard {
    let pieces = self.all_pieces();
      (attacks_bb(PieceType::Rook, sq, pieces) & (self.pieces(PieceType::Rook, enemy) | self.pieces(PieceType::Queen, enemy))) |
        (attacks_bb(PieceType::Bishop, sq, pieces) & (self.pieces(PieceType::Bishop, enemy) | self.pieces(PieceType::Queen, enemy))) |
        (attacks_bb(PieceType::King, sq, pieces) & self.pieces(PieceType::King, enemy)) |
        (attacks_bb(PieceType::Knight, sq, pieces) & self.pieces(PieceType::Knight, enemy)) |
        (PAWN_BB[Color::other(enemy) as usize][sq as usize] & self.pieces(PieceType::Pawn, enemy))
  }

  #[inline]
  pub fn checkers(&self, us: Color) -> Bitboard {
    let kqs = lsb(self.pieces(PieceType::King, us));
    self.attackers_to(kqs, Color::other(us))
  }
}