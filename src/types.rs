
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PieceType {
    NoPieceType = 0,
    Pawn = 1, Knight, Bishop, Rook, Queen, King,
    PieceTypeNb = 7,
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Piece {
    NoPiece = 0,
    WPawn = 1, WKnight, WBishop, WRook, WQueen, WKing,
    BPawn, BKnight, BBishop, BRook, BQueen, BKing,
    PieceNb = 32,
}

#[repr(i8)]
#[derive(Copy, Clone)]
pub enum Square {
    SquareNone = -1,
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2    , B2, C2, D2, E2, F2, G2, H2,
    A3    , B3, C3, D3, E3, F3, G3, H3,
    A4    , B4, C4, D4, E4, F4, G4, H4,
    A5    , B5, C5, D5, E5, F5, G5, H5,
    A6    , B6, C6, D6, E6, F6, G6, H6,
    A7    , B7, C7, D7, E7, F7, G7, H7,
    A8    , B8, C8, D8, E8, F8, G8, H8,
    SquareNb = 64,
}

impl Square {
    #[inline(always)]
    pub const fn from_index(idx: i8) -> Self {
        debug_assert!(idx >= 0 && idx < 64);
        unsafe { std::mem::transmute(idx) }
    }
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Color {
    White = 0,
    Black,
    ColorNb = 2,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum File {
    FileA = 0,
    FileB,
    FileC,
    FileD,
    FileE,
    FileF,
    FileG,
    FileH,
}

impl File {
    #[inline(always)]
    pub const fn from_index(idx: u8) -> Self { unsafe { std::mem::transmute(idx) } }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Rank {
    Rank1 = 0,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
}

impl Rank {
    #[inline(always)]
    pub const fn from_index(idx: u8) -> Self { unsafe { std::mem::transmute(idx) } }
}

pub type Bitboard = u64;

#[derive(Copy, Clone)]
pub struct Move(pub u16);
impl Move{
    #[inline(always)]
    pub const fn new(from: Square, to: Square) -> Self {
        let from = from as u16;
        let to = to as u16;
        Move(from | (to << 6))
    }
}