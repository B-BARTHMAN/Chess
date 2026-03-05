use crate::bitboard::PrettyBitboard;
use crate::magic_bishop::bishop_attacks;
use crate::magic_rook::rook_attacks;
use crate::types::Square;

mod position;
mod types;
mod bitboard;
mod movegen;
mod movelist;
mod magic_bishop;
mod magic_rook;

fn main() {
    let blockers = 0b1000000000001000000000u64;
    let attacks = bishop_attacks(Square::C3, blockers);
    let attacks2 = rook_attacks(Square::C3, blockers);
}
