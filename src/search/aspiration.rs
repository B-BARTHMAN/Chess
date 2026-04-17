use crate::eval::score::{Score, INF};
use crate::moves::chess_move::Move;
use crate::position::position::Position;
use crate::search::searcher::SearchWorker;

// --- Aspiration tuning ---------------------------------------------------
// Your eval is currently in whole pawns (pawn=1, queen=9), so these are in
// pawns too. With a centipawn eval you'd bump the initial delta to ~25 and
// the cap to ~1000.
pub const ASPIRATION_MIN_DEPTH: i32 = 3;
const ASPIRATION_INITIAL_DELTA: Score = 1;
const ASPIRATION_MAX_DELTA: Score = 1000;

impl SearchWorker {
  pub fn aspiration_search(&self, pos: &mut Position, depth: i32, prev_score: Score) -> (Move, Score) {
    let mut delta = ASPIRATION_INITIAL_DELTA;
    let mut alpha = (prev_score - delta).max(-INF);
    let mut beta  = (prev_score + delta).min(INF);

    loop {
      let (mv, score) = self.search_root(pos, depth, alpha, beta);

      if self.should_stop() { return (mv, score); }

      if score <= alpha {
        // Fail-low: we only know the true score is <= alpha. Widen downward.
        alpha = (score - delta).max(-INF);
      } else if score >= beta {
        // Fail-high: true score is >= beta. Widen upward.
        beta = (score + delta).min(INF);
      } else {
        // Score inside window — trustworthy result.
        return (mv, score);
      }

      delta = delta.saturating_mul(2);
      if delta >= ASPIRATION_MAX_DELTA {
        // Give up and force a definitive full-window search.
        alpha = -INF;
        beta  = INF;
      }

    }
  }
}