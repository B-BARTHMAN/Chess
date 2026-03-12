use crate::bitboard::{pop_lsb, shift_bb, A_FILE, H_FILE, RANK_2, RANK_3, RANK_6, RANK_7};
use crate::magic_bishop::bishop_attacks;
use crate::magic_rook::rook_attacks;
use crate::movelist::MoveList;
use crate::position::Position;
use crate::types::{Bitboard, PieceType, Color, Square, Direction};

#[repr(u8)]
pub enum MoveGenType {
    Capture,
    Quiet,
    Evasion
}

pub fn generate_moves<const PT: u8, const US: u8>(pos: &Position, target: Bitboard, movelist: &mut MoveList) {
    debug_assert_ne!(PT, PieceType::Pawn as u8);
    debug_assert_ne!(PT, PieceType::King as u8);

    let mut bb = pos.pieces::<PT, US>();
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        let to = attacks_bb::<PT>(from, pos.all_pieces()) & target & !pos.by_color::<US>();
        movelist.fill_normal(from, to);
    }
}
pub fn gen_pawn<const US: u8, const T: u8>(pos: &Position, target: Bitboard, movelist: &mut MoveList) {
    let white = US == Color::White as u8;
    let them = 1 - US;

    let forward = if white { Direction::North } else { Direction::South };
    let capture_left = if white { Direction::NorthWest } else { Direction::SouthEast };
    let capture_right = if white { Direction::NorthEast } else { Direction::SouthWest };

    let rank3 = if white { RANK_3 } else { RANK_6 };
    let rank7 = if white { RANK_7 } else { RANK_2 };

    let forward_step = if white { 8 } else { -8 };
    let double_step = if white { 16 } else { -16 };
    let capture_left_step = if white { 7 } else { -7 };
    let capture_right_step = if white { 9 } else { -9 };

    let gen_quiet = T != MoveGenType::Capture as u8;
    let gen_capture = T != MoveGenType::Quiet as u8;

    let all_pawns = pos.pieces::<{PieceType::Pawn as u8}, US>();

    let pawns_rank7 = all_pawns & rank7;
    let pawns = all_pawns & !rank7;

    // Precompute shifts
    let forward_moves = shift_bb(pawns, forward);
    let capture_left_moves = shift_bb(pawns, capture_left);
    let capture_right_moves = shift_bb(pawns, capture_right);

    if gen_quiet {
        let single_pushes = forward_moves & target;
        let double_pushes = shift_bb(single_pushes & rank3, forward) & target;

        movelist.fill_pawns(single_pushes, forward_step);
        movelist.fill_pawns(double_pushes, double_step);
    }

    if gen_capture {
        movelist.fill_pawns(capture_left_moves & target, capture_left_step);
        movelist.fill_pawns(capture_right_moves & target, capture_right_step);

        if pos.ep_square() != Square::SquareNone {
            let bb = pawns & PAWN_BB[them as usize][pos.ep_square() as usize];
            movelist.fill_enpassants(bb, pos.ep_square());
        }
    }

    if pawns_rank7 != 0 {
        let promotion_forward = shift_bb(pawns_rank7, forward);
        let promotion_capture_left = shift_bb(pawns_rank7, capture_left);
        let promotion_capture_right = shift_bb(pawns_rank7, capture_right);

        if gen_quiet {
            movelist.fill_promotions::<T>(promotion_forward & target, forward_step);
        }

        if gen_capture {
            movelist.fill_promotions::<T>(promotion_capture_left & target, capture_left_step);
            movelist.fill_promotions::<T>(promotion_capture_right & target, capture_right_step);
        }
    }
}

