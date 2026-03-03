use crate::types::Move;

const MAX_MOVES: usize = 128;
pub struct MoveList {
    moves: [Move; MAX_MOVES],
    count: usize
}
impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: [Move(0); MAX_MOVES],
            count: 0
        }
    }

    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        debug_assert!(self.count < MAX_MOVES);
        self.moves[self.count] = m;
        self.count += 1;
    }
    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.count]
    }
}