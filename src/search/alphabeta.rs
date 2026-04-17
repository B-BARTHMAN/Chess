use crate::eval::score::{Score, DRAW_SCORE, INF, MATE_SCORE};
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::Searcher;

impl Searcher {
  pub fn negamax(&self, pos: &mut Position, depth: i32, mut alpha: Score, beta: Score, ply: i32) -> Score {
    // Terminal: Enter qsearch
    if depth == 0 {
      return self.qsearch(pos, alpha, beta, ply);
    }

    // Move loop: Generate Moves
    let picker: MovePicker = MovePicker::new(pos, SearchMode::Search);

    let mut best = -INF;

    for mv in picker {

      pos.do_move(mv);

      // Check if move was legal
      if !pos.is_legal() { pos.undo_move(mv); continue; }

      let score = -self.negamax(pos, depth - 1, -beta, -alpha, ply + 1);
      pos.undo_move(mv);

      if score > best { best = score; }
      if score > alpha { alpha = score; }
      if alpha >= beta { return beta; }
    }

    // No Moves: Checkmate or Stalemate
    if best == -INF { return if pos.in_check() { -MATE_SCORE + ply } else { DRAW_SCORE }; }

    best

  }
}