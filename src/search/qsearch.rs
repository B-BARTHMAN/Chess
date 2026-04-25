use crate::eval::evaluate;
use crate::eval::score::{Score, INF};
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::searcher::SearchWorker;

impl SearchWorker {
  pub fn qsearch(&mut self, pos: &mut Position, mut alpha: Score, beta: Score, ply: i32) -> Score {
    if self.should_stop() { return 0; }

    let stand_pat = evaluate(pos);
    if stand_pat >= beta { return beta; }
    if stand_pat > alpha { alpha = stand_pat; }

    let mut best: Score = stand_pat;

    // No TT move in qsearch for now — pass None.
    let picker = MovePicker::new(pos, SearchMode::QSearch, None);

    for mv in picker {
      pos.do_move(mv);
      if !pos.is_legal() { pos.undo_move(mv); continue; }

      let score = -self.qsearch(pos, -beta, -alpha, ply + 1);
      pos.undo_move(mv);

      if score > best { best = score; }
      if score > alpha { alpha = score; }
      if score >= beta { return beta; }
    }

    best
  }
}