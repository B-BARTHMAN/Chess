// src/position/zobrist.rs
use crate::board::square::SQUARE_COUNT;
use crate::piece::piece::PIECE_COUNT;

const fn rotl(x: u64, k: u32) -> u64 {
  (x << k) | (x >> (64 - k))
}

const fn splitmix64(state: u64) -> (u64, u64) {
  let next = state.wrapping_add(0x9E3779B97F4A7C15);
  let mut z = next;
  z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
  z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
  (next, z ^ (z >> 31))
}

const fn xoshiro256ss(s: [u64; 4]) -> ([u64; 4], u64) {
  let result = rotl(s[1].wrapping_mul(5), 7).wrapping_mul(9);
  let t = s[1] << 17;
  let mut s = s;
  s[2] ^= s[0];
  s[3] ^= s[1];
  s[1] ^= s[2];
  s[0] ^= s[3];
  s[2] ^= t;
  s[3] = rotl(s[3], 45);
  (s, result)
}

struct Keys {
  piece_square: [[u64; SQUARE_COUNT]; PIECE_COUNT],
  side_to_move: u64,
  castling: [u64; 16],
  ep_file: [u64; 8]
}

const fn compute_keys() -> Keys {
  let (a, s0) = splitmix64(0xDEAD_BEEF_C0DE_CAFE);
  let (b, s1) = splitmix64(a);
  let (c, s2) = splitmix64(b);
  let (_, s3) = splitmix64(c);
  let mut state = [s0, s1, s2, s3];

  let mut piece_square = [[0u64; SQUARE_COUNT]; PIECE_COUNT];
  let mut p = 0;
  while p < PIECE_COUNT {
    let mut sq = 0;
    while sq < SQUARE_COUNT {
      let (next, key) = xoshiro256ss(state);
      state = next;
      piece_square[p][sq] = key;
      sq += 1;
    }
    p += 1;
  }

  let (next, side_to_move) = xoshiro256ss(state);
  state = next;

  let mut base = [0u64; 4];
  let mut i = 0;
  while i < 4 {
    let (next, k) = xoshiro256ss(state);
    state = next;
    base[i] = k;
    i += 1;
  }

  let mut castling = [0u64; 16];
  let mut mask = 0;
  while mask < 16 {
    let mut combined = 0u64;
    let mut bit = 0;
    while bit < 4 {
      if (mask >> bit) & 1 == 1 {
        combined ^= base[bit];
      }
      bit += 1;
    }
    castling[mask] = combined;
    mask += 1;
  }

  let mut ep_file = [0u64; 8];
  let mut f = 0;
  while f < 8 {
    let (next, k) = xoshiro256ss(state);
    state = next;
    ep_file[f] = k;
    f += 1;
  }

  Keys { piece_square, side_to_move, castling, ep_file }
}

const KEYS: Keys = compute_keys();

pub const PIECE_SQUARE_KEYS: [[u64; SQUARE_COUNT]; PIECE_COUNT] = KEYS.piece_square;
pub const SIDE_TO_MOVE_KEY: u64 = KEYS.side_to_move;
pub const CASTLING_KEYS: [u64; 16] = KEYS.castling;
pub const EP_FILE_KEYS: [u64; 8] = KEYS.ep_file;