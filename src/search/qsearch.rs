use crate::eval::evaluate;
use crate::eval::score::{Score, INF};
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::searcher::SearchWorker;

impl SearchWorker {
  pub fn qsearch(&self, pos: &mut Position, mut alpha: Score, beta: Score, ply: i32) -> Score {
    // Abort check. Returned value is irrelevant — root discards it.
    if self.should_stop() { return 0; }

    let mut best: Score = -INF;
    // Move loop: Generate Moves
    let picker: MovePicker = MovePicker::new(pos, SearchMode::QSearch);

    for mv in picker {
      pos.do_move(mv);

      // Check if move was legal
      if !pos.is_legal() { pos.undo_move(mv); continue; }

      let score = -self.qsearch(pos, -beta, -alpha, ply + 1);
      pos.undo_move(mv);

      if score > best { best = score; }
      if score > alpha { alpha = score; }
      if score >= beta { return beta; }
    }

    // No captures left: evaluate the position
    if best == -INF { return evaluate(pos); }

    best
  }
}