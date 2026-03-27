#[repr(i8)]
#[derive(Copy, Clone)]
pub enum Direction {
  North = 8,
  East = 1,

  South = -(Direction::North as i8),
  West = -(Direction::East as i8),

  NorthEast = (Direction::North as i8) + (Direction::East as i8),
  NorthWest = (Direction::North as i8) + (Direction::West as i8),
  SouthEast = (Direction::South as i8) + (Direction::East as i8),
  SouthWest = (Direction::South as i8) + (Direction::West as i8),
}