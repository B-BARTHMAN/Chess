use std::ops::Index;
use crate::util::indexable::Indexable;

pub const PIECE_TYPE_COUNT: usize = 6;
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceType {
    None = 255,
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECE_TYPES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
];

impl Indexable for PieceType {
    fn idx(self) -> usize {
        self as usize
    }
}
