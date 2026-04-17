use crate::eval::Evaluator;
use crate::eval::psqt::PIECE_SQUARE_VALUE;
use crate::board::square::{Square, SQUARE_COUNT};
use crate::piece::color::Color;
use crate::piece::piece::Piece;
use crate::position::position::Position;

pub struct PSQTEvaluator;

impl Evaluator for PSQTEvaluator {
  fn eval(&self, pos: &Position) -> i32 {
    let mut score = 0;
    for sq_idx in 0..(SQUARE_COUNT as i8) {
      let sq = Square::from_index(sq_idx);
      let piece = pos.by_square[sq];
      if piece == Piece::NoPiece { continue; }
      score += PIECE_SQUARE_VALUE[piece as usize][sq_idx as usize];
    }
    if pos.side_to_move == Color::Black { -score } else { score }
  }
}