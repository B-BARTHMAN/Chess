use std::ops::Index;
use rand::Rng;
use crate::moves::chess_move::Move;
const MAX_MOVES: usize = 218;

pub struct MoveList {
    moves: [Move; MAX_MOVES],
    count: usize,
}

impl MoveList {
    #[inline]
    pub fn new() -> Self {
        MoveList {
            moves: [Move(0); MAX_MOVES],
            count: 0,
        }
    }
    #[inline]
    pub fn push(&mut self, m: Move) {
        debug_assert!(self.count < MAX_MOVES);
        self.moves[self.count] = m;
        self.count += 1;
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.count].iter()
    }

    #[inline]
    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        use rand::seq::SliceRandom;
        self.moves[..self.count].shuffle(rng);
    }
}

impl Index<usize> for MoveList {
    type Output = Move;
    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[index]
    }
}
