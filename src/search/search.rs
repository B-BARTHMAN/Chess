use crate::eval::score::{Score, INF};
use crate::moves::chess_move::Move;
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::aspiration::ASPIRATION_MIN_DEPTH;
use crate::search::searcher::SearchWorker;

impl SearchWorker {
  pub fn search(&self, pos: &mut Position, max_depth: i32) -> (Move, Score) {
    let mut best_move = Move(0);
    let mut best_score = -INF;

    for depth in 1..=max_depth {
      let (mv, score) = if depth < ASPIRATION_MIN_DEPTH {
        self.search_root(pos, depth, -INF, INF)
      } else {
        self.aspiration_search(pos, depth, best_score)
      };

      if self.should_stop() { break; }

      best_move = mv;
      best_score = score;
    }

    (best_move, best_score)
  }
  pub fn search_root(&self, pos: &mut Position, depth: i32, mut alpha: Score, beta: Score) -> (Move, Score) {
    let mut best_move = Move(0);
    let mut best_score = -INF;

    for mv in MovePicker::new(pos, SearchMode::Search) {
      pos.do_move(mv);
      if !pos.is_legal() { pos.undo_move(mv); continue; }

      let score = -self.negamax(pos, depth - 1, -beta, -alpha, 1);

      pos.undo_move(mv);

      if self.should_stop() { break; }

      if score > best_score { best_score = score; best_move = mv; }
      if score > alpha { alpha = score; }
      if alpha >= beta { break; }
    }

    (best_move, best_score)
  }
}