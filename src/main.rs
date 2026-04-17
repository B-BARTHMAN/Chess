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
mod uci;

fn main() -> std::io::Result<()> {
  uci::run()
}