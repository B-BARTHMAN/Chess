use crate::bitboard::bitboard::Bitboard;
use crate::board::square::Square;
#[inline]
pub const fn lsb(bb: Bitboard) -> Square {
    if bb == 0 {
        Square::None
    } else {
        Square::from_index(bb.trailing_zeros() as i8)
    }
}

#[inline]
pub const fn pop_lsb(bb: &mut Bitboard) -> Square {
    let square = lsb(*bb);
    *bb &= *bb - 1;
    square
}

#[inline]
pub const fn count_ones(bb: Bitboard) -> i32 {
    bb.count_ones() as i32
}
