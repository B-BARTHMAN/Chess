use crate::moves::movelist::MoveList;
use crate::movegen::movegen::{MoveGenType, move_gen};
use crate::perft::perft::{perft, perft_divide};
use piece::piece_type::PieceType;
use crate::position::position::Position;

mod bitboard;
mod moves;
mod movegen;
mod perft;
mod position;
mod util;
mod piece;
mod board;

fn main() {
    perft_divide(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
        5,
    );
    //println!("total: {}", total);
}
