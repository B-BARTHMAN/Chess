use crate::board::direction::Direction;
use crate::util::indexable::Indexable;

pub const COLOR_COUNT: usize = 2;
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Indexable for Color {
    #[inline]
    fn idx(self) -> usize { self as usize }
}

impl Color {
    #[inline]
    pub const fn other(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    #[inline]
    pub const fn forward(&self) -> Direction {
        match self {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        }
    }
    #[inline]
    pub const fn pawn_capture_left(&self) -> Direction {
        match self {
            Color::White => Direction::NorthWest,
            Color::Black => Direction::SouthEast,
        }
    }
    #[inline]
    pub const fn pawn_capture_right(&self) -> Direction {
        match self {
            Color::White => Direction::NorthEast,
            Color::Black => Direction::SouthWest,
        }
    }
    #[inline]
    pub fn from_fen(fen: char) -> Self {
        debug_assert!("wb".contains(fen));
        match fen{
            'w' => Color::White,
            'b' => Color::Black,
            _ => unreachable!(),
        }
    }
}
