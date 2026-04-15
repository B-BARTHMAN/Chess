use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::magic_bishop::bishop_attacks;
use crate::bitboard::magic_rook::rook_attacks;
use crate::board::square::Square;
use crate::movegen::precompute::{KING_BB, KNIGHT_BB};
use crate::piece::piece_type::PieceType;

#[inline]
pub fn attacks_bb(pt: PieceType, square: Square, blockers: Bitboard) -> Bitboard {
  debug_assert_ne!(pt, PieceType::Pawn);

  match pt {
    PieceType::Knight => KNIGHT_BB[square as usize],
    PieceType::Bishop => bishop_attacks(square, blockers),
    PieceType::Rook => rook_attacks(square, blockers),
    PieceType::Queen => bishop_attacks(square, blockers) | rook_attacks(square, blockers),
    PieceType::King => KING_BB[square as usize],
    PieceType::Pawn => unreachable!(),
    _ => panic!(),
  }
}