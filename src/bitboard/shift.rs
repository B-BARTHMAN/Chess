use crate::bitboard::bitboard::Bitboard;
use crate::board::direction::Direction;
use crate::board::square::Square;
use crate::board::rank::Rank;
use crate::board::file::File;
use crate::bitboard::masks::{FILE_A, FILE_H};

#[inline]
pub const fn shift_bb(bb: Bitboard, d: Direction) -> Bitboard {
  #[inline]
  const fn shift(bb: Bitboard, d: Direction) -> Bitboard {
    let x = d as i8;
    if x > 0 {
      return bb << x
    }
    bb >> -x
  }

  match d {
    Direction::North => shift(bb, Direction::North),
    Direction::South => shift(bb, Direction::South),
    Direction::East => shift(bb & !FILE_H, Direction::East),
    Direction::West => shift(bb & !FILE_A, Direction::West),
    Direction::NorthEast => shift(bb & !FILE_H, Direction::NorthEast),
    Direction::NorthWest => shift(bb & !FILE_A, Direction::NorthWest),
    Direction::SouthEast => shift(bb & !FILE_H, Direction::SouthEast),
    Direction::SouthWest => shift(bb & !FILE_A, Direction::SouthWest),
  }
}

#[inline]
pub fn shift_square(square: Square, d: Direction) -> Square {
  let sq = square as i8;
  let rank = Square::rank_of(square);
  let file = Square::file_of(square);

  match d {
    Direction::North     => if rank == Rank::R8 { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::South     => if rank == Rank::R1 { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::East      => if file == File::H  { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::West      => if file == File::A  { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::NorthEast => if rank == Rank::R8 || file == File::H { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::NorthWest => if rank == Rank::R8 || file == File::A { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::SouthEast => if rank == Rank::R1 || file == File::H { Square::None }  else { Square::from_index(sq + d as i8)},
    Direction::SouthWest => if rank == Rank::R1 || file == File::A { Square::None }  else { Square::from_index(sq + d as i8)},
  }
}