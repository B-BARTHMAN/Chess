use crate::movegen::movegen::{gen_captures, gen_evasions, gen_quiet, generate};
use crate::moves::chess_move::Move;
use crate::moves::movelist::MoveList;
use crate::position::position::Position;

pub enum SearchMode {
  Search,
  QSearch,
}

enum Stage {
  TTMove,
  Captures,
  Quiets,
  Done
}

pub struct MovePicker {
  movelist: MoveList,
  captures_end: usize,
  stage: Stage,
  index: usize,
  tt_move: Option<Move>,
}

impl MovePicker {
  pub fn new(pos: &Position, mode: SearchMode, tt_move: Option<Move>) -> Self {
    let mut movelist = MoveList::new();

    if pos.in_check() {
      gen_evasions(pos, &mut movelist);
      //movelist.shuffle(&mut rand::rng());
      return MovePicker {
        captures_end: movelist.len(),
        movelist,
        stage: Stage::TTMove,
        index: 0,
        tt_move,
      }
    }

    gen_captures(pos, &mut movelist);
    let captures_end = movelist.len();

    if let SearchMode::Search = mode {
      gen_quiet(pos, &mut movelist);
    }
    MovePicker {movelist, captures_end, stage: Stage::TTMove, index: 0, tt_move}

  }
}

impl Iterator for MovePicker {
  type Item = Move;

  fn next(&mut self) -> Option<Move> {
    loop {
      match self.stage {

        Stage::TTMove => {
          self.stage = Stage::Captures;
          if let Some(mv) = self.tt_move { return Some(mv) }
        }

        Stage::Captures => {
          if self.index < self.captures_end {
            let m = self.movelist[self.index];
            self.index += 1;
            if let Some(tt) = self.tt_move { if m.0 == tt.0 {continue; }}
            return Some(m);
          }
          self.stage = Stage::Quiets;
        }

        Stage::Quiets => {
          if self.index < self.movelist.len() {
            let m = self.movelist[self.index];
            self.index += 1;
            if let Some(tt) = self.tt_move { if m.0 == tt.0 {continue; }}
            return Some(m);
          }
          self.stage = Stage::Done;
        }

        Stage::Done => return None
      }
    }
  }
}