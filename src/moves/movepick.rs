use crate::movegen::movegen::generate;
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::position::position::Position;

struct MovePicker {
  movelist: MoveList,
  index: usize,
}

impl MovePicker {
  pub fn new(pos: &Position) -> Self {
    let mut movelist = MoveList::new();
    generate(pos, &mut movelist);
    movelist.shuffle(&mut rand::rng());
    MovePicker { movelist, index: 0 }
  }

  pub fn next(&mut self) -> Option<Move> {
    let m = self.movelist[self.index];
    self.index += 1;
    Some(m)
  }
}