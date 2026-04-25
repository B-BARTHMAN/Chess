use crate::eval::score::{Score, DRAW_SCORE, INF, MATE_SCORE};
use crate::moves::chess_move::Move;
use crate::moves::movepick::{MovePicker, SearchMode};
use crate::position::position::Position;
use crate::search::searcher::SearchWorker;
use crate::search::tt::Bound;

impl SearchWorker {
  pub fn negamax(&mut self, pos: &mut Position, depth: i32, mut alpha: Score, beta: Score, ply: i32) -> Score {
    // Abort check. Returned value is irrelevant — root discards it.
    if self.should_stop() { return 0; }

    let key = pos.state().zobrist;
    let alpha_orig = alpha;

    // TT Probe
    let mut tt_move: Option<Move> = None;
    if let Some(e) = self.tt.probe(key) {
      if e.mv.0 != 0 { tt_move = Some(e.mv); }
      if e.depth >= depth as u8 {
        let s = e.score;
        match e.bound {
          Bound::Exact => return s,
          Bound::Lower => if s >= beta { return s },
          Bound::Upper => if s <= alpha { return s },
          _ => {}
        }
      }
    }

    // Terminal: Enter qsearch
    if depth == 0 {
      return self.qsearch(pos, alpha, beta, ply);
    }

    // Move loop: Generate Moves
    let picker: MovePicker = MovePicker::new(pos, SearchMode::Search, tt_move);
    let mut best = -INF;
    let best_move = Move(0);

    for mv in picker {

      pos.do_move(mv);

      // Check if move was legal
      if !pos.is_legal() { pos.undo_move(mv); continue; }

      let score = -self.negamax(pos, depth - 1, -beta, -alpha, ply + 1);
      pos.undo_move(mv);

      if score > best { best = score; }
      if score > alpha { alpha = score; }
      if alpha >= beta {
        if !self.should_stop() {
          self.tt.store(key, mv, score, depth as u8, Bound::Lower);
        }
        return beta;
      }
    }

    // No Moves: Checkmate or Stalemate
    if best == -INF { return if pos.in_check() { -MATE_SCORE + ply } else { DRAW_SCORE }; }

    if !self.should_stop() {
      let bound = if best > alpha_orig { Bound::Exact } else { Bound::Upper };
      self.tt.store(key, best_move, best, depth as u8, bound);
    }

    best

  }
}