#[inline(always)]
fn attacks_bb<const PT: u8>(square: Square, blockers: Bitboard) -> Bitboard {
    debug_assert_ne!(PT, PieceType::Pawn as u8);

    const KING: u8 = PieceType::King as u8;
    const KNIGHT: u8 = PieceType::Knight as u8;
    const BISHOP: u8 = PieceType::Bishop as u8;
    const ROOK: u8 = PieceType::Rook as u8;
    const QUEEN: u8 = PieceType::Queen as u8;

    match PT {
        KNIGHT => KNIGHT_BB[square as usize],
        BISHOP => bishop_attacks(square, blockers),
        ROOK => rook_attacks(square, blockers),
        QUEEN => bishop_attacks(square, blockers) | rook_attacks(square, blockers),
        KING => KING_BB[square as usize],
        _ => unreachable!(),
    }
}
const KNIGHT_BB: [Bitboard; Square::SquareNb as usize] = {
    // Create result
    let mut result: [Bitboard; Square::SquareNb as usize] = [0u64; Square::SquareNb as usize];
    // Define all steps a knight can take
    const STEPS: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    // Start with First Square
    let mut sq: i8 = Square::A1 as i8;
    while sq < 64 {
        // loop through all steps
        let mut i = 0;
        while i < 8 {
            let to = sq + STEPS[i];
            // Check if square is valid
            if to >= 0 && to < 64 && ((sq & 0b111i8) - (to & 0b111i8)).abs() <= 2 {
                // Add it to bitboard
                result[sq as usize] |= 1u64 << to;
            }
            i += 1;
        }
        sq += 1;
    }
    result
};
const KING_BB: [Bitboard; Square::SquareNb as usize] = {
    // Create result
    let mut result: [Bitboard; Square::SquareNb as usize] = [0u64; Square::SquareNb as usize];
    // Define all steps a king can take
    const STEPS: [i8; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
    // Start with First Square
    let mut sq: i8 = Square::A1 as i8;
    while sq < 64 {
        // Loop though all steps
        let mut i = 0;
        while i < 8 {
            let to = sq + STEPS[i];
            // Check if square is valid
            if to >= 0 && to < 64 && ((sq & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                // Add it to bitboard
                result[sq as usize] |= 1u64 << to;
            }
            i += 1;
        }
        sq += 1;
    }
    result
};
const PAWN_BB: [[Bitboard; Square::SquareNb as usize]; Color::ColorNb as usize] = {
    // Create result
    let mut result: [[Bitboard; Square::SquareNb as usize]; Color::ColorNb as usize] = [[0u64; Square::SquareNb as usize]; Color::ColorNb as usize];
    // Start with First Square
    let mut sq: i8 = Square::A1 as i8;
    while sq < 64 {

        let nw = sq + 7;
        let ne = sq + 9;
        let se = sq - 7;
        let sw = sq - 9;

        if nw >= 0 && nw < 64 && ((sq & 0b111) - (nw & 0b111i8)).abs() <= 1 {
            result[Color::White as usize][sq as usize] |= 1u64 << nw;
        }
        if ne >= 0 && ne < 64 && ((sq & 0b111) - (ne & 0b111i8)).abs() <= 1 {
            result[Color::White as usize][sq as usize] |= 1u64 << ne;
        }
        if sw >= 0 && sw < 64 && ((sq & 0b111) - (sw & 0b111i8)).abs() <= 1 {
            result[Color::Black as usize][sq as usize] |= 1u64 << sw;
        }
        if se >= 0 && se < 64 && ((sq & 0b111) - (se & 0b111i8)).abs() <= 1 {
            result[Color::Black as usize][sq as usize] |= 1u64 << se;
        }

        sq += 1;
    }
    result
};
pub const BETWEEN_BB: [[Bitboard; Square::SquareNb as usize]; Square::SquareNb as usize] = {
    let mut result: [[Bitboard; 64]; 64] = [[0u64; 64]; 64];
    let mut from: i8 = Square::A1 as i8;
    while from < 64 {
        let mut to: i8 = Square::A1 as i8;
        while to < 64 {

            // Check if same Rank
            if (from >> 3) == (to >> 3) {
                let dir = (to - from).signum();
                let mut sq = from + dir;

                'inner: loop {
                    result[from as usize][to as usize] |= 1u64 << sq;
                    if(sq == to) {
                        break 'inner;
                    }
                    sq += dir;
                }
            }
            // Check if same File
            else if (from & 0b111) == (to & 0b111) {
                let dir = (to - from).signum() * 8;
                let mut sq = from + dir;

                'inner: loop {
                    result[from as usize][to as usize] |= 1u64 << sq;
                    if(sq == to) {
                        break 'inner;
                    }
                    sq += dir;
                }
            }
            // Check same Diagonal
            else if ((from >> 3) - (from & 0b111)) == ((to >> 3) - (to & 0b111)) {
                let dir = (to - from).signum() * 9;
                let mut sq = from + dir;

                'inner: loop {
                    result[from as usize][to as usize] |= 1u64 << sq;
                    if(sq == to) {
                        break 'inner;
                    }
                    sq += dir;
                }
            }
            // Check same AntiDiagonal
            else if ((from >> 3) + (from & 0b111)) == ((to >> 3) + (to & 0b111)) {
                let dir = (to - from).signum() * 7;
                let mut sq = from + dir;

                'inner: loop {
                    result[from as usize][to as usize] |= 1u64 << sq;
                    if(sq == to) {
                        break 'inner;
                    }
                    sq += dir;
                }
            }
            // Other cases
            else {
                result[from as usize][to as usize] |= 1u64 << to;
            }

            to += 1;
        }
        from += 1;
    }
    result
};