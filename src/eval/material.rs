use crate::bitboard::ops::count_ones;
use crate::eval::Evaluator;
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

pub struct MaterialEvaluator;

impl Evaluator for MaterialEvaluator {
  fn eval(&self, pos: &Position) -> i32 {
    let us = pos.side_to_move;
    let them = us.other();

      1 * (count_ones(pos.pieces(PieceType::Pawn, us)) - count_ones(pos.pieces(PieceType::Pawn, them))) +
      3 * (count_ones(pos.pieces(PieceType::Knight, us)) - count_ones(pos.pieces(PieceType::Knight, them))) +
      3 * (count_ones(pos.pieces(PieceType::Bishop, us)) - count_ones(pos.pieces(PieceType::Bishop, them))) +
      5 * (count_ones(pos.pieces(PieceType::Rook, us)) - count_ones(pos.pieces(PieceType::Rook, them))) +
      9 * (count_ones(pos.pieces(PieceType::Queen, us)) - count_ones(pos.pieces(PieceType::Queen, them)))
  }
}