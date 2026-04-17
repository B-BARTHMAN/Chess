use crate::movegen::movegen::{gen_captures, gen_evasions, gen_quiet, generate};
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::position::position::Position;

pub enum SearchMode {
  Search,
  QSearch,
}

enum Stage {
  Captures,
  Quiets,
  Done
}

pub struct MovePicker {
  movelist: MoveList,
  captures_end: usize,
  stage: Stage,
  index: usize,
}

impl MovePicker {
  pub fn new(pos: &Position, mode: SearchMode) -> Self {
    let mut movelist = MoveList::new();

    if pos.in_check() {
      gen_evasions(pos, &mut movelist);
      movelist.shuffle(&mut rand::rng());
      return MovePicker {
        captures_end: movelist.len(),
        movelist,
        stage: Stage::Captures,
        index: 0,
      }
    }

    gen_captures(pos, &mut movelist);
    movelist.shuffle_range(0, movelist.len(), &mut rand::rng());
    let captures_end = movelist.len();

    if let SearchMode::Search = mode {
      gen_quiet(pos, &mut movelist);
      movelist.shuffle_range(captures_end, movelist.len(), &mut rand::rng());
    }

    MovePicker {movelist, captures_end, stage: Stage::Captures, index: 0}

  }
}

impl Iterator for MovePicker {
  type Item = Move;

  fn next(&mut self) -> Option<Move> {
    loop {
      match self.stage {

        Stage::Captures => {
          if self.index < self.captures_end {
            let m = self.movelist[self.index];
            self.index += 1;
            return Some(m);
          }
          self.stage = Stage::Quiets;
        }

        Stage::Quiets => {
          if self.index < self.movelist.len() {
            let m = self.movelist[self.index];
            self.index += 1;
            return Some(m);
          }
          self.stage = Stage::Done;
        }

        Stage::Done => return None
      }
    }
  }
}