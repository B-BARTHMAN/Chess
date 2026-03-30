use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::masks::square_bb;
use crate::board::square::Square;
use bitflags::bitflags;

pub const WHITE_KINGSIDE: Bitboard =
    square_bb(Square::E1) | square_bb(Square::F1) | square_bb(Square::G1);
pub const WHITE_QUEENSIDE: Bitboard =
    square_bb(Square::C1) | square_bb(Square::D1) | square_bb(Square::E1);
pub const BLACK_KINGSIDE: Bitboard =
    square_bb(Square::E8) | square_bb(Square::F8) | square_bb(Square::G8);
pub const BLACK_QUEENSIDE: Bitboard =
    square_bb(Square::C8) | square_bb(Square::D8) | square_bb(Square::E8);
pub const WHITE_KINGSIDE_PATH: Bitboard = square_bb(Square::F1) | square_bb(Square::G1);
pub const WHITE_QUEENSIDE_PATH: Bitboard =
    square_bb(Square::B1) | square_bb(Square::C1) | square_bb(Square::D1);
pub const BLACK_KINGSIDE_PATH: Bitboard = square_bb(Square::F8) | square_bb(Square::G8);
pub const BLACK_QUEENSIDE_PATH: Bitboard =
    square_bb(Square::B8) | square_bb(Square::C8) | square_bb(Square::D8);

bitflags! {
  #[derive(Copy, Clone, PartialEq, Eq)]
  pub struct CastlingRights: u8 {
    const WhiteKingside  = 0b0001;
    const WhiteQueenside = 0b0010;
    const BlackKingside  = 0b0100;
    const BlackQueenside = 0b1000;
  }
}

impl CastlingRights {
    pub fn from_fen(fen: &str) -> Self {
        let mut rights = CastlingRights::empty();

        if fen == "-" {
            return rights;
        }

        for c in fen.chars() {
            match c {
                'K' => rights |= CastlingRights::WhiteKingside,
                'Q' => rights |= CastlingRights::WhiteQueenside,
                'k' => rights |= CastlingRights::BlackKingside,
                'q' => rights |= CastlingRights::BlackQueenside,
                _ => panic!("Invalid castling rights character: '{}'", c),
            }
        }

        rights
    }
}
