use crate::bitboard::bitboard::Bitboard;
use crate::board::square::Square;
use crate::movegen::precompute::KING_BB;
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::piece::color::Color;
use crate::position::castling::CastlingRights;
use crate::position::position::Position;

pub fn gen_king(ksq: Square, target: Bitboard, move_list: &mut MoveList) {
  move_list.fill_normal(ksq, KING_BB[ksq as usize] & target);
}
pub fn gen_castles(pos: &Position, move_list: &mut MoveList) {
  let (king_side, queen_side) = match pos.side_to_move {
    Color::White => (CastlingRights::WhiteKingside, CastlingRights::WhiteQueenside),
    Color::Black => (CastlingRights::BlackKingside, CastlingRights::BlackQueenside),
  };

  if pos.can_castle(king_side) {move_list.push(Move::castle(king_side)) }
  if pos.can_castle(queen_side) {move_list.push(Move::castle(queen_side)) }
}