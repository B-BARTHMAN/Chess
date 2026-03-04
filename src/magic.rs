use crate::bitboard::{
    count_1s, file_of, is_ok, lsb, pop_lsb, rank_of, shift_bb, shift_square, square_bb,
};
use crate::types::{Bitboard, Direction, File, PieceType, Rank, Square};

use std::io::Write;

// Rook: 800MB
// Bishop: 41MB
fn generate_mask<const PT: u8>(square: Square) -> Bitboard {
    let mut result = 0u64;

    // Select directions based on the piece type
    let directions: &[(Direction, fn(Square) -> bool)] = match PT {
        x if x == PieceType::Rook as u8 => &[
            (Direction::North, |sq| {
                !is_ok(sq) || rank_of(sq) == Rank::Rank8
            }),
            (Direction::South, |sq| {
                !is_ok(sq) || rank_of(sq) == Rank::Rank1
            }),
            (Direction::West, |sq| {
                !is_ok(sq) || file_of(sq) == File::FileA
            }),
            (Direction::East, |sq| {
                !is_ok(sq) || file_of(sq) == File::FileH
            }),
        ],
        x if x == PieceType::Bishop as u8 => &[
            (Direction::NorthEast, |sq| {
                !is_ok(sq) || rank_of(sq) == Rank::Rank8 || file_of(sq) == File::FileH
            }),
            (Direction::NorthWest, |sq| {
                !is_ok(sq) || rank_of(sq) == Rank::Rank8 || file_of(sq) == File::FileA
            }),
            (Direction::SouthEast, |sq| {
                !is_ok(sq) || rank_of(sq) == Rank::Rank1 || file_of(sq) == File::FileH
            }),
            (Direction::SouthWest, |sq| {
                !is_ok(sq) || rank_of(sq) == Rank::Rank1 || file_of(sq) == File::FileA
            }),
        ],
        _ => &[], // For other piece types, could also panic! or return 0
    };

    for (dir, stop) in directions {
        let mut sq = square;
        while {
            sq = shift_square(sq, *dir);
            !stop(sq)
        } {
            result |= square_bb(sq);
        }
    }

    result
}

fn sliding_attacks<const PT: u8>(square: Square, mask: Bitboard) -> Bitboard {
    debug_assert!(
        PT == PieceType::Rook as u8
            || PT == PieceType::Bishop as u8
            || PT == PieceType::Queen as u8
    );

    let mut result = 0u64;

    let directions: &[Direction] = match PT {
        x if x == PieceType::Bishop as u8 => &[
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ],
        x if x == PieceType::Rook as u8 => &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ],
        x if x == PieceType::Queen as u8 => &[
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ],
        _ => &[],
    };

    for dir in directions {
        let mut bb = square_bb(square);
        loop {
            bb = shift_bb(bb, *dir);
            result |= bb;
            bb &= !mask;

            if bb == 0 {
                break;
            }
        }
    }

    result
}

fn transform_mask(mut mask: Bitboard, mut i: u16) -> Bitboard {
    let mut result = 0u64;
    while mask != 0 {
        let square = pop_lsb(&mut mask);
        result |= square_bb(square) * ((i & 1) as u64);
        i >>= 1;
    }
    result
}

fn test_magic<const PT: u8>(square: Square, mask: Bitboard, magic: u64, bits: u8, used: &mut [bool], slides: &mut [u64], ) -> bool {
    let table_size = 1usize << bits;

    // Reset buffers
    for i in 0..table_size {
        used[i] = false;
    }

    let possibilities = count_1s(mask);

    for i in 0..(1 << possibilities) {
        let occupancy = transform_mask(mask, i as u16);
        let attack = sliding_attacks::<PT>(square, occupancy);

        let index = (occupancy.wrapping_mul(magic)) >> (64 - bits);
        let idx = index as usize;

        if used[idx] {
            if slides[idx] != attack {
                return false;
            }
        } else {
            used[idx] = true;
            slides[idx] = attack;
        }
    }

    true
}

fn find_magic_square<const PT: u8>(square: Square, mask: Bitboard, bits: u8, ) -> Option<u64> {
    let table_size = 1usize << bits;

    // Allocate once per square search
    let mut used = vec![false; table_size];
    let mut slides = vec![0u64; table_size];

    for _ in 0..1_000_0000u64 {
        // Restrict random candidate to relevant bits
        let magic = rand::random::<u64>();

        if test_magic::<PT>(
            square,
            mask,
            magic,
            bits,
            &mut used,
            &mut slides,
        ) {
            return Some(magic);
        }
    }

    None
}

#[inline(always)]
fn max_bits<const PT: u8>(square: Square) -> u8 {
    count_1s(generate_mask::<PT>(square)) + 1
}

