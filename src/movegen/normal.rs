use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::magic_bishop::bishop_attacks;
use crate::bitboard::magic_rook::rook_attacks;
use crate::bitboard::ops::pop_lsb;
use crate::board::square::Square;
use crate::movegen::attacks::attacks_bb;
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::movegen::precompute::{KING_BB, KNIGHT_BB};
use crate::piece::color::Color;
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

pub fn generate_all_pieces(pos: &Position, target: Bitboard, us: Color, ml: &mut MoveList) {
    generate_moves(pos, target, PieceType::Knight, us, ml);
    generate_moves(pos, target, PieceType::Bishop, us, ml);
    generate_moves(pos, target, PieceType::Rook, us, ml);
    generate_moves(pos, target, PieceType::Queen, us, ml);
}
fn generate_moves(
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
        let to = attacks_bb(pt, from, pos.occupied()) & target & !pos.by_color[col];
        movelist.fill_normal(from, to);
    }
}
