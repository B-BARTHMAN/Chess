use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::masks::{RANK_2, RANK_3, RANK_6, RANK_7};
use crate::bitboard::ops::pop_lsb;
use crate::bitboard::shift::shift_bb;
use crate::board::direction::Direction;
use crate::board::square::Square;
use crate::chess_move::chess_move::{Move, PromotionType};
use crate::chess_move::movelist::MoveList;
use crate::movegen::movegen::MoveGenType;
use crate::movegen::precompute::PAWN_BB;
use crate::piece::color::Color;
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

pub fn generate_pawns<const T: u8>(pos: &Position, target: Bitboard, color: Color, movelist: &mut MoveList) {
  let white = color == Color::White;
  let them = Color::other(color);

  let forward = if white { Direction::North } else { Direction::South };
  let capture_left = if white { Direction::NorthWest } else { Direction::SouthEast };
  let capture_right = if white { Direction::NorthEast } else { Direction::SouthWest };

  let rank3 = if white { RANK_3 } else { RANK_6 };
  let rank7 = if white { RANK_7 } else { RANK_2 };

  let forward_step = if white { 8 } else { -8 };
  let double_step = if white { 16 } else { -16 };
  let capture_left_step = if white { 7 } else { -7 };
  let capture_right_step = if white { 9 } else { -9 };

  let gen_quiet = T != MoveGenType::Capture as u8;
  let gen_capture = T != MoveGenType::Quiet as u8;

  let all_pawns = pos.pieces(PieceType::Pawn, color);
  let enemies = pos.by_color(them);

  let pawns_rank7 = all_pawns & rank7;
  let pawns = all_pawns & !rank7;

  // Precompute shifts
  let forward_moves = shift_bb(pawns, forward);
  let capture_left_moves = shift_bb(pawns, capture_left);
  let capture_right_moves = shift_bb(pawns, capture_right);

  if gen_quiet {
    let single_pushes = forward_moves & target;
    let double_pushes = shift_bb(single_pushes & rank3, forward) & target;

    movelist.fill_pawns(single_pushes, forward_step);
    movelist.fill_pawns(double_pushes, double_step);
  }

  if gen_capture {
    movelist.fill_pawns(capture_left_moves & target, capture_left_step);
    movelist.fill_pawns(capture_right_moves & target, capture_right_step);

    if pos.ep_square() != Square::None {
      let bb = pawns & PAWN_BB[them as usize][pos.ep_square() as usize];
      movelist.fill_enpassants(bb, pos.ep_square());
    }
  }

  if pawns_rank7 != 0 {
    let promotion_forward = shift_bb(pawns_rank7, forward);
    let promotion_capture_left = shift_bb(pawns_rank7, capture_left);
    let promotion_capture_right = shift_bb(pawns_rank7, capture_right);
    movelist.fill_promotions::<T>(promotion_forward & !pos.all_pieces(), forward_step);
    movelist.fill_promotions::<T>(promotion_capture_left & enemies, capture_left_step);
    movelist.fill_promotions::<T>(promotion_capture_right & enemies, capture_right_step);
  }
}

impl MoveList {
  fn fill_pawns(&mut self, mut to_bb: Bitboard, step: i8) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      let from = Square::from_index((to as i8) - step);
      self.push(Move::normal(from, to));
    }
  }
  fn fill_promotions<const T: u8>(&mut self, mut to_bb: Bitboard, step: i8) {
    while to_bb != 0 {
      let to = pop_lsb(&mut to_bb);
      let from = Square::from_index((to as i8) - step);
      if T != (MoveGenType::Quiet as u8) {
        self.push(Move::promotion(from, to, PromotionType::Queen));
      }
      if T != (MoveGenType::Capture as u8) {
        self.push(Move::promotion(from, to, PromotionType::Knight));
        self.push(Move::promotion(from, to, PromotionType::Bishop));
        self.push(Move::promotion(from, to, PromotionType::Rook));
      }
    }
  }
  fn fill_enpassants(&mut self, mut from_bb: Bitboard, to: Square) {
    while from_bb != 0 {
      let sq = pop_lsb(&mut from_bb);
      self.push(Move::enpassant(sq, to));
    }
  }
}