pub fn find_magic<const PT: u8>(filepath: &str) {
    let mut magics: [Option<u64>; Square::SquareNb as usize] = [None; Square::SquareNb as usize];
    let mut shifts: [Option<u8>;  Square::SquareNb as usize] = [None; Square::SquareNb as usize];

    let mut masks = [0u64; Square::SquareNb as usize];
    let mut max_bits_arr = [0u8; Square::SquareNb as usize];

    for square in 0..Square::SquareNb as usize {
        let sq = Square::from_index(square as i8);
        let mask = generate_mask::<PT>(sq);
        masks[square] = mask;
        max_bits_arr[square] = count_1s(mask) + 1;
    }

    loop {
        let mut skip_count = 0;
        'forloop: for square in 0..Square::SquareNb as usize {
            let sq = Square::from_index(square as i8);

            let bits = if let Some(existing) = shifts[square] {
                existing - 1
            } else {
                max_bits_arr[square]
            };
            //////////////
            if filepath.contains("rook"){
                if bits < best_rook[sq as usize] {
                    skip_count += 1;
                    if skip_count == Square::SquareNb as usize {
                        return
                    }
                    continue 'forloop;
                }
            }
            else if filepath.contains("bishop"){
                if bits < best_bishop[sq as usize] {
                    skip_count += 1;
                    if skip_count == Square::SquareNb as usize {
                        return
                    }
                    continue 'forloop;
                }
            }
            ////

            /*if bits == (max_bits_arr[square] - 3) {
                continue;
            }*/


            let magic= find_magic_square::<PT>(sq, masks[square], bits);

            if let Some(m) = magic {
                magics[square] = Some(m);
                shifts[square] = Some(bits);
            }
        }

        let found = magics.iter().filter(|m| m.is_some()).count();

        let total_entries: u64 = shifts
            .iter()
            .filter_map(|&s| s)
            .map(|s| 1u64 << s)
            .sum();

        let total_kb = (total_entries * 8) as f64 / 1024.0;

        println!(
            "{filepath}: Found: {}/{} | Estimated table size: {} KB,  skipped {skip_count}",
            found,
            Square::SquareNb as i8,
            total_kb
        );

        if magics.iter().all(|m| m.is_some())
            && shifts.iter().all(|s| s.is_some())
        {
            //println!("All magic numbers found! Saving to file...");

            let mut file =
                std::fs::File::create(filepath).expect("Failed to create file");

            writeln!(file, "Magics:").unwrap();
            for m in magics.iter() {
                writeln!(file, "{}", m.unwrap()).unwrap();
            }

            writeln!(file, "\nShifts:").unwrap();
            for s in shifts.iter() {
                writeln!(file, "{}", s.unwrap()).unwrap();
            }

            //println!("Saved to {}", filepath);
        }

        //println!("Restarting search...\n");
    }
}

const best_bishop: [u8; Square::SquareNb as usize] = [
    5, 4, 5, 5, 5, 5, 4, 5,
    4, 4, 5, 5, 5, 5, 4, 4,
    4, 4, 7, 7, 7, 7, 4, 4,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    4, 4, 7, 7, 7, 7, 4, 4,
    4, 4, 5, 5, 5, 5, 4, 4,
    5, 4, 5, 5, 5, 5, 4, 5
];
const best_rook: [u8; Square::SquareNb as usize] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    10,  9,  9,  9,  9,  9,  9, 10,
    11, 10, 10, 10, 10, 11, 10, 11
];

pub const MAGIC_BISHOP: [u64; Square::SquareNb as usize] = [
    0xffedf9fd7cfcffff, 0xfc0962854a77f576, 0x73501c21c9d00f33, 0x66841c0f822ccb7d, 0x50d60a101bdf5d70, 0xa4260170ffb15c3b, 0xfc0a66c64a7ef576, 0x7ffdfdfcbd79ffff,
    0xfc0846a64a34fff6, 0xfc087a874a3cf7f6, 0xb123d4e5f6a789be, 0xee0a1b2c3d4e5f00, 0x3439a3b179c4d8ba, 0x8721bc45defa0191, 0xfc0864ae59b4ff76, 0x3c0860af4b35ff76,
    0x73c01af56cf4cffb, 0x41a01cfad64aaffc, 0x375a9c8d7e6f4b78, 0x192b3c4d5e6f708d, 0x08e3d7c6b5a4934c, 0x6a2f3b4c5d6e7800, 0x7c0c028f5b34ff76, 0xfc0a028e5ab4df76,
    0x25f3a9c4d6e7b041, 0xeab34c5d6f789230, 0xb041c3d5e7f91262, 0x897a6b5c4d3e2f10, 0x8b2d4f6a7c9e10e2, 0x0b78c9d6e5f43259, 0x986f3c2b1a4d5e42, 0x99aabbccddeeff18,
    0x4c7e2d3a5b6f8c6b, 0x6b2a4c5d7e8f901c, 0x5cc8d7e6f5a4b249, 0x98f1e2d3c4b5a060, 0x31f2a3b4c5d6e678, 0x50c4d3e2f1a0b12b, 0xa95f4c3b2d1e0f43, 0x2d5c7e8f901a3b18,
    0xdcefd9b54bfcc09f, 0xf95ffa765afd602b, 0x0e09d8c7b6a543a9, 0x7fc1b2a3d4e5f241, 0xfa3c2b1a0d9e8f83, 0x17b3c5d7e9f10280, 0x43ff9a5cf4ca0c01, 0x4bffcd8e7c587601,
    0xfc0ff2865334f576, 0xfc0bf6ce5924f576, 0xba3d4c5e6f789156, 0x34f1e2d3c4b5a187, 0xe23c4b5a69788700, 0xc78a9b0d1e2f3d23, 0xc3ffb7dc36ca8c89, 0xc3ff8a54f4ca2c89,
    0xfffffcfcfd79edff, 0xfc0863fccb147576, 0x513a2b4c6d7e8f09, 0x9d8c7b6a5f4e3d14, 0xca3b4d5e6f708f04, 0x7d8e9f0a1b2c3d28, 0xfc087e8e4bb2f736, 0x43ff9e4ef4ca2c89,
];

