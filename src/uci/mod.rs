pub mod engine;
pub mod parse;

use std::io::{self, BufRead};

pub use engine::UciEngine;

pub fn run() -> io::Result<()> {
  let stdin = io::stdin();
  let mut engine = UciEngine::new();
  for line in stdin.lock().lines() {
    let line = line?;
    let line = line.trim();
    if line.is_empty() { continue; }
    if engine.handle(line) { break; }
  }
  Ok(())
}