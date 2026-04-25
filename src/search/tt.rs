use crate::eval::score::Score;
use crate::moves::chess_move::Move;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Bound {
  None,
  Exact,
  Lower,
  Upper,
}

#[derive(Copy, Clone)]
pub struct TTEntry {
  pub key: u64,
  pub mv: Move,
  pub score: Score,
  pub depth: u8,
  pub bound: Bound,
}

impl TTEntry {
  pub const EMPTY: Self = Self {
    key: 0, mv: Move(0), score: 0, depth: 0, bound: Bound::None,
  };
}

pub struct TranspositionTable {
  buckets: Vec<(TTEntry, TTEntry)>,
  mask: usize
}

impl TranspositionTable {
  pub fn new(size_mb: usize) -> Self {
    let bucket_bytes = std::mem::size_of::<(TTEntry, TTEntry)>();
    let raw = (size_mb * 1024 * 1024) / bucket_bytes;
    let count = if raw < 2 { 1 } else { 1usize << (usize::BITS - 1 - raw.leading_zeros()) };
    Self { buckets: vec![(TTEntry::EMPTY, TTEntry::EMPTY); count], mask: count - 1 }
  }

  pub fn clear(&mut self) {
    for b in self.buckets.iter_mut() { *b = (TTEntry::EMPTY, TTEntry::EMPTY); }
  }

  #[inline]
  fn index(&self, key: u64) -> usize { (key as usize) & self.mask }

  pub fn probe(&self, key: u64) -> Option<TTEntry> {
    let b = &self.buckets[self.index(key)];
    if b.0.key == key && b.0.bound != Bound::None {
      return Some(b.0);
    }
    if b.1.key == key && b.1.bound != Bound::None {
      return Some(b.1);
    }
    None
  }

  pub fn store(&mut self, key: u64, mv: Move, score: Score, depth: u8, bound: Bound) {
    let idx = self.index(key);
    let b = &mut self.buckets[idx];
    let new = TTEntry { key, mv, score, depth, bound };

    let dp = &b.0;
    if dp.bound == Bound::None || dp.key == key || depth >= dp.depth {
      b.0 = new;
    } else {
      b.1 = new;
    }
  }
}