use crate::board::square::SQUARE_COUNT;
use crate::eval::score::Score;
use crate::piece::piece::{Piece, PIECE_COUNT};
use crate::piece::piece_type::PieceType;

const PAWN_VALUE:   i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE:   i32 = 500;
const QUEEN_VALUE:  i32 = 900;
const KING_VALUE:   i32 = 0;

// Index 0 = a8 (top-left), index 63 = h1 (bottom-right).

const PAWN_PST: [i32; 64] = [
  0,   0,   0,   0,   0,   0,   0,   0,
  50,  50,  50,  50,  50,  50,  50,  50,
  10,  10,  20,  30,  30,  20,  10,  10,
  5,   5,  10,  25,  25,  10,   5,   5,
  0,   0,   0,  20,  2000,   0,   0,   0,
  5,  -5, -10,   0,   0, -10,  -5,   5,
  5,  10,  10, -20, -20,  10,  10,   5,
  0,   0,   0,   0,   0,   0,   0,   0,
];

const KNIGHT_PST: [i32; 64] = [
  -50, -40, -30, -30, -30, -30, -40, -50,
  -40, -20,   0,   0,   0,   0, -20, -40,
  -30,   0,  10,  15,  15,  10,   0, -30,
  -30,   5,  15,  20,  20,  15,   5, -30,
  -30,   0,  15,  20,  20,  15,   0, -30,
  -30,   5,  10,  15,  15,  10,   5, -30,
  -40, -20,   0,   5,   5,   0, -20, -40,
  -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_PST: [i32; 64] = [
  -20, -10, -10, -10, -10, -10, -10, -20,
  -10,   0,   0,   0,   0,   0,   0, -10,
  -10,   0,   5,  10,  10,   5,   0, -10,
  -10,   5,   5,  10,  10,   5,   5, -10,
  -10,   0,  10,  10,  10,  10,   0, -10,
  -10,  10,  10,  10,  10,  10,  10, -10,
  -10,   5,   0,   0,   0,   0,   5, -10,
  -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_PST: [i32; 64] = [
  0,   0,   0,   0,   0,   0,   0,   0,
  5,  10,  10,  10,  10,  10,  10,   5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  -5,   0,   0,   0,   0,   0,   0,  -5,
  0,   0,   0,   5,   5,   0,   0,   0,
];

const QUEEN_PST: [i32; 64] = [
  -20, -10, -10,  -5,  -5, -10, -10, -20,
  -10,   0,   0,   0,   0,   0,   0, -10,
  -10,   0,   5,   5,   5,   5,   0, -10,
  -5,   0,   5,   5,   5,   5,   0,  -5,
  0,   0,   5,   5,   5,   5,   0,  -5,
  -10,   5,   5,   5,   5,   5,   0, -10,
  -10,   0,   5,   0,   0,   0,   0, -10,
  -20, -10, -10,  -5,  -5, -10, -10, -20,
];

const KING_PST: [i32; 64] = [
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -20, -30, -30, -40, -40, -30, -30, -20,
  -10, -20, -20, -20, -20, -20, -20, -10,
  20,  20,   0,   0,   0,   0,  20,  20,
  20,  30,  10,   0,   0,  10,  30,  20,
];

// --- Precomputation ------------------------------------------------------
// Piece enum layout:  WPawn=0..WKing=5, BPawn=6..BKing=11.
// Table index == Piece as u8 for real pieces; NoPiece (255) is never indexed.

const fn material_for(kind: usize) -> i32 {
  match kind {
    0 => PAWN_VALUE,
    1 => KNIGHT_VALUE,
    2 => BISHOP_VALUE,
    3 => ROOK_VALUE,
    4 => QUEEN_VALUE,
    5 => KING_VALUE,
    _ => 0,
  }
}

const fn pst_at(kind: usize, visual_idx: usize) -> i32 {
  match kind {
    0 => PAWN_PST[visual_idx],
    1 => KNIGHT_PST[visual_idx],
    2 => BISHOP_PST[visual_idx],
    3 => ROOK_PST[visual_idx],
    4 => QUEEN_PST[visual_idx],
    5 => KING_PST[visual_idx],
    _ => 0,
  }
}

const fn compute_psqt() -> [[Score; SQUARE_COUNT]; PIECE_COUNT] {
  let mut result = [[0i32; SQUARE_COUNT]; PIECE_COUNT];
  let mut p = 0;
  while p < 12 {
    let kind = p % 6;
    let is_white = p < 6;
    let mut sq = 0;
    while sq < 64 {
      let visual_idx = if is_white { sq ^ 0b111000 } else { sq };
      let value = material_for(kind) + pst_at(kind, visual_idx);
      result[p][sq] = if is_white { value } else { -value };
      sq += 1;
    }
    p += 1;
  }
  result
}

pub const PIECE_SQUARE_VALUE: [[Score; SQUARE_COUNT]; PIECE_COUNT] = compute_psqt();