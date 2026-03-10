use crate::types::MoveType::Normal;
use crate::types::Piece::NoPiece;

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

impl Piece {
    #[inline(always)]
    pub const fn from_fen(fen: char) -> Piece {
        match fen {
            'k' => Piece::BKing,
            'q' => Piece::BQueen,
            'n' => Piece::BKnight,
            'b' => Piece::BBishop,
            'r' => Piece::BRook,
            'p' => Piece::BPawn,
            'K' => Piece::WKing,
            'Q' => Piece::WQueen,
            'N' => Piece::WKnight,
            'B' => Piece::WBishop,
            'R' => Piece::WRook,
            'P' => Piece::WPawn,
            _ => Piece::NoPiece,
        }
    }
    #[inline(always)]
    pub const fn piece_type(&self) -> PieceType {
        match self {
            Piece::BKing => PieceType::King,
            Piece::WPawn => PieceType::Pawn,
            Piece::WKnight => PieceType::Knight,
            Piece::WBishop => PieceType::Bishop,
            Piece::WRook => PieceType::Rook,
            Piece::WQueen => PieceType::Queen,
            Piece::WKing => PieceType::King,
            Piece::BPawn => PieceType::Pawn,
            Piece::BKnight => PieceType::Knight,
            Piece::BBishop => PieceType::Bishop,
            Piece::BRook => PieceType::Rook,
            Piece::BQueen => PieceType::Queen,
            _ => PieceType::NoPieceType,
        }
    }
    #[inline(always)]
    pub const fn color(&self) -> Color {
        match self {
            Piece::BKing => Color::Black,
            Piece::WPawn => Color::White,
            Piece::WKnight => Color::White,
            Piece::WBishop => Color::White,
            Piece::WRook => Color::White,
            Piece::WQueen => Color::White,
            Piece::WKing => Color::White,
            Piece::BPawn => Color::Black,
            Piece::BKnight => Color::Black,
            Piece::BBishop => Color::Black,
            Piece::BRook => Color::Black,
            Piece::BQueen => Color::Black,
            _ => unreachable!(),
        }
    }
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

impl Color {
    #[inline(always)]
    pub const fn other(self) -> Color {
        match self {
            Color::White  => Color::Black,
            Color::Black => Color::White,
            _ => unreachable!(),
        }
    }
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

#[repr(u16)]
pub enum MoveType {
    Normal = 0u16 << 12,
    Promotion = 1u16 << 12,
    EnPassant = 2u16 << 12,
    Castle = 3u16 << 12,
}

#[repr(u16)]
pub enum PromotionType {
    Queen = 0u16 << 14,
    Rook = 1u16 << 14,
    Bishop = 2u16 << 14,
    Knight = 3u16 << 14,
}

#[derive(Copy, Clone)]
pub struct Move(pub u16);
impl Move {
    #[inline(always)]
    pub const fn normal(from: Square, to: Square) -> Self {
        let from = from as u16;
        let to = to as u16;
        Move(from | (to << 6))
    }
    #[inline(always)]
    pub const fn promotion(from: Square, to: Square, pt: PromotionType) -> Self {
        let from = from as u16;
        let to = to as u16;
        Move(from | (to << 6) | (MoveType::Promotion as u16) | (pt as u16))
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from = (self.0 & 0x3F) as u8;
        let to = ((self.0 >> 6) & 0x3F) as u8;

        fn square_to_str(sq: u8) -> String {
            let file = (sq % 8) as u8 + b'A';
            let rank = (sq / 8) as u8 + b'1';
            format!("{}{}", file as char, rank as char)
        }

        write!(f, "{}-{}", square_to_str(from), square_to_str(to))
    }
}