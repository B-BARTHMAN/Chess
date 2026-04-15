use crate::perft::perft::perft_divide;

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
    perft_divide(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
        6,
    );
}
