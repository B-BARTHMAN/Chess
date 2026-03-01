use crate::types::{Bitboard, Square, Direction};

const A_FILE: Bitboard = 0x0101010101010101u64;
const B_FILE: Bitboard = A_FILE << 1;
const C_FILE: Bitboard = A_FILE << 2;
const D_FILE: Bitboard = A_FILE << 3;
const E_FILE: Bitboard = A_FILE << 4;
const F_FILE: Bitboard = A_FILE << 5;
const G_FILE: Bitboard = A_FILE << 6;
const H_FILE: Bitboard = A_FILE << 7;

const RANK_1: Bitboard = 0xFFu64;
const RANK_2: Bitboard = RANK_1 << (8 * 1);
const RANK_3: Bitboard = RANK_1 << (8 * 2);
const RANK_4: Bitboard = RANK_1 << (8 * 3);
const RANK_5: Bitboard = RANK_1 << (8 * 4);
const RANK_6: Bitboard = RANK_1 << (8 * 5);
const RANK_7: Bitboard = RANK_1 << (8 * 6);
const RANK_8: Bitboard = RANK_1 << (8 * 7);

pub fn lsb(bb: Bitboard) -> Square {
    return if bb == 0 {
        Square::SquareNone
    } else {
        Square::from_index(bb.trailing_zeros() as i8)
    }
}

pub fn pop_lsb(bb: &mut Bitboard) -> Square {
    let square = lsb(*bb);
    *bb &= *bb - 1;
    return square
}
pub const fn square_bb(square: Square) -> Bitboard {
    return 1u64 << (square as usize)
}
pub const fn shift_bb(bb: Bitboard, d: Direction) -> Bitboard {
    match d {
        Direction::North => bb << 8,
        Direction::South => bb >> 8,
        Direction::East => (bb & !H_FILE) << 1,
        Direction::West => (bb & !A_FILE) >> 1,
        Direction::NorthEast => (bb & !H_FILE) << 9,
        Direction::NorthWest => (bb & !A_FILE) << 7,
        Direction::SouthEast => (bb & !H_FILE) >> 7,
        Direction::SouthWest => (bb & !A_FILE) >> 9,
    }
}