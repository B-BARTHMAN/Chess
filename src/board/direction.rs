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

impl Direction {
    pub const fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }
}
