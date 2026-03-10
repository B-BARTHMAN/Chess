use crate::types::{Bitboard, Square};

#[inline(always)]
pub fn bishop_attacks(square: Square, blockers: Bitboard) -> Bitboard {
    let mask = MASK_BISHOP[square as usize];
    let magic = MAGIC_BISHOP[square as usize];
    let shift = SHIFT_BISHOP[square as usize];

    let index = (blockers & mask).wrapping_mul(magic) >> (64 - shift);
    lookup(square, index as usize)
}
#[inline(always)]
const fn lookup(square: Square, index: usize) -> Bitboard {
    match square{
        Square::SquareNone => {0u64}
        Square::A1 => {BISHOP_BB_A1[index]}
        Square::B1 => {BISHOP_BB_B1[index]}
        Square::C1 => {BISHOP_BB_C1[index]}
        Square::D1 => {BISHOP_BB_D1[index]}
        Square::E1 => {BISHOP_BB_E1[index]}
        Square::F1 => {BISHOP_BB_F1[index]}
        Square::G1 => {BISHOP_BB_G1[index]}
        Square::H1 => {BISHOP_BB_H1[index]}
        Square::A2 => {BISHOP_BB_A2[index]}
        Square::B2 => {BISHOP_BB_B2[index]}
        Square::C2 => {BISHOP_BB_C2[index]}
        Square::D2 => {BISHOP_BB_D2[index]}
        Square::E2 => {BISHOP_BB_E2[index]}
        Square::F2 => {BISHOP_BB_F2[index]}
        Square::G2 => {BISHOP_BB_G2[index]}
        Square::H2 => {BISHOP_BB_H2[index]}
        Square::A3 => {BISHOP_BB_A3[index]}
        Square::B3 => {BISHOP_BB_B3[index]}
        Square::C3 => {BISHOP_BB_C3[index]}
        Square::D3 => {BISHOP_BB_D3[index]}
        Square::E3 => {BISHOP_BB_E3[index]}
        Square::F3 => {BISHOP_BB_F3[index]}
        Square::G3 => {BISHOP_BB_G3[index]}
        Square::H3 => {BISHOP_BB_H3[index]}
        Square::A4 => {BISHOP_BB_A4[index]}
        Square::B4 => {BISHOP_BB_B4[index]}
        Square::C4 => {BISHOP_BB_C4[index]}
        Square::D4 => {BISHOP_BB_D4[index]}
        Square::E4 => {BISHOP_BB_E4[index]}
        Square::F4 => {BISHOP_BB_F4[index]}
        Square::G4 => {BISHOP_BB_G4[index]}
        Square::H4 => {BISHOP_BB_H4[index]}
        Square::A5 => {BISHOP_BB_A5[index]}
        Square::B5 => {BISHOP_BB_B5[index]}
        Square::C5 => {BISHOP_BB_C5[index]}
        Square::D5 => {BISHOP_BB_D5[index]}
        Square::E5 => {BISHOP_BB_E5[index]}
        Square::F5 => {BISHOP_BB_F5[index]}
        Square::G5 => {BISHOP_BB_G5[index]}
        Square::H5 => {BISHOP_BB_H5[index]}
        Square::A6 => {BISHOP_BB_A6[index]}
        Square::B6 => {BISHOP_BB_B6[index]}
        Square::C6 => {BISHOP_BB_C6[index]}
        Square::D6 => {BISHOP_BB_D6[index]}
        Square::E6 => {BISHOP_BB_E6[index]}
        Square::F6 => {BISHOP_BB_F6[index]}
        Square::G6 => {BISHOP_BB_G6[index]}
        Square::H6 => {BISHOP_BB_H6[index]}
        Square::A7 => {BISHOP_BB_A7[index]}
        Square::B7 => {BISHOP_BB_B7[index]}
        Square::C7 => {BISHOP_BB_C7[index]}
        Square::D7 => {BISHOP_BB_D7[index]}
        Square::E7 => {BISHOP_BB_E7[index]}
        Square::F7 => {BISHOP_BB_F7[index]}
        Square::G7 => {BISHOP_BB_G7[index]}
        Square::H7 => {BISHOP_BB_H7[index]}
        Square::A8 => {BISHOP_BB_A8[index]}
        Square::B8 => {BISHOP_BB_B8[index]}
        Square::C8 => {BISHOP_BB_C8[index]}
        Square::D8 => {BISHOP_BB_D8[index]}
        Square::E8 => {BISHOP_BB_E8[index]}
        Square::F8 => {BISHOP_BB_F8[index]}
        Square::G8 => {BISHOP_BB_G8[index]}
        Square::H8 => {BISHOP_BB_H8[index]}
        Square::SquareNb => {0u64}
    }
}
const MASK_BISHOP: [Bitboard; Square::SquareNb as usize] = {
    let mut result: [Bitboard; Square::SquareNb as usize] = [0u64; Square::SquareNb as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];

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
const MAGIC_BISHOP: [u64; Square::SquareNb as usize] = {[
    0xd358eacff6ef9bff, 0xb90589d7cab7fa4a, 0x719cc7cf93f9697e, 0x7c580e11420ccde2, 0x1e7c1c214bbe01d0, 0x68a0c9e57e1fe834, 0x5462963bfa7f262d, 0x9a5ae90af697ffbb,
    0xba469bc6b199bff6, 0x6476da8a4def1bfb, 0xcfd5700b898d004d, 0x3659b8dc01eb2238, 0x103e940c20522e8b, 0x492aca0f2c205ef8, 0x3bf022cbb1e47fd8, 0x72387cd73994ffeb,
    0xa5c00b74ac95aff2, 0xf6e02ebb064257f8, 0x51e4015202fb47fc, 0x261801ac0e408821, 0x9a76000c011c1e32, 0x4bea017901012943, 0x52bc057353453f76, 0xc3e200f65ebcdfa7,
    0x8bbc6016c0a82d24, 0xb80fefa91d5b3d58, 0x82cbe8089c134400, 0xef7f0041c4040002, 0x324b009001004016, 0xb849420341010d00, 0xf71189941b999f7d, 0xfc3b8e19d78e0f1a,
    0x153f90a68bc06866, 0xf8e4131c4268505d, 0x649e169802b003b4, 0xcf55601801270050, 0x9ff81204005ff100, 0x8001a51a002b008b, 0x8964110bd1820815, 0xf8d4968885b1040d,
    0xfd9fda73cc11c00b, 0x0e17fab63adfa00a, 0x62972061a8107002, 0xcd5c093a44008803, 0x64acc10a1401b602, 0xfec00526ea801100, 0x42bfd57b13324402, 0xd0df9abd9ba84e00,
    0x4b97ee2e99e48ac0, 0x2f8ffede13b18f5c, 0x6bcbda0329881782, 0xb982110561980558, 0x636e3471420602bd, 0x6edfdc901e1a0497, 0x2e7f0babf368c08c, 0xcd3fe9c560975a60,
    0x3ea7fd3ab012e004, 0x8ca713faf65d3515, 0xc1d5e5d3ee3ef0a6, 0xe8d69f38a120982b, 0x200058b92024a40a, 0x034a47aa565d7e06, 0xe1dc7f737ae820b3, 0x7e7fe074d26c524a,
]};
const SHIFT_BISHOP: [u8; Square::SquareNb as usize] = {[
    5, 4, 5, 5, 5, 5, 4, 5,
    4, 4, 5, 5, 5, 5, 4, 4,
    4, 4, 7, 7, 7, 7, 4, 4,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    4, 4, 7, 7, 7, 7, 4, 4,
    4, 4, 5, 5, 5, 5, 4, 4,
    5, 4, 5, 5, 5, 5, 4, 5,
]};
const BISHOP_BB_A1: [Bitboard; (1 << SHIFT_BISHOP[Square::A1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A1 as usize];
    let magic = MAGIC_BISHOP[Square::A1 as usize];
    let shift = SHIFT_BISHOP[Square::A1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B1: [Bitboard; (1 << SHIFT_BISHOP[Square::B1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B1 as usize];
    let magic = MAGIC_BISHOP[Square::B1 as usize];
    let shift = SHIFT_BISHOP[Square::B1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C1: [Bitboard; (1 << SHIFT_BISHOP[Square::C1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C1 as usize];
    let magic = MAGIC_BISHOP[Square::C1 as usize];
    let shift = SHIFT_BISHOP[Square::C1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D1: [Bitboard; (1 << SHIFT_BISHOP[Square::D1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D1 as usize];
    let magic = MAGIC_BISHOP[Square::D1 as usize];
    let shift = SHIFT_BISHOP[Square::D1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E1: [Bitboard; (1 << SHIFT_BISHOP[Square::E1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E1 as usize];
    let magic = MAGIC_BISHOP[Square::E1 as usize];
    let shift = SHIFT_BISHOP[Square::E1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F1: [Bitboard; (1 << SHIFT_BISHOP[Square::F1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F1 as usize];
    let magic = MAGIC_BISHOP[Square::F1 as usize];
    let shift = SHIFT_BISHOP[Square::F1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G1: [Bitboard; (1 << SHIFT_BISHOP[Square::G1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G1 as usize];
    let magic = MAGIC_BISHOP[Square::G1 as usize];
    let shift = SHIFT_BISHOP[Square::G1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H1: [Bitboard; (1 << SHIFT_BISHOP[Square::H1 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H1 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H1 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H1 as usize];
    let magic = MAGIC_BISHOP[Square::H1 as usize];
    let shift = SHIFT_BISHOP[Square::H1 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A2: [Bitboard; (1 << SHIFT_BISHOP[Square::A2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A2 as usize];
    let magic = MAGIC_BISHOP[Square::A2 as usize];
    let shift = SHIFT_BISHOP[Square::A2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B2: [Bitboard; (1 << SHIFT_BISHOP[Square::B2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B2 as usize];
    let magic = MAGIC_BISHOP[Square::B2 as usize];
    let shift = SHIFT_BISHOP[Square::B2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C2: [Bitboard; (1 << SHIFT_BISHOP[Square::C2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C2 as usize];
    let magic = MAGIC_BISHOP[Square::C2 as usize];
    let shift = SHIFT_BISHOP[Square::C2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D2: [Bitboard; (1 << SHIFT_BISHOP[Square::D2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D2 as usize];
    let magic = MAGIC_BISHOP[Square::D2 as usize];
    let shift = SHIFT_BISHOP[Square::D2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E2: [Bitboard; (1 << SHIFT_BISHOP[Square::E2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E2 as usize];
    let magic = MAGIC_BISHOP[Square::E2 as usize];
    let shift = SHIFT_BISHOP[Square::E2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F2: [Bitboard; (1 << SHIFT_BISHOP[Square::F2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F2 as usize];
    let magic = MAGIC_BISHOP[Square::F2 as usize];
    let shift = SHIFT_BISHOP[Square::F2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G2: [Bitboard; (1 << SHIFT_BISHOP[Square::G2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G2 as usize];
    let magic = MAGIC_BISHOP[Square::G2 as usize];
    let shift = SHIFT_BISHOP[Square::G2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H2: [Bitboard; (1 << SHIFT_BISHOP[Square::H2 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H2 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H2 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H2 as usize];
    let magic = MAGIC_BISHOP[Square::H2 as usize];
    let shift = SHIFT_BISHOP[Square::H2 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A3: [Bitboard; (1 << SHIFT_BISHOP[Square::A3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A3 as usize];
    let magic = MAGIC_BISHOP[Square::A3 as usize];
    let shift = SHIFT_BISHOP[Square::A3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B3: [Bitboard; (1 << SHIFT_BISHOP[Square::B3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B3 as usize];
    let magic = MAGIC_BISHOP[Square::B3 as usize];
    let shift = SHIFT_BISHOP[Square::B3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C3: [Bitboard; (1 << SHIFT_BISHOP[Square::C3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C3 as usize];
    let magic = MAGIC_BISHOP[Square::C3 as usize];
    let shift = SHIFT_BISHOP[Square::C3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D3: [Bitboard; (1 << SHIFT_BISHOP[Square::D3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D3 as usize];
    let magic = MAGIC_BISHOP[Square::D3 as usize];
    let shift = SHIFT_BISHOP[Square::D3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E3: [Bitboard; (1 << SHIFT_BISHOP[Square::E3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E3 as usize];
    let magic = MAGIC_BISHOP[Square::E3 as usize];
    let shift = SHIFT_BISHOP[Square::E3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F3: [Bitboard; (1 << SHIFT_BISHOP[Square::F3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F3 as usize];
    let magic = MAGIC_BISHOP[Square::F3 as usize];
    let shift = SHIFT_BISHOP[Square::F3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G3: [Bitboard; (1 << SHIFT_BISHOP[Square::G3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G3 as usize];
    let magic = MAGIC_BISHOP[Square::G3 as usize];
    let shift = SHIFT_BISHOP[Square::G3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H3: [Bitboard; (1 << SHIFT_BISHOP[Square::H3 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H3 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H3 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H3 as usize];
    let magic = MAGIC_BISHOP[Square::H3 as usize];
    let shift = SHIFT_BISHOP[Square::H3 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A4: [Bitboard; (1 << SHIFT_BISHOP[Square::A4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A4 as usize];
    let magic = MAGIC_BISHOP[Square::A4 as usize];
    let shift = SHIFT_BISHOP[Square::A4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B4: [Bitboard; (1 << SHIFT_BISHOP[Square::B4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B4 as usize];
    let magic = MAGIC_BISHOP[Square::B4 as usize];
    let shift = SHIFT_BISHOP[Square::B4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C4: [Bitboard; (1 << SHIFT_BISHOP[Square::C4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C4 as usize];
    let magic = MAGIC_BISHOP[Square::C4 as usize];
    let shift = SHIFT_BISHOP[Square::C4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D4: [Bitboard; (1 << SHIFT_BISHOP[Square::D4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D4 as usize];
    let magic = MAGIC_BISHOP[Square::D4 as usize];
    let shift = SHIFT_BISHOP[Square::D4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E4: [Bitboard; (1 << SHIFT_BISHOP[Square::E4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E4 as usize];
    let magic = MAGIC_BISHOP[Square::E4 as usize];
    let shift = SHIFT_BISHOP[Square::E4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F4: [Bitboard; (1 << SHIFT_BISHOP[Square::F4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F4 as usize];
    let magic = MAGIC_BISHOP[Square::F4 as usize];
    let shift = SHIFT_BISHOP[Square::F4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G4: [Bitboard; (1 << SHIFT_BISHOP[Square::G4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G4 as usize];
    let magic = MAGIC_BISHOP[Square::G4 as usize];
    let shift = SHIFT_BISHOP[Square::G4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H4: [Bitboard; (1 << SHIFT_BISHOP[Square::H4 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H4 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H4 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H4 as usize];
    let magic = MAGIC_BISHOP[Square::H4 as usize];
    let shift = SHIFT_BISHOP[Square::H4 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A5: [Bitboard; (1 << SHIFT_BISHOP[Square::A5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A5 as usize];
    let magic = MAGIC_BISHOP[Square::A5 as usize];
    let shift = SHIFT_BISHOP[Square::A5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B5: [Bitboard; (1 << SHIFT_BISHOP[Square::B5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B5 as usize];
    let magic = MAGIC_BISHOP[Square::B5 as usize];
    let shift = SHIFT_BISHOP[Square::B5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C5: [Bitboard; (1 << SHIFT_BISHOP[Square::C5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C5 as usize];
    let magic = MAGIC_BISHOP[Square::C5 as usize];
    let shift = SHIFT_BISHOP[Square::C5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D5: [Bitboard; (1 << SHIFT_BISHOP[Square::D5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D5 as usize];
    let magic = MAGIC_BISHOP[Square::D5 as usize];
    let shift = SHIFT_BISHOP[Square::D5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E5: [Bitboard; (1 << SHIFT_BISHOP[Square::E5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E5 as usize];
    let magic = MAGIC_BISHOP[Square::E5 as usize];
    let shift = SHIFT_BISHOP[Square::E5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F5: [Bitboard; (1 << SHIFT_BISHOP[Square::F5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F5 as usize];
    let magic = MAGIC_BISHOP[Square::F5 as usize];
    let shift = SHIFT_BISHOP[Square::F5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G5: [Bitboard; (1 << SHIFT_BISHOP[Square::G5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G5 as usize];
    let magic = MAGIC_BISHOP[Square::G5 as usize];
    let shift = SHIFT_BISHOP[Square::G5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H5: [Bitboard; (1 << SHIFT_BISHOP[Square::H5 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H5 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H5 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H5 as usize];
    let magic = MAGIC_BISHOP[Square::H5 as usize];
    let shift = SHIFT_BISHOP[Square::H5 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A6: [Bitboard; (1 << SHIFT_BISHOP[Square::A6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A6 as usize];
    let magic = MAGIC_BISHOP[Square::A6 as usize];
    let shift = SHIFT_BISHOP[Square::A6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B6: [Bitboard; (1 << SHIFT_BISHOP[Square::B6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B6 as usize];
    let magic = MAGIC_BISHOP[Square::B6 as usize];
    let shift = SHIFT_BISHOP[Square::B6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C6: [Bitboard; (1 << SHIFT_BISHOP[Square::C6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C6 as usize];
    let magic = MAGIC_BISHOP[Square::C6 as usize];
    let shift = SHIFT_BISHOP[Square::C6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D6: [Bitboard; (1 << SHIFT_BISHOP[Square::D6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D6 as usize];
    let magic = MAGIC_BISHOP[Square::D6 as usize];
    let shift = SHIFT_BISHOP[Square::D6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E6: [Bitboard; (1 << SHIFT_BISHOP[Square::E6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E6 as usize];
    let magic = MAGIC_BISHOP[Square::E6 as usize];
    let shift = SHIFT_BISHOP[Square::E6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F6: [Bitboard; (1 << SHIFT_BISHOP[Square::F6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F6 as usize];
    let magic = MAGIC_BISHOP[Square::F6 as usize];
    let shift = SHIFT_BISHOP[Square::F6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G6: [Bitboard; (1 << SHIFT_BISHOP[Square::G6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G6 as usize];
    let magic = MAGIC_BISHOP[Square::G6 as usize];
    let shift = SHIFT_BISHOP[Square::G6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H6: [Bitboard; (1 << SHIFT_BISHOP[Square::H6 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H6 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H6 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H6 as usize];
    let magic = MAGIC_BISHOP[Square::H6 as usize];
    let shift = SHIFT_BISHOP[Square::H6 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A7: [Bitboard; (1 << SHIFT_BISHOP[Square::A7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A7 as usize];
    let magic = MAGIC_BISHOP[Square::A7 as usize];
    let shift = SHIFT_BISHOP[Square::A7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B7: [Bitboard; (1 << SHIFT_BISHOP[Square::B7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B7 as usize];
    let magic = MAGIC_BISHOP[Square::B7 as usize];
    let shift = SHIFT_BISHOP[Square::B7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C7: [Bitboard; (1 << SHIFT_BISHOP[Square::C7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C7 as usize];
    let magic = MAGIC_BISHOP[Square::C7 as usize];
    let shift = SHIFT_BISHOP[Square::C7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D7: [Bitboard; (1 << SHIFT_BISHOP[Square::D7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D7 as usize];
    let magic = MAGIC_BISHOP[Square::D7 as usize];
    let shift = SHIFT_BISHOP[Square::D7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E7: [Bitboard; (1 << SHIFT_BISHOP[Square::E7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E7 as usize];
    let magic = MAGIC_BISHOP[Square::E7 as usize];
    let shift = SHIFT_BISHOP[Square::E7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F7: [Bitboard; (1 << SHIFT_BISHOP[Square::F7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F7 as usize];
    let magic = MAGIC_BISHOP[Square::F7 as usize];
    let shift = SHIFT_BISHOP[Square::F7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G7: [Bitboard; (1 << SHIFT_BISHOP[Square::G7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G7 as usize];
    let magic = MAGIC_BISHOP[Square::G7 as usize];
    let shift = SHIFT_BISHOP[Square::G7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H7: [Bitboard; (1 << SHIFT_BISHOP[Square::H7 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H7 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H7 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H7 as usize];
    let magic = MAGIC_BISHOP[Square::H7 as usize];
    let shift = SHIFT_BISHOP[Square::H7 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_A8: [Bitboard; (1 << SHIFT_BISHOP[Square::A8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::A8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::A8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::A8 as usize];
    let magic = MAGIC_BISHOP[Square::A8 as usize];
    let shift = SHIFT_BISHOP[Square::A8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_B8: [Bitboard; (1 << SHIFT_BISHOP[Square::B8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::B8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::B8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::B8 as usize];
    let magic = MAGIC_BISHOP[Square::B8 as usize];
    let shift = SHIFT_BISHOP[Square::B8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_C8: [Bitboard; (1 << SHIFT_BISHOP[Square::C8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::C8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::C8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::C8 as usize];
    let magic = MAGIC_BISHOP[Square::C8 as usize];
    let shift = SHIFT_BISHOP[Square::C8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_D8: [Bitboard; (1 << SHIFT_BISHOP[Square::D8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::D8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::D8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::D8 as usize];
    let magic = MAGIC_BISHOP[Square::D8 as usize];
    let shift = SHIFT_BISHOP[Square::D8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_E8: [Bitboard; (1 << SHIFT_BISHOP[Square::E8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::E8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::E8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::E8 as usize];
    let magic = MAGIC_BISHOP[Square::E8 as usize];
    let shift = SHIFT_BISHOP[Square::E8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_F8: [Bitboard; (1 << SHIFT_BISHOP[Square::F8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::F8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::F8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::F8 as usize];
    let magic = MAGIC_BISHOP[Square::F8 as usize];
    let shift = SHIFT_BISHOP[Square::F8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_G8: [Bitboard; (1 << SHIFT_BISHOP[Square::G8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::G8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::G8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::G8 as usize];
    let magic = MAGIC_BISHOP[Square::G8 as usize];
    let shift = SHIFT_BISHOP[Square::G8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
const BISHOP_BB_H8: [Bitboard; (1 << SHIFT_BISHOP[Square::H8 as usize]) as usize] = {
    let mut result: [Bitboard; (1 << SHIFT_BISHOP[Square::H8 as usize]) as usize] = [0u64; (1 << SHIFT_BISHOP[Square::H8 as usize]) as usize];
    let mask = MASK_BISHOP[Square::H8 as usize];
    let magic = MAGIC_BISHOP[Square::H8 as usize];
    let shift = SHIFT_BISHOP[Square::H8 as usize];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];
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
