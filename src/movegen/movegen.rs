use crate::bitboard::ops::{count_ones, lsb};
use crate::moves::movelist::MoveList;
use crate::movegen::king::{gen_king, gen_castles};
use crate::movegen::normal::generate_all_pieces;
use crate::movegen::pawns::{gen_pawn_quiet, gen_pawn_captures, gen_pawn_evasions};
use crate::movegen::precompute::{BETWEEN_BB, KING_BB};
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

pub fn gen_quiet(pos: &Position, ml: &mut MoveList) {
    let us = pos.side_to_move;
    let ksq = lsb(pos.pieces(PieceType::King, us));
    let target = pos.empty();

    generate_all_pieces(pos, target, us, ml);
    gen_pawn_quiet(pos, us, ml);
    gen_king(ksq, target, ml);
    gen_castles(pos, ml);
}

pub fn gen_captures(pos: &Position, ml: &mut MoveList) {
    let us = pos.side_to_move;
    let ksq = lsb(pos.pieces(PieceType::King, us));
    let target = pos.by_color[us.other()];

    generate_all_pieces(pos, target, us, ml);
    gen_pawn_captures(pos, us, ml);
    gen_king(ksq, target, ml);
}

pub fn gen_evasions(pos: &Position, ml: &mut MoveList) {
    let us = pos.side_to_move;
    let checkers = pos.checkers(us);
    let ksq = lsb(pos.pieces(PieceType::King, us));
    let king_target = KING_BB[ksq as usize] & !pos.by_color[us];

    if count_ones(checkers) > 1 {
        gen_king(ksq, king_target, ml);
        return;
    }

    let target = BETWEEN_BB[ksq as usize][lsb(checkers) as usize];
    generate_all_pieces(pos, target, us, ml);
    gen_pawn_evasions(pos, target, us, ml);
    gen_king(ksq, king_target, ml);
}

pub fn generate(pos: &Position, ml: &mut MoveList) {
    if pos.in_check() {
        gen_evasions(pos, ml);
    } else {
        gen_quiet(pos, ml);
        gen_captures(pos, ml);
    }
}