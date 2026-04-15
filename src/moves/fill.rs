use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::ops::pop_lsb;
use crate::bitboard::shift::shift_square;
use crate::board::direction::Direction;
use crate::board::square::Square;
use crate::moves::chess_move::{Move};
use crate::moves::movelist::MoveList;
use crate::moves::promotion_type::PromotionType;

impl MoveList {
  pub fn fill_normal(&mut self, from: Square, mut to_bb: Bitboard) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      self.push(Move::normal(from, to));
    }
  }
  pub fn fill_pawns(&mut self, mut to_bb: Bitboard, dir: Direction) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      let from = shift_square(to, dir.opposite());
      self.push(Move::normal(from, to));
    }
  }
  pub fn fill_double_pawns(&mut self, mut to_bb: Bitboard, dir: Direction) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      let from = shift_square(shift_square(to, dir.opposite()), dir.opposite());
      self.push(Move::normal(from, to));
    }
  }
  pub fn fill_queen_promotions(&mut self, mut to_bb: Bitboard, dir: Direction) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      let from = shift_square(to, dir.opposite());
      self.push(Move::promotion(from, to, PromotionType::Queen))
    }
  }
  pub fn fill_under_promotions(&mut self, mut to_bb: Bitboard, dir: Direction) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      let from = shift_square(to, dir.opposite());
      self.push(Move::promotion(from, to, PromotionType::Knight));
      self.push(Move::promotion(from, to, PromotionType::Bishop));
      self.push(Move::promotion(from, to, PromotionType::Rook));
    }
  }
  pub fn fill_enpassants(&mut self, mut from_bb: Bitboard, to: Square) {
    while from_bb != 0 {
      let sq = pop_lsb(&mut from_bb);
      self.push(Move::enpassant(sq, to));
    }
  }
}