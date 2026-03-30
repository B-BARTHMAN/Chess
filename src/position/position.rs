use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::masks::square_bb;
use crate::bitboard::ops::{lsb, pop_lsb};
use crate::board::square::{Square, SQUARE_COUNT};
use crate::moves::chess_move::Move;
use crate::movegen::normal::attacks_bb;
use crate::movegen::precompute::PAWN_BB;
use crate::piece::color::{COLOR_COUNT, Color};
use crate::piece::piece::Piece;
use crate::piece::piece_type::{PIECE_TYPE_COUNT, PIECE_TYPES, PieceType};
use crate::position::castling::{
    BLACK_KINGSIDE, BLACK_KINGSIDE_PATH, BLACK_QUEENSIDE, BLACK_QUEENSIDE_PATH, CastlingRights,
    WHITE_KINGSIDE, WHITE_KINGSIDE_PATH, WHITE_QUEENSIDE, WHITE_QUEENSIDE_PATH,
};
use crate::position::state::State;
use crate::util::by::By;

pub struct Position {
    pub by_type: By<PieceType, Bitboard, PIECE_TYPE_COUNT>,
    pub by_color: By<Color, Bitboard, COLOR_COUNT>,
    pub by_square: By<Square, Piece, SQUARE_COUNT>,
    pub states: Vec<State>,
    pub side_to_move: Color,
}

impl Position {
    #[inline]
    pub fn pieces(&self, pt: PieceType, c: Color) -> Bitboard { self.by_type[pt] & self.by_color[c] }
    #[inline]
    pub fn all_pieces(&self) -> Bitboard { self.by_color[Color::White] | self.by_color[Color::Black] }
    #[inline]
    pub fn empty(&self) -> Bitboard { !self.all_pieces() }
    #[inline]
    pub fn enemies(&self, c: Color) -> Bitboard { self.by_color[c.other()] }
    #[inline]
    pub fn state(&self) -> &State { self.states.last().unwrap() }
    #[inline]
    pub fn ep_square(&self) -> Square {
        self.state().ep_square
    }
    #[inline]
    pub fn can_castle(&self, rights: CastlingRights) -> bool { self.castle_allowed(rights) && !self.castle_blocked(rights) && !self.castle_attacked(rights) }
    #[inline]
    fn castle_allowed(&self, rights: CastlingRights) -> bool {
        self.state().castling_rights & rights != CastlingRights::empty()
    }
    #[inline]
    fn castle_blocked(&self, rights: CastlingRights) -> bool {
        let pieces = self.all_pieces();
        match rights {
            CastlingRights::WhiteKingside => (WHITE_KINGSIDE_PATH & pieces) != 0,
            CastlingRights::WhiteQueenside => (WHITE_QUEENSIDE_PATH & pieces) != 0,
            CastlingRights::BlackKingside => (BLACK_KINGSIDE_PATH & pieces) != 0,
            CastlingRights::BlackQueenside => (BLACK_QUEENSIDE_PATH & pieces) != 0,
            _ => panic!("Encountered weird castling rights type"),
        }
    }
    #[inline]
    fn castle_attacked(&self, rights: CastlingRights) -> bool {
        let enemy = self.side_to_move.other();
        let mut castle_squares = match rights {
            CastlingRights::WhiteKingside => WHITE_KINGSIDE,
            CastlingRights::WhiteQueenside => WHITE_QUEENSIDE,
            CastlingRights::BlackKingside => BLACK_KINGSIDE,
            CastlingRights::BlackQueenside => BLACK_QUEENSIDE,
            _ => panic!("Encountered weird castling rights type"),
        };
        while castle_squares != 0 {
            let sq = pop_lsb(&mut castle_squares);
            if self.attackers_to(sq, enemy) != 0 {
                return true;
            }
        }
        false
    }
    #[inline]
    pub fn attackers_to(&self, sq: Square, enemy: Color) -> Bitboard {
        let pieces = self.all_pieces();
        (attacks_bb(PieceType::Rook, sq, pieces)
            & (self.pieces(PieceType::Rook, enemy) | self.pieces(PieceType::Queen, enemy)))
            | (attacks_bb(PieceType::Bishop, sq, pieces)
                & (self.pieces(PieceType::Bishop, enemy) | self.pieces(PieceType::Queen, enemy)))
            | (attacks_bb(PieceType::King, sq, pieces) & self.pieces(PieceType::King, enemy))
            | (attacks_bb(PieceType::Knight, sq, pieces) & self.pieces(PieceType::Knight, enemy))
            | (PAWN_BB[enemy.other() as usize][sq as usize]
                & self.pieces(PieceType::Pawn, enemy))
    }

    #[inline]
    pub fn checkers(&self, us: Color) -> Bitboard {
        let kqs = lsb(self.pieces(PieceType::King, us));
        self.attackers_to(kqs, us.other())
    }
}
