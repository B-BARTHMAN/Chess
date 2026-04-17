use crate::eval::score::{Score, INF};
use crate::moves::chess_move::Move;
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::Searcher;

impl Searcher {
  pub fn search(&self, pos: &mut Position, depth: i32) -> (Move, Score) {
    let mut best_move = Move(0);
    let mut best_score = -INF;
    let mut alpha = -INF;
    let beta = INF;

    for mv in MovePicker::new(pos, SearchMode::Search) {
      pos.do_move(mv);

      // Check if move was legal
      if !pos.is_legal() { pos.undo_move(mv); continue; }

      let score = -self.negamax(pos, depth - 1, -beta, -alpha, 1);
      pos.undo_move(mv);

      if score > best_score {
        best_score = score;
        best_move = mv;
      }
      if score > alpha {
        alpha = score;
      }
    }

    (best_move, best_score)
  }
}