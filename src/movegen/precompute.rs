use crate::bitboard::bitboard::Bitboard;
use crate::board::square::{SQUARE_COUNT, Square};
use crate::piece::color::{COLOR_COUNT, Color};

pub const KNIGHT_BB: [Bitboard; SQUARE_COUNT] = {
    // Create result
    let mut result: [Bitboard; SQUARE_COUNT] = [0u64; SQUARE_COUNT];
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
pub const KING_BB: [Bitboard; SQUARE_COUNT] = {
    // Create result
    let mut result: [Bitboard; SQUARE_COUNT] = [0u64; SQUARE_COUNT];
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
pub const PAWN_BB: [[Bitboard; SQUARE_COUNT]; COLOR_COUNT] = {
    // Create result
    let mut result: [[Bitboard; SQUARE_COUNT]; COLOR_COUNT] = [[0u64; SQUARE_COUNT]; COLOR_COUNT];
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
pub const BETWEEN_BB: [[Bitboard; SQUARE_COUNT]; SQUARE_COUNT] = {
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
                    if sq == to {
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
                    if sq == to {
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
                    if sq == to {
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
                    if sq == to {
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
