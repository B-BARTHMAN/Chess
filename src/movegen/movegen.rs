use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::ops::{count_1s, lsb};
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::movegen::normal::generate_moves;
use crate::movegen::pawns::generate_pawns;
use crate::movegen::precompute::{BETWEEN_BB, KING_BB};
use crate::piece::color::Color;
use crate::piece::piece_type::PieceType;
use crate::position::castling::CastlingRights;
use crate::position::position::Position;

#[repr(u8)]
pub enum MoveGenType {
    Capture,
    Quiet,
    Evasion,
}
pub fn move_gen<const T: u8>(pos: &Position, movelist: &mut MoveList) {
    let us = pos.side_to_move;
    let checkers = pos.checkers(us);
    let ksq = lsb(pos.pieces(PieceType::King, us));

    let mut target: Bitboard = 0u64;

    if T != MoveGenType::Evasion as u8 || count_1s(checkers) <= 1 {
        if T == MoveGenType::Evasion as u8 {
            target = BETWEEN_BB[ksq as usize][lsb(checkers) as usize];
        } else if T == MoveGenType::Capture as u8 {
            target = pos.by_color[us.other()]
        } else if T == MoveGenType::Quiet as u8 {
            target = !pos.all_pieces()
        } else {
            panic!("Unknown MoveGenType");
        }

        generate_moves(pos, target, PieceType::Knight, us, movelist);
        generate_moves(pos, target, PieceType::Bishop, us, movelist);
        generate_moves(pos, target, PieceType::Queen, us, movelist);
        generate_moves(pos, target, PieceType::Rook, us, movelist);
        generate_pawns::<T>(pos, target, us, movelist);
    }

    // King moves
    if T == MoveGenType::Evasion as u8 {
        let king_bb = KING_BB[ksq as usize] & !pos.by_color[us];
        movelist.fill_normal(ksq, king_bb);
    } else {
        let king_bb = KING_BB[ksq as usize] & target;
        movelist.fill_normal(ksq, king_bb);
    }

    // Castles
    if T == MoveGenType::Quiet as u8 {
        if us == Color::White {
            // White castle
            if pos.can_castle(CastlingRights::WhiteKingside) {
                movelist.push(Move::castle(CastlingRights::WhiteKingside));
            }
            if pos.can_castle(CastlingRights::WhiteQueenside) {
                movelist.push(Move::castle(CastlingRights::WhiteQueenside));
            }
        } else {
            // Black castle
            // White castle
            if pos.can_castle(CastlingRights::BlackKingside) {
                movelist.push(Move::castle(CastlingRights::BlackKingside));
            }
            if pos.can_castle(CastlingRights::BlackQueenside) {
                movelist.push(Move::castle(CastlingRights::BlackQueenside));
            }
        }
    }
}
pub fn generate(pos: &Position, movelist: &mut MoveList) {
    let ksq = pos.pieces(PieceType::King, pos.side_to_move);
    if count_1s(pos.attackers_to(lsb(ksq), pos.side_to_move.other())) != 0 {
        move_gen::<{ MoveGenType::Evasion as u8 }>(pos, movelist);
    } else {
        move_gen::<{ MoveGenType::Quiet as u8 }>(pos, movelist);
        move_gen::<{ MoveGenType::Capture as u8 }>(pos, movelist);
    }
}
