use bitflags::Bits;
use crate::bitboard::bitboard::Bitboard;
use crate::bitboard::masks::{RANK_2, RANK_3, RANK_6, RANK_7};
use crate::bitboard::shift::shift_bb;
use crate::board::direction::Direction;
use crate::board::square::Square;
use crate::moves::movelist::MoveList;
use crate::movegen::precompute::PAWN_BB;
use crate::piece::color::Color;
use crate::piece::piece_type::PieceType;
use crate::position::position::Position;

struct PawnContext {
    forward:            Direction,
    capture_left:       Direction,
    capture_right:      Direction,
    rank3:              Bitboard,
    rank7:              Bitboard,
}

impl PawnContext {
    fn new(color: Color) -> Self {
        let white = color == Color::White;
        Self {
            forward:            color.forward(),
            capture_left:       color.pawn_capture_left(),
            capture_right:      color.pawn_capture_right(),
            rank3:              if white { RANK_3 } else { RANK_6 },
            rank7:              if white { RANK_7 } else { RANK_2 },
        }
    }
}

pub fn gen_pawn_quiet(pos: &Position, us: Color, move_list: &mut MoveList) {
    let context = PawnContext::new(us);
    let empty = pos.empty();
    let enemies = pos.by_color[us.other()];
    let all_pawns = pos.pieces(PieceType::Pawn, us);
    let pawns = all_pawns & !context.rank7;
    let pawns_rank7 = all_pawns & context.rank7;

    let single_pushes = shift_bb(pawns, context.forward) & empty;
    let double_pushes = shift_bb(single_pushes & context.rank3, context.forward) & empty;
    move_list.fill_pawns(single_pushes, context.forward);
    move_list.fill_double_pawns(double_pushes, context.forward);

    if pawns_rank7 != 0 {
        let promo_forward= shift_bb(pawns_rank7, context.forward) & empty;
        let promo_left= shift_bb(pawns_rank7, context.capture_left) & enemies;
        let promo_right = shift_bb(pawns_rank7, context.capture_right) & enemies;
        move_list.fill_under_promotions(promo_forward, context.forward);
        move_list.fill_under_promotions(promo_left, context.capture_left);
        move_list.fill_under_promotions(promo_right, context.capture_right);
    }
}

pub fn gen_pawn_captures(pos: &Position, us: Color, move_list: &mut MoveList) {
    let context = PawnContext::new(us);
    let empty = pos.empty();
    let enemies = pos.by_color[us.other()];
    let all_pawns = pos.pieces(PieceType::Pawn, us);
    let pawns = all_pawns & !context.rank7;
    let pawns_rank7 = all_pawns & context.rank7;

    move_list.fill_pawns(shift_bb(pawns, context.capture_left)  & enemies, context.capture_left);
    move_list.fill_pawns(shift_bb(pawns, context.capture_right) & enemies, context.capture_right);

    if pos.ep_square() != Square::None {
        let ep_attackers = pawns & PAWN_BB[us.other() as usize][pos.ep_square() as usize];
        move_list.fill_enpassants(ep_attackers, pos.ep_square());
    }

    if pawns_rank7 != 0 {
        let promo_forward= shift_bb(pawns_rank7, context.forward) & empty;
        let promo_left= shift_bb(pawns_rank7, context.capture_left) & enemies;
        let promo_right= shift_bb(pawns_rank7, context.capture_right) & enemies;
        move_list.fill_queen_promotions(promo_forward, context.forward);
        move_list.fill_queen_promotions(promo_left, context.capture_left);
        move_list.fill_queen_promotions(promo_right, context.capture_right);
    }
}

pub fn gen_pawn_evasions(pos: &Position, target: Bitboard, us: Color, move_list: &mut MoveList) {
    let context = PawnContext::new(us);
    let empty = pos.empty();
    let enemies = pos.by_color[us.other()];
    let all_pawns = pos.pieces(PieceType::Pawn, us);
    let pawns = all_pawns & !context.rank7;
    let pawns_rank7 = all_pawns & context.rank7;

    let single_pushes = shift_bb(pawns, context.forward) & empty;
    let double_pushes = shift_bb(single_pushes & context.rank3, context.forward) & empty;
    move_list.fill_pawns(single_pushes & target, context.forward);
    move_list.fill_double_pawns(double_pushes & target, context.forward);

    move_list.fill_pawns(shift_bb(pawns, context.capture_left)  & enemies & target, context.capture_left);
    move_list.fill_pawns(shift_bb(pawns, context.capture_right) & enemies & target, context.capture_right);

    if pos.ep_square() != Square::None {
        let ep_attackers = pawns & PAWN_BB[us.other() as usize][pos.ep_square() as usize];
        move_list.fill_enpassants(ep_attackers, pos.ep_square());
    }

    if pawns_rank7 != 0 {
        let promo_forward= shift_bb(pawns_rank7, context.forward) & empty & target;
        let promo_left= shift_bb(pawns_rank7, context.capture_left) & enemies & target;
        let promo_right= shift_bb(pawns_rank7, context.capture_right) & enemies & target;
        move_list.fill_queen_promotions(promo_forward, context.forward);
        move_list.fill_queen_promotions(promo_left, context.capture_left);
        move_list.fill_queen_promotions(promo_right, context.capture_right);
        move_list.fill_under_promotions(promo_forward, context.forward);
        move_list.fill_under_promotions(promo_left, context.capture_left);
        move_list.fill_under_promotions(promo_right, context.capture_right);
    }
}