pub const SHIFT_BISHOP: [u8; Square::SquareNb as usize] = [
    5, 4, 5, 5, 5, 5, 4, 5,
    4, 4, 5, 5, 5, 5, 4, 4,
    4, 4, 7, 7, 7, 7, 4, 4,
    5, 5, 7,10,10, 7, 5, 5,
    5, 5, 7,10,10, 7, 5, 5,
    4, 4, 7, 7, 7, 7, 4, 4,
    4, 4, 5, 5, 5, 5, 4, 4,
    5, 4, 5, 5, 5, 5, 4, 5,
];

pub const MAGIC_ROOK: [u64; Square::SquareNb as usize] = [
    0x2bd3a2f7cb3a63cb, 0x1158a84b9f7584ff, 0x0e2b87cb7b5181e6, 0xa42723b6f1d88eae, 0xf3c85d4d1a8431cc, 0x978d21f3b0b680ed, 0xb038ba76e68d5fa2, 0xce3f9c6e5ee0fa04,
    0x00326c8fa8aa207e, 0x00b58c5d994b5dc5, 0x5dc7e130882bc260, 0xce84b913e1a8882a, 0x066dc3d4e06a4ef7, 0xcb88f30a72e8ab8c, 0xcc9f379e24a1e44f, 0x0de5f8d4c02e0e7f,
    0xdf52e504a7e4d4db, 0xddfe32b47c3ca3f5, 0xbd180ccfc91dfe6d, 0xc08e40d73c8f5b15, 0x0c05ff2f2e2f2e2d, 0x5b8dc3f22337f894, 0x01b5e4c2d89a0cfc, 0xe6ec92076a3ad219,
    0x18aee8fc7b1b9e5c, 0x0e89cb2d21f7f3a3, 0xb7c752eaf0b02b0e, 0x0c15b3bfaefce813, 0x0c0b9b3d3e7d4f00, 0x05adea3b7f13f93f, 0x05cfb5a2d1f8dff4, 0x020f080a340f3e8e,
    0x01d3a2a943b10000, 0x016b497e8a0b4fbb, 0x0e6cfabc3d5917db, 0x0916c91c8d3f9f73, 0x05b0eaaf2d33a776, 0x0d7e2f3427e2ad51, 0x0b73a2f5b1cf48b5, 0x04932f290cd4a50f,
    0x06e5c0305435f5a4, 0x0024a22b8f9a4820, 0x01170fc1c50038d1, 0x0e2f2f33ae42af2d, 0x0b42d3a1b8c1f230, 0x09183a6d4bfc239a, 0x08552e1aa6f3a2f8, 0x0880ae27eb3d83d4,
    0x48FFFE99FECFAA00, 0x48FFFE99FECFAA00, 0x497FFFADFF9C2E00, 0x613FFFDDFFCE9200, 0xffffffe9ffe7ce00, 0xfffffff5fff3e600, 0x0003ff95e5e6a4c0, 0x510FFFF5F63C96A0,
    0xEBFFFFB9FF9FC526, 0x61FFFEDDFEEDAEAE, 0x53BFFFEDFFDEB1A2, 0x127FFFB9FFDFB5F6, 0x411FFFDDFFDBF4D6, 0x9b9c2d85c75aef86, 0x0003ffef27eebe74, 0x7645FFFECBFEA79E,
];

pub const SHIFT_ROOK: [u8; Square::SquareNb as usize] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    10,  9,  9,  9,  9,  9,  9, 10,
    11, 10, 10, 10, 10, 11, 10, 11,
];

