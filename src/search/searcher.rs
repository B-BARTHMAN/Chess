use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use crate::eval::score::Score;
use crate::moves::chess_move::Move;
use crate::position::position::Position;

pub struct Searcher {
  stop: Arc<AtomicBool>,
  handle: Option<JoinHandle<(Move, Score)>>,
}

pub struct SearchWorker {
  stop: Arc<AtomicBool>,
}

impl Searcher {
  pub fn new() -> Self {
    Self { stop: Arc::new(AtomicBool::new(false)), handle: None }
  }
  pub fn start_search(&mut self, mut pos: Position, depth: i32) {
    assert!(self.handle.is_none(), "a search is already running");
    self.stop.store(false, Ordering::Relaxed);
    let stop = Arc::clone(&self.stop);
    self.handle = Some(thread::spawn(move || {
      let worker = SearchWorker { stop };
      worker.search(&mut pos, depth)
    }))
  }
  pub fn stop_flag(&self) -> Arc<AtomicBool> { Arc::clone(&self.stop) }
  pub fn wait(&mut self) -> Option<(Move, Score)> {
    self.handle.take().map(|h| h.join().expect("search thread panicked"))
  }
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