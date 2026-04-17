use crate::eval::evaluate;
use crate::eval::score::{Score, INF};
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::Searcher;

impl Searcher{
  pub fn qsearch(&self, pos: &mut Position, mut alpha: Score, beta: Score, ply: i32) -> Score {
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