use crate::bitboard::bitboard::Bitboard;
use crate::board::square::Square;
pub const FILE_A: Bitboard = 0x0101010101010101u64;
pub const FILE_B: Bitboard = FILE_A << 1;
pub const FILE_C: Bitboard = FILE_A << 2;
pub const FILE_D: Bitboard = FILE_A << 3;
pub const FILE_E: Bitboard = FILE_A << 4;
pub const FILE_F: Bitboard = FILE_A << 5;
pub const FILE_G: Bitboard = FILE_A << 6;
pub const FILE_H: Bitboard = FILE_A << 7;

pub const RANK_1: Bitboard = 0xFFu64;
pub const RANK_2: Bitboard = RANK_1 << (8 * 1);
pub const RANK_3: Bitboard = RANK_1 << (8 * 2);
pub const RANK_4: Bitboard = RANK_1 << (8 * 3);
pub const RANK_5: Bitboard = RANK_1 << (8 * 4);
pub const RANK_6: Bitboard = RANK_1 << (8 * 5);
pub const RANK_7: Bitboard = RANK_1 << (8 * 6);
pub const RANK_8: Bitboard = RANK_1 << (8 * 7);

#[inline]
pub const fn square_bb(square: Square) -> Bitboard {
    1u64 << (square as usize)
}
