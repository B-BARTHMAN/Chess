use crate::movegen::movegen::gen_evasions;
use crate::moves::movelist::MoveList;
use crate::perft::perft::perft_divide;
use crate::position::position::Position;
use crate::search::Searcher;

mod bitboard;
mod moves;
mod movegen;
mod perft;
mod position;
mod util;
mod piece;
mod board;
mod eval;
mod search;

fn main() {
    let mut pos = Position::from_fen("3rr1k1/pp2qp1p/2n2Bp1/5Q1N/7b/2P2P2/PP2N2P/2K1R1R1 b - - 4 26");
    let searcher: Searcher = Searcher::new();
    let (mv, score) = searcher.search(&mut pos, 6);
    println!("{}, {}", mv, score);
}
