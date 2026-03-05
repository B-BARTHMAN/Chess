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

pub const KNIGHT_BB: [Bitboard; Square::SquareNb as usize] = {
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
pub const KING_BB: [Bitboard; Square::SquareNb as usize] = {
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