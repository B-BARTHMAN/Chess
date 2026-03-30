use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::magic_bishop::bishop_attacks;
use crate::bitboard::magic_rook::rook_attacks;
use crate::bitboard::ops::pop_lsb;
use crate::board::square::Square;
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::movegen::precompute::{KING_BB, KNIGHT_BB};
use crate::piece::color::Color;
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

pub fn generate_moves(
    pos: &Position,
    target: Bitboard,
    pt: PieceType,
    col: Color,
    movelist: &mut MoveList,
) {
    debug_assert_ne!(pt, PieceType::Pawn);
    debug_assert_ne!(pt, PieceType::King);

    let mut bb = pos.pieces(pt, col); //pos.pieces::<PT, US>();
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        let to = attacks_bb(pt, from, pos.all_pieces()) & target & !pos.by_color[col];
        movelist.fill_normal(from, to);
    }
}

#[inline(always)]
pub fn attacks_bb(pt: PieceType, square: Square, blockers: Bitboard) -> Bitboard {
    debug_assert_ne!(pt, PieceType::Pawn);

    match pt {
        PieceType::Knight => KNIGHT_BB[square as usize],
        PieceType::Bishop => bishop_attacks(square, blockers),
        PieceType::Rook => rook_attacks(square, blockers),
        PieceType::Queen => bishop_attacks(square, blockers) | rook_attacks(square, blockers),
        PieceType::King => KING_BB[square as usize],
        PieceType::Pawn => unreachable!(),
        _ => panic!(),
    }
}

impl MoveList {
    pub fn fill_normal(&mut self, from: Square, mut to_bb: Bitboard) {
        while to_bb != 0 {
            let to = pop_lsb(&mut to_bb);
            self.push(Move::normal(from, to));
        }
    }
}
