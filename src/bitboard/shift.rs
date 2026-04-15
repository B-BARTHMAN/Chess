use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::masks::{FILE_A, FILE_H};
use crate::board::direction::Direction;
use crate::board::file::{file_of, File};
use crate::board::rank::{rank_of, Rank};
use crate::board::square::Square;

#[inline]
pub const fn shift_bb(bb: Bitboard, d: Direction) -> Bitboard {
    let step = d as i8;
    let shifted = if step > 0 { bb << step } else { bb >> -step };
    match d {
        Direction::East      |
        Direction::NorthEast |
        Direction::SouthEast => shifted & !FILE_A,
        Direction::West      |
        Direction::NorthWest |
        Direction::SouthWest => shifted & !FILE_H,
        _                    => shifted,
    }
}

#[inline]
pub fn shift_square(square: Square, d: Direction) -> Square {
    let sq = square as i8;
    let rank = rank_of(square);
    let file = file_of(square);

    match d {
        Direction::North => {
            if rank == Rank::R8 {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::South => {
            if rank == Rank::R1 {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::East => {
            if file == File::H {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::West => {
            if file == File::A {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::NorthEast => {
            if rank == Rank::R8 || file == File::H {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::NorthWest => {
            if rank == Rank::R8 || file == File::A {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::SouthEast => {
            if rank == Rank::R1 || file == File::H {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
        Direction::SouthWest => {
            if rank == Rank::R1 || file == File::A {
                Square::None
            } else {
                Square::from_index(sq + d as i8)
            }
        }
    }
}
