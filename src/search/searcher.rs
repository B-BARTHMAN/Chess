use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use crate::eval::score::Score;
use crate::moves::chess_move::Move;
use crate::position::position::Position;
use crate::search::tt::TranspositionTable;

pub struct Searcher {
  stop: Arc<AtomicBool>,
  handle: Option<JoinHandle<(Move, Score, TranspositionTable)>>,
  tt: Option<TranspositionTable>,
}

pub struct SearchWorker {
  stop: Arc<AtomicBool>,
  pub tt: TranspositionTable,
}

impl Searcher {
  pub fn new() -> Self {

    Self {
      stop: Arc::new(AtomicBool::new(false)),
      handle: None,
      tt: Some(TranspositionTable::new(16))
    }
  }
  pub fn start_search(&mut self, mut pos: Position, depth: i32) {
    assert!(self.handle.is_none(), "a search is already running");
    self.stop.store(false, Ordering::Relaxed);
    let stop = Arc::clone(&self.stop);
    let tt = self.tt.take().expect("tt missing — was wait() called after the previous search?");
    self.handle = Some(thread::spawn(move || {
      let mut worker = SearchWorker { stop, tt };
      let (mv, score) = worker.search(&mut pos, depth);
      (mv, score, worker.tt)
    }));
  }

  pub fn wait(&mut self) -> Option<(Move, Score)> {
    self.handle.take().map(|h| {
      let (mv, score, tt) = h.join().expect("search thread panicked");
      self.tt = Some(tt);
      (mv, score)
    })
  }
  pub fn stop_flag(&self) -> Arc<AtomicBool> { Arc::clone(&self.stop) }
  pub fn is_searching(&self) -> bool { self.handle.is_some() }
}

impl Default for Searcher {
  fn default() -> Self { Self::new() }
}

impl SearchWorker {
  #[inline]
  pub fn should_stop(&self) -> bool {
    self.stop.load(Ordering::Relaxed)
  }
}