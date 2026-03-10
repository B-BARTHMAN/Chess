use crate::types::{Bitboard, Square, Direction, File, Rank};

pub const A_FILE: Bitboard = 0x0101010101010101u64;
const B_FILE: Bitboard = A_FILE << 1;
const C_FILE: Bitboard = A_FILE << 2;
const D_FILE: Bitboard = A_FILE << 3;
const E_FILE: Bitboard = A_FILE << 4;
const F_FILE: Bitboard = A_FILE << 5;
const G_FILE: Bitboard = A_FILE << 6;
pub const H_FILE: Bitboard = A_FILE << 7;

const RANK_1: Bitboard = 0xFFu64;
pub const RANK_2: Bitboard = RANK_1 << (8 * 1);
pub const RANK_3: Bitboard = RANK_1 << (8 * 2);
const RANK_4: Bitboard = RANK_1 << (8 * 3);
const RANK_5: Bitboard = RANK_1 << (8 * 4);
pub const RANK_6: Bitboard = RANK_1 << (8 * 5);
pub const RANK_7: Bitboard = RANK_1 << (8 * 6);
const RANK_8: Bitboard = RANK_1 << (8 * 7);

#[inline(always)]
pub fn lsb(bb: Bitboard) -> Square {
    return if bb == 0 {
        Square::SquareNone
    } else {
        Square::from_index(bb.trailing_zeros() as i8)
    }
}

#[inline(always)]
pub fn pop_lsb(bb: &mut Bitboard) -> Square {
    let square = lsb(*bb);
    *bb &= *bb - 1;
    square
}

#[inline(always)]
pub fn count_1s(bb: Bitboard) -> u8 {
    bb.count_ones() as u8
}

pub const fn square_bb(square: Square) -> Bitboard {
    1u64 << (square as usize)
}
#[inline(always)]
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

#[inline(always)]
pub const fn shift_square(square: Square, d: Direction) -> Square {
    let sq = square as i8;
    let r = rank_of(square) as i8;
    let f = file_of(square) as i8;

    match d {
        Direction::North => if r == 7 { Square::SquareNone } else { Square::from_index(sq + 8) },
        Direction::South => if r == 0 { Square::SquareNone } else { Square::from_index(sq - 8) },
        Direction::East => if f == 7 { Square::SquareNone } else { Square::from_index(sq + 1) },
        Direction::West => if f == 0 { Square::SquareNone } else { Square::from_index(sq - 1) },
        Direction::NorthEast => if r == 7 || f == 7 { Square::SquareNone } else { Square::from_index(sq + 9) },
        Direction::NorthWest => if r == 7 || f == 0 { Square::SquareNone } else { Square::from_index(sq + 7) },
        Direction::SouthEast => if r == 0 || f == 7 { Square::SquareNone } else { Square::from_index(sq - 7) },
        Direction::SouthWest => if r == 0 || f == 0 { Square::SquareNone } else { Square::from_index(sq - 9) },
    }
}

#[inline(always)]
pub const fn is_ok(square: Square) -> bool {
    (square as i8) < 64 && (square as i8) >= 0
}

#[inline(always)]
pub const fn file_of(square: Square) -> File {
    File::from_index((square as u8) & 0b111)
}

#[inline(always)]
pub const fn rank_of(square: Square) -> Rank {
    Rank::from_index((square as u8) >> 3)
}

pub struct PrettyBitboard(pub u64);

impl std::fmt::Display for PrettyBitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bb = self.0;

        writeln!(f, "  A B C D E F G H")?;
        writeln!(f, "  ----------------")?;

        for rank in (0..8).rev() {
            write!(f, "{}| ", rank + 1)?;

            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (bb >> square) & 1;

                write!(f, "{} ", bit)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}