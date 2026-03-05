use crate::types::{Bitboard, Square};

pub fn rook_attacks(square: Square, blockers: Bitboard) -> Bitboard {
    let mask = MASK_ROOK[square as usize];
    let magic = MAGIC_ROOK[square as usize];
    let shift = SHIFT_ROOK[square as usize];

    let index = (blockers & mask).wrapping_mul(magic) >> (64 - shift);
    lookup(square, index as usize)
}
const fn lookup(square: Square, index: usize) -> Bitboard {
    match square{
        Square::SquareNone => {0u64}
        Square::A1 => { ROOK_BB_A1[index]}
        Square::B1 => { ROOK_BB_B1[index]}
        Square::C1 => { ROOK_BB_C1[index]}
        Square::D1 => { ROOK_BB_D1[index]}
        Square::E1 => { ROOK_BB_E1[index]}
        Square::F1 => { ROOK_BB_F1[index]}
        Square::G1 => { ROOK_BB_G1[index]}
        Square::H1 => { ROOK_BB_H1[index]}
        Square::A2 => { ROOK_BB_A2[index]}
        Square::B2 => { ROOK_BB_B2[index]}
        Square::C2 => { ROOK_BB_C2[index]}
        Square::D2 => { ROOK_BB_D2[index]}
        Square::E2 => { ROOK_BB_E2[index]}
        Square::F2 => { ROOK_BB_F2[index]}
        Square::G2 => { ROOK_BB_G2[index]}
        Square::H2 => { ROOK_BB_H2[index]}
        Square::A3 => { ROOK_BB_A3[index]}
        Square::B3 => { ROOK_BB_B3[index]}
        Square::C3 => { ROOK_BB_C3[index]}
        Square::D3 => { ROOK_BB_D3[index]}
        Square::E3 => { ROOK_BB_E3[index]}
        Square::F3 => { ROOK_BB_F3[index]}
        Square::G3 => { ROOK_BB_G3[index]}
        Square::H3 => { ROOK_BB_H3[index]}
        Square::A4 => { ROOK_BB_A4[index]}
        Square::B4 => { ROOK_BB_B4[index]}
        Square::C4 => { ROOK_BB_C4[index]}
        Square::D4 => { ROOK_BB_D4[index]}
        Square::E4 => { ROOK_BB_E4[index]}
        Square::F4 => { ROOK_BB_F4[index]}
        Square::G4 => { ROOK_BB_G4[index]}
        Square::H4 => { ROOK_BB_H4[index]}
        Square::A5 => { ROOK_BB_A5[index]}
        Square::B5 => { ROOK_BB_B5[index]}
        Square::C5 => { ROOK_BB_C5[index]}
        Square::D5 => { ROOK_BB_D5[index]}
        Square::E5 => { ROOK_BB_E5[index]}
        Square::F5 => { ROOK_BB_F5[index]}
        Square::G5 => { ROOK_BB_G5[index]}
        Square::H5 => { ROOK_BB_H5[index]}
        Square::A6 => { ROOK_BB_A6[index]}
        Square::B6 => { ROOK_BB_B6[index]}
        Square::C6 => { ROOK_BB_C6[index]}
        Square::D6 => { ROOK_BB_D6[index]}
        Square::E6 => { ROOK_BB_E6[index]}
        Square::F6 => { ROOK_BB_F6[index]}
        Square::G6 => { ROOK_BB_G6[index]}
        Square::H6 => { ROOK_BB_H6[index]}
        Square::A7 => { ROOK_BB_A7[index]}
        Square::B7 => { ROOK_BB_B7[index]}
        Square::C7 => { ROOK_BB_C7[index]}
        Square::D7 => { ROOK_BB_D7[index]}
        Square::E7 => { ROOK_BB_E7[index]}
        Square::F7 => { ROOK_BB_F7[index]}
        Square::G7 => { ROOK_BB_G7[index]}
        Square::H7 => { ROOK_BB_H7[index]}
        Square::A8 => { ROOK_BB_A8[index]}
        Square::B8 => { ROOK_BB_B8[index]}
        Square::C8 => { ROOK_BB_C8[index]}
        Square::D8 => { ROOK_BB_D8[index]}
        Square::E8 => { ROOK_BB_E8[index]}
        Square::F8 => { ROOK_BB_F8[index]}
        Square::G8 => { ROOK_BB_G8[index]}
        Square::H8 => { ROOK_BB_H8[index]}
        Square::SquareNb => {0u64}
    }
}
const MASK_ROOK: [Bitboard; Square::SquareNb as usize] = {
    let mut result: [Bitboard; Square::SquareNb as usize] = [0u64; Square::SquareNb as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];

    let mut sq = Square::A1 as i8;
    while sq < 64 {
        let mut i = 0;
        while i < 4 {
            let mut from = sq;
            let mut to = sq + STEPS[i];
            while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                result[sq as usize] |= 1u64 << from;
                from = to;
                to = to + STEPS[i];
            }
            i += 1;
        }
        result[sq as usize] ^= 1u64 << sq;
        sq +=1;
    }
    result
};
const MAGIC_ROOK: [u64; Square::SquareNb as usize] = {[
    0x2492f03ba449770b, 0x8a257639fa93076f, 0x35075a7f00603507, 0x7c71c9b4ef8eeb36, 0x526351534db2634b, 0x349f0360f9dee1f0, 0x7f8ffa3c7ffe68f7, 0x5abdce6c447bc17a,
    0x120274c59bb2bd66, 0xd1a5768e0d321f92, 0xed0e000b275dffec, 0x0717479de8b8c5f3, 0xefd300080031013c, 0x0a3674e8a276752d, 0xa58c0030223b3408, 0xb4ea00008c19ffe3,
    0x1f0f4201c04000e4, 0xdab0c0d096e6d7ab, 0xc72d82001206c061, 0xcc6e5bc8aca5ce72, 0x51d9f2002a207a00, 0xb809abff6de9e400, 0xccc024001830434a, 0x694a8200010d499c,
    0x9c445a353007c005, 0xfe1f320b7c4cf2eb, 0xfbd3811200416201, 0xa342d8244028ecbf, 0x3842b08200182084, 0x767f9916ae473d41, 0x434d580c00502a13, 0xd44f689e000100c4,
    0x9b055ea306bd4094, 0xbd5fdd046a2e3980, 0x8b56ab65bffb9d23, 0xe183c55d68238823, 0x2d2d93176bb244d8, 0xc2ff3fec0014597b, 0x0dab0f1814006a10, 0x7151dc29ca000285,
    0xab3644e61a15cbdd, 0x9db6050087e20040, 0x1b29832202c20012, 0xb04726b7f76ec038, 0x893ec703f6363631, 0x16440a6c43d3bc2f, 0xd563105607140088, 0x4b158bd28c0e0003,
    0x24bd8200450cea00, 0xccfff3b69d7f59d0, 0x269981b021460600, 0xbb22b00300e15900, 0x8325689421300a00, 0x61dbdc48501f3a00, 0xda5b1a2850270c00, 0x1ada7d108c00d600,
    0xb7c6030126c08032, 0x7bfffe9733dace86, 0xb04b41a9a0010091, 0x4e04a2004a5040ea, 0x5d91cf630028004d, 0x0eb7fff477188f6a, 0x249261980e0b300c, 0x0dd77104002eb592,
]};
const SHIFT_ROOK: [u8; Square::SquareNb as usize] = {[
    13, 12, 12, 12, 12, 12, 12, 13,
    12, 11, 11, 11, 10, 11, 10, 11,
    12, 11, 10, 11, 10, 11, 10, 11,
    12, 11, 10, 11, 11, 11, 10, 11,
    12, 11, 11, 11, 11, 11, 10, 11,
    12, 10, 10, 11, 11, 11, 10, 11,
    11, 10, 10, 10, 11, 11, 10, 11,
    12, 11, 11, 11, 11, 12, 11, 12,
]};
const ROOK_BB_A1: [Bitboard; (1 << SHIFT_ROOK[Square::A1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A1 as usize]) as usize];
    let mask = MASK_ROOK[Square::A1 as usize];
    let magic = MAGIC_ROOK[Square::A1 as usize];
    let shift = SHIFT_ROOK[Square::A1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B1: [Bitboard; (1 << SHIFT_ROOK[Square::B1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B1 as usize]) as usize];
    let mask = MASK_ROOK[Square::B1 as usize];
    let magic = MAGIC_ROOK[Square::B1 as usize];
    let shift = SHIFT_ROOK[Square::B1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C1: [Bitboard; (1 << SHIFT_ROOK[Square::C1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C1 as usize]) as usize];
    let mask = MASK_ROOK[Square::C1 as usize];
    let magic = MAGIC_ROOK[Square::C1 as usize];
    let shift = SHIFT_ROOK[Square::C1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D1: [Bitboard; (1 << SHIFT_ROOK[Square::D1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D1 as usize]) as usize];
    let mask = MASK_ROOK[Square::D1 as usize];
    let magic = MAGIC_ROOK[Square::D1 as usize];
    let shift = SHIFT_ROOK[Square::D1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E1: [Bitboard; (1 << SHIFT_ROOK[Square::E1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E1 as usize]) as usize];
    let mask = MASK_ROOK[Square::E1 as usize];
    let magic = MAGIC_ROOK[Square::E1 as usize];
    let shift = SHIFT_ROOK[Square::E1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F1: [Bitboard; (1 << SHIFT_ROOK[Square::F1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F1 as usize]) as usize];
    let mask = MASK_ROOK[Square::F1 as usize];
    let magic = MAGIC_ROOK[Square::F1 as usize];
    let shift = SHIFT_ROOK[Square::F1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G1: [Bitboard; (1 << SHIFT_ROOK[Square::G1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G1 as usize]) as usize];
    let mask = MASK_ROOK[Square::G1 as usize];
    let magic = MAGIC_ROOK[Square::G1 as usize];
    let shift = SHIFT_ROOK[Square::G1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H1: [Bitboard; (1 << SHIFT_ROOK[Square::H1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H1 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H1 as usize]) as usize];
    let mask = MASK_ROOK[Square::H1 as usize];
    let magic = MAGIC_ROOK[Square::H1 as usize];
    let shift = SHIFT_ROOK[Square::H1 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H1 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A2: [Bitboard; (1 << SHIFT_ROOK[Square::A2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A2 as usize]) as usize];
    let mask = MASK_ROOK[Square::A2 as usize];
    let magic = MAGIC_ROOK[Square::A2 as usize];
    let shift = SHIFT_ROOK[Square::A2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B2: [Bitboard; (1 << SHIFT_ROOK[Square::B2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B2 as usize]) as usize];
    let mask = MASK_ROOK[Square::B2 as usize];
    let magic = MAGIC_ROOK[Square::B2 as usize];
    let shift = SHIFT_ROOK[Square::B2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C2: [Bitboard; (1 << SHIFT_ROOK[Square::C2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C2 as usize]) as usize];
    let mask = MASK_ROOK[Square::C2 as usize];
    let magic = MAGIC_ROOK[Square::C2 as usize];
    let shift = SHIFT_ROOK[Square::C2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D2: [Bitboard; (1 << SHIFT_ROOK[Square::D2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D2 as usize]) as usize];
    let mask = MASK_ROOK[Square::D2 as usize];
    let magic = MAGIC_ROOK[Square::D2 as usize];
    let shift = SHIFT_ROOK[Square::D2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E2: [Bitboard; (1 << SHIFT_ROOK[Square::E2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E2 as usize]) as usize];
    let mask = MASK_ROOK[Square::E2 as usize];
    let magic = MAGIC_ROOK[Square::E2 as usize];
    let shift = SHIFT_ROOK[Square::E2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F2: [Bitboard; (1 << SHIFT_ROOK[Square::F2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F2 as usize]) as usize];
    let mask = MASK_ROOK[Square::F2 as usize];
    let magic = MAGIC_ROOK[Square::F2 as usize];
    let shift = SHIFT_ROOK[Square::F2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G2: [Bitboard; (1 << SHIFT_ROOK[Square::G2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G2 as usize]) as usize];
    let mask = MASK_ROOK[Square::G2 as usize];
    let magic = MAGIC_ROOK[Square::G2 as usize];
    let shift = SHIFT_ROOK[Square::G2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H2: [Bitboard; (1 << SHIFT_ROOK[Square::H2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H2 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H2 as usize]) as usize];
    let mask = MASK_ROOK[Square::H2 as usize];
    let magic = MAGIC_ROOK[Square::H2 as usize];
    let shift = SHIFT_ROOK[Square::H2 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H2 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A3: [Bitboard; (1 << SHIFT_ROOK[Square::A3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A3 as usize]) as usize];
    let mask = MASK_ROOK[Square::A3 as usize];
    let magic = MAGIC_ROOK[Square::A3 as usize];
    let shift = SHIFT_ROOK[Square::A3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B3: [Bitboard; (1 << SHIFT_ROOK[Square::B3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B3 as usize]) as usize];
    let mask = MASK_ROOK[Square::B3 as usize];
    let magic = MAGIC_ROOK[Square::B3 as usize];
    let shift = SHIFT_ROOK[Square::B3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C3: [Bitboard; (1 << SHIFT_ROOK[Square::C3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C3 as usize]) as usize];
    let mask = MASK_ROOK[Square::C3 as usize];
    let magic = MAGIC_ROOK[Square::C3 as usize];
    let shift = SHIFT_ROOK[Square::C3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D3: [Bitboard; (1 << SHIFT_ROOK[Square::D3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D3 as usize]) as usize];
    let mask = MASK_ROOK[Square::D3 as usize];
    let magic = MAGIC_ROOK[Square::D3 as usize];
    let shift = SHIFT_ROOK[Square::D3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E3: [Bitboard; (1 << SHIFT_ROOK[Square::E3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E3 as usize]) as usize];
    let mask = MASK_ROOK[Square::E3 as usize];
    let magic = MAGIC_ROOK[Square::E3 as usize];
    let shift = SHIFT_ROOK[Square::E3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F3: [Bitboard; (1 << SHIFT_ROOK[Square::F3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F3 as usize]) as usize];
    let mask = MASK_ROOK[Square::F3 as usize];
    let magic = MAGIC_ROOK[Square::F3 as usize];
    let shift = SHIFT_ROOK[Square::F3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G3: [Bitboard; (1 << SHIFT_ROOK[Square::G3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G3 as usize]) as usize];
    let mask = MASK_ROOK[Square::G3 as usize];
    let magic = MAGIC_ROOK[Square::G3 as usize];
    let shift = SHIFT_ROOK[Square::G3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H3: [Bitboard; (1 << SHIFT_ROOK[Square::H3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H3 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H3 as usize]) as usize];
    let mask = MASK_ROOK[Square::H3 as usize];
    let magic = MAGIC_ROOK[Square::H3 as usize];
    let shift = SHIFT_ROOK[Square::H3 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H3 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A4: [Bitboard; (1 << SHIFT_ROOK[Square::A4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A4 as usize]) as usize];
    let mask = MASK_ROOK[Square::A4 as usize];
    let magic = MAGIC_ROOK[Square::A4 as usize];
    let shift = SHIFT_ROOK[Square::A4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B4: [Bitboard; (1 << SHIFT_ROOK[Square::B4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B4 as usize]) as usize];
    let mask = MASK_ROOK[Square::B4 as usize];
    let magic = MAGIC_ROOK[Square::B4 as usize];
    let shift = SHIFT_ROOK[Square::B4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C4: [Bitboard; (1 << SHIFT_ROOK[Square::C4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C4 as usize]) as usize];
    let mask = MASK_ROOK[Square::C4 as usize];
    let magic = MAGIC_ROOK[Square::C4 as usize];
    let shift = SHIFT_ROOK[Square::C4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D4: [Bitboard; (1 << SHIFT_ROOK[Square::D4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D4 as usize]) as usize];
    let mask = MASK_ROOK[Square::D4 as usize];
    let magic = MAGIC_ROOK[Square::D4 as usize];
    let shift = SHIFT_ROOK[Square::D4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E4: [Bitboard; (1 << SHIFT_ROOK[Square::E4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E4 as usize]) as usize];
    let mask = MASK_ROOK[Square::E4 as usize];
    let magic = MAGIC_ROOK[Square::E4 as usize];
    let shift = SHIFT_ROOK[Square::E4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F4: [Bitboard; (1 << SHIFT_ROOK[Square::F4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F4 as usize]) as usize];
    let mask = MASK_ROOK[Square::F4 as usize];
    let magic = MAGIC_ROOK[Square::F4 as usize];
    let shift = SHIFT_ROOK[Square::F4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G4: [Bitboard; (1 << SHIFT_ROOK[Square::G4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G4 as usize]) as usize];
    let mask = MASK_ROOK[Square::G4 as usize];
    let magic = MAGIC_ROOK[Square::G4 as usize];
    let shift = SHIFT_ROOK[Square::G4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H4: [Bitboard; (1 << SHIFT_ROOK[Square::H4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H4 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H4 as usize]) as usize];
    let mask = MASK_ROOK[Square::H4 as usize];
    let magic = MAGIC_ROOK[Square::H4 as usize];
    let shift = SHIFT_ROOK[Square::H4 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H4 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A5: [Bitboard; (1 << SHIFT_ROOK[Square::A5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A5 as usize]) as usize];
    let mask = MASK_ROOK[Square::A5 as usize];
    let magic = MAGIC_ROOK[Square::A5 as usize];
    let shift = SHIFT_ROOK[Square::A5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B5: [Bitboard; (1 << SHIFT_ROOK[Square::B5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B5 as usize]) as usize];
    let mask = MASK_ROOK[Square::B5 as usize];
    let magic = MAGIC_ROOK[Square::B5 as usize];
    let shift = SHIFT_ROOK[Square::B5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C5: [Bitboard; (1 << SHIFT_ROOK[Square::C5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C5 as usize]) as usize];
    let mask = MASK_ROOK[Square::C5 as usize];
    let magic = MAGIC_ROOK[Square::C5 as usize];
    let shift = SHIFT_ROOK[Square::C5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D5: [Bitboard; (1 << SHIFT_ROOK[Square::D5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D5 as usize]) as usize];
    let mask = MASK_ROOK[Square::D5 as usize];
    let magic = MAGIC_ROOK[Square::D5 as usize];
    let shift = SHIFT_ROOK[Square::D5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E5: [Bitboard; (1 << SHIFT_ROOK[Square::E5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E5 as usize]) as usize];
    let mask = MASK_ROOK[Square::E5 as usize];
    let magic = MAGIC_ROOK[Square::E5 as usize];
    let shift = SHIFT_ROOK[Square::E5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F5: [Bitboard; (1 << SHIFT_ROOK[Square::F5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F5 as usize]) as usize];
    let mask = MASK_ROOK[Square::F5 as usize];
    let magic = MAGIC_ROOK[Square::F5 as usize];
    let shift = SHIFT_ROOK[Square::F5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G5: [Bitboard; (1 << SHIFT_ROOK[Square::G5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G5 as usize]) as usize];
    let mask = MASK_ROOK[Square::G5 as usize];
    let magic = MAGIC_ROOK[Square::G5 as usize];
    let shift = SHIFT_ROOK[Square::G5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H5: [Bitboard; (1 << SHIFT_ROOK[Square::H5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H5 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H5 as usize]) as usize];
    let mask = MASK_ROOK[Square::H5 as usize];
    let magic = MAGIC_ROOK[Square::H5 as usize];
    let shift = SHIFT_ROOK[Square::H5 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H5 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A6: [Bitboard; (1 << SHIFT_ROOK[Square::A6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A6 as usize]) as usize];
    let mask = MASK_ROOK[Square::A6 as usize];
    let magic = MAGIC_ROOK[Square::A6 as usize];
    let shift = SHIFT_ROOK[Square::A6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B6: [Bitboard; (1 << SHIFT_ROOK[Square::B6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B6 as usize]) as usize];
    let mask = MASK_ROOK[Square::B6 as usize];
    let magic = MAGIC_ROOK[Square::B6 as usize];
    let shift = SHIFT_ROOK[Square::B6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C6: [Bitboard; (1 << SHIFT_ROOK[Square::C6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C6 as usize]) as usize];
    let mask = MASK_ROOK[Square::C6 as usize];
    let magic = MAGIC_ROOK[Square::C6 as usize];
    let shift = SHIFT_ROOK[Square::C6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D6: [Bitboard; (1 << SHIFT_ROOK[Square::D6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D6 as usize]) as usize];
    let mask = MASK_ROOK[Square::D6 as usize];
    let magic = MAGIC_ROOK[Square::D6 as usize];
    let shift = SHIFT_ROOK[Square::D6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E6: [Bitboard; (1 << SHIFT_ROOK[Square::E6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E6 as usize]) as usize];
    let mask = MASK_ROOK[Square::E6 as usize];
    let magic = MAGIC_ROOK[Square::E6 as usize];
    let shift = SHIFT_ROOK[Square::E6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F6: [Bitboard; (1 << SHIFT_ROOK[Square::F6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F6 as usize]) as usize];
    let mask = MASK_ROOK[Square::F6 as usize];
    let magic = MAGIC_ROOK[Square::F6 as usize];
    let shift = SHIFT_ROOK[Square::F6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G6: [Bitboard; (1 << SHIFT_ROOK[Square::G6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G6 as usize]) as usize];
    let mask = MASK_ROOK[Square::G6 as usize];
    let magic = MAGIC_ROOK[Square::G6 as usize];
    let shift = SHIFT_ROOK[Square::G6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H6: [Bitboard; (1 << SHIFT_ROOK[Square::H6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H6 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H6 as usize]) as usize];
    let mask = MASK_ROOK[Square::H6 as usize];
    let magic = MAGIC_ROOK[Square::H6 as usize];
    let shift = SHIFT_ROOK[Square::H6 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H6 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A7: [Bitboard; (1 << SHIFT_ROOK[Square::A7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A7 as usize]) as usize];
    let mask = MASK_ROOK[Square::A7 as usize];
    let magic = MAGIC_ROOK[Square::A7 as usize];
    let shift = SHIFT_ROOK[Square::A7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B7: [Bitboard; (1 << SHIFT_ROOK[Square::B7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B7 as usize]) as usize];
    let mask = MASK_ROOK[Square::B7 as usize];
    let magic = MAGIC_ROOK[Square::B7 as usize];
    let shift = SHIFT_ROOK[Square::B7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C7: [Bitboard; (1 << SHIFT_ROOK[Square::C7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C7 as usize]) as usize];
    let mask = MASK_ROOK[Square::C7 as usize];
    let magic = MAGIC_ROOK[Square::C7 as usize];
    let shift = SHIFT_ROOK[Square::C7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D7: [Bitboard; (1 << SHIFT_ROOK[Square::D7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D7 as usize]) as usize];
    let mask = MASK_ROOK[Square::D7 as usize];
    let magic = MAGIC_ROOK[Square::D7 as usize];
    let shift = SHIFT_ROOK[Square::D7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E7: [Bitboard; (1 << SHIFT_ROOK[Square::E7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E7 as usize]) as usize];
    let mask = MASK_ROOK[Square::E7 as usize];
    let magic = MAGIC_ROOK[Square::E7 as usize];
    let shift = SHIFT_ROOK[Square::E7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F7: [Bitboard; (1 << SHIFT_ROOK[Square::F7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F7 as usize]) as usize];
    let mask = MASK_ROOK[Square::F7 as usize];
    let magic = MAGIC_ROOK[Square::F7 as usize];
    let shift = SHIFT_ROOK[Square::F7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G7: [Bitboard; (1 << SHIFT_ROOK[Square::G7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G7 as usize]) as usize];
    let mask = MASK_ROOK[Square::G7 as usize];
    let magic = MAGIC_ROOK[Square::G7 as usize];
    let shift = SHIFT_ROOK[Square::G7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H7: [Bitboard; (1 << SHIFT_ROOK[Square::H7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H7 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H7 as usize]) as usize];
    let mask = MASK_ROOK[Square::H7 as usize];
    let magic = MAGIC_ROOK[Square::H7 as usize];
    let shift = SHIFT_ROOK[Square::H7 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H7 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_A8: [Bitboard; (1 << SHIFT_ROOK[Square::A8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::A8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::A8 as usize]) as usize];
    let mask = MASK_ROOK[Square::A8 as usize];
    let magic = MAGIC_ROOK[Square::A8 as usize];
    let shift = SHIFT_ROOK[Square::A8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::A8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_B8: [Bitboard; (1 << SHIFT_ROOK[Square::B8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::B8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::B8 as usize]) as usize];
    let mask = MASK_ROOK[Square::B8 as usize];
    let magic = MAGIC_ROOK[Square::B8 as usize];
    let shift = SHIFT_ROOK[Square::B8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::B8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_C8: [Bitboard; (1 << SHIFT_ROOK[Square::C8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::C8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::C8 as usize]) as usize];
    let mask = MASK_ROOK[Square::C8 as usize];
    let magic = MAGIC_ROOK[Square::C8 as usize];
    let shift = SHIFT_ROOK[Square::C8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::C8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_D8: [Bitboard; (1 << SHIFT_ROOK[Square::D8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::D8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::D8 as usize]) as usize];
    let mask = MASK_ROOK[Square::D8 as usize];
    let magic = MAGIC_ROOK[Square::D8 as usize];
    let shift = SHIFT_ROOK[Square::D8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::D8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_E8: [Bitboard; (1 << SHIFT_ROOK[Square::E8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::E8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::E8 as usize]) as usize];
    let mask = MASK_ROOK[Square::E8 as usize];
    let magic = MAGIC_ROOK[Square::E8 as usize];
    let shift = SHIFT_ROOK[Square::E8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::E8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_F8: [Bitboard; (1 << SHIFT_ROOK[Square::F8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::F8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::F8 as usize]) as usize];
    let mask = MASK_ROOK[Square::F8 as usize];
    let magic = MAGIC_ROOK[Square::F8 as usize];
    let shift = SHIFT_ROOK[Square::F8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::F8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_G8: [Bitboard; (1 << SHIFT_ROOK[Square::G8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::G8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::G8 as usize]) as usize];
    let mask = MASK_ROOK[Square::G8 as usize];
    let magic = MAGIC_ROOK[Square::G8 as usize];
    let shift = SHIFT_ROOK[Square::G8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::G8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};
const ROOK_BB_H8: [Bitboard; (1 << SHIFT_ROOK[Square::H8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_ROOK[Square::H8 as usize]) as usize] = [0u64; (1 << SHIFT_ROOK[Square::H8 as usize]) as usize];
    let mask = MASK_ROOK[Square::H8 as usize];
    let magic = MAGIC_ROOK[Square::H8 as usize];
    let shift = SHIFT_ROOK[Square::H8 as usize];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];
    let mut i = 0;
    while i < (1u64 << mask.count_ones()){
        let mut blockers = 0u64;
        let mut mask_copy = mask;
        let mut bits:u64 = i;
        while mask_copy != 0 {
            let lsb= mask_copy.trailing_zeros();
            blockers |= (bits & 0b1u64) << lsb;
            mask_copy &= mask_copy - 1;
            bits >>= 1;
        }
        let index = blockers.wrapping_mul(magic) >> (64 - shift);

        let mut bb = 0u64;
        let mut j = 0;
        while j < 4 {
            let mut from = Square::H8 as i8;
            let mut to = from + STEPS[j];
            'steploop: while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                let to_bb = 1u64 << to;
                bb |= to_bb;
                if (to_bb & blockers) != 0 { break 'steploop; }
                from = to;
                to += STEPS[j];
            }
            j += 1;
        }

        result[index as usize] = bb;
        i+= 1;
    }
    result
};