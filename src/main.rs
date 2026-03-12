use crate::bitboard::PrettyBitboard;
use crate::magic_bishop::bishop_attacks;
use crate::magic_rook::rook_attacks;
use crate::movegen::{gen_pawn, generate_moves, MoveGenType, BETWEEN_BB};
use crate::movelist::MoveList;
use crate::position::Position;
use crate::types::{Bitboard, Color, PieceType, Square};

mod position;
mod types;
mod bitboard;
mod movegen;
mod movelist;
mod magic_bishop;
mod magic_rook;

fn main() {
  let pos = Position::from_fen("r1bqkbnr/ppp1pppp/n7/2PpP3/8/8/PP1P1PPP/RNBQKBNR w KQkq d6 0 5");
  let mut movelist = MoveList::new();

  let target = pos.enemies::<{ Color::White as u8 }>();

  generate_moves::<{ PieceType::Rook as u8 }, { Color::White as u8 }>(&pos, target, &mut movelist);
  generate_moves::<{ PieceType::Knight as u8 }, { Color::White as u8 }>(&pos, target, &mut movelist);
  generate_moves::<{ PieceType::Bishop as u8 }, { Color::White as u8 }>(&pos, target, &mut movelist);
  generate_moves::<{ PieceType::Queen as u8 }, { Color::White as u8 }>(&pos, target, &mut movelist);
  gen_pawn::<{ Color::White as u8 },{ MoveGenType::Capture as u8 }>(&pos, target, &mut movelist);

  for m in movelist.iter() {
    println!("{}", m);
  }

}
