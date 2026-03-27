use crate::chess_move::movelist::MoveList;
use crate::movegen::movegen::{move_gen, MoveGenType};
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

mod board;
mod chess_move;
mod piece;
mod bitboard;
mod position;
mod movegen;

fn main() {
  let mut pos = Position::from_fen("rnbqkbn1/ppppp1p1/8/4KpPr/8/N3P3/PPPP1P1P/R1B2BNR w q f6 0 10");
  let mut movelist: MoveList = MoveList::new();

  move_gen::<{MoveGenType::Capture as u8}>(&pos, &mut movelist);
  move_gen::<{MoveGenType::Quiet as u8}>(&pos, &mut movelist);
  for m in movelist.iter() {

    pos.do_move(*m);

    print!("{}", m);
    if pos.is_legal() {
      println!(": LEGAL");
    } else {
      println!(": INVALID");
    }

    pos.undo_move(*m);

  }
}
