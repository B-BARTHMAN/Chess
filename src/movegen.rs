use crate::bitboard::{file_of, is_ok, pop_lsb, square_bb};
use crate::movelist::MoveList;
use crate::position::Position;
use crate::types::{Bitboard, PieceType, Color, Square};

pub fn generate_moves<const PT: u8, const US: u8>(pos: &Position, target: Bitboard, movelist: &mut MoveList) {
    let mut bb = pos.pieces::<PT, US>();
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        // NOT DONE
    }
}

const fn knight_moves(square: Square) -> Bitboard {
    let mut bb: Bitboard = 0u64;
    let steps: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let mut i = 0;
    while i < 8 {
        let to = Square::from_index((square as i8) + steps[i]);
        if is_ok(to) && ((file_of(square) as i8) - (file_of(square) as i8).abs() <= 2) {
            bb |= square_bb(to);
        }
        i += 1;
    }
    bb
}