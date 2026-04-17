use std::io::{stdout, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::moves::chess_move::Move;
use crate::piece::color::Color;
use crate::position::position::Position;
use crate::search::searcher::Searcher;
use crate::uci::parse::{format_uci_move, parse_uci_move};

const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct UciEngine {
  initial_fen: String,
  moves: Vec<Move>,
  stop_flag: Option<Arc<AtomicBool>>,
}

impl UciEngine {
  pub fn new() -> Self {
    Self { initial_fen: STARTPOS_FEN.into(), moves: Vec::new(), stop_flag: None }
  }

  pub fn handle(&mut self, line: &str) -> bool {
    let cmd = line.split_whitespace().next().unwrap_or("");
    match cmd {
      "uci"        => self.hello(),
      "isready"    => send("readyok"),
      "ucinewgame" => self.new_game(),
      "position"   => self.handle_position(&line[cmd.len()..]),
      "go"         => self.handle_go(&line[cmd.len()..]),
      "stop"       => self.signal_stop(),
      "quit"       => { self.signal_stop(); return true; }
      _ => {}
    }
    false
  }
  fn handle_position(&mut self, args: &str) {
    let toks: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    let fen = match toks.get(i).copied() {
      Some("startpos") => { i += 1; STARTPOS_FEN.to_string() }
      Some("fen") => {
        i += 1;
        if toks.len() < i + 6 { return; }
        let f = toks[i..i + 6].join(" ");
        i += 6;
        f
      }
      _ => return,
    };

    let move_toks: &[&str] = if toks.get(i).copied() == Some("moves") {
      &toks[i + 1..]
    } else {
      &[]
    };

    let mut pos = Position::from_fen(&fen);
    let mut moves = Vec::new();
    for tok in move_toks {
      match parse_uci_move(&mut pos, tok) {
        Some(mv) => { pos.do_move(mv); moves.push(mv); }
        None => break, // malformed / illegal — stop replaying
      }
    }

    self.initial_fen = fen;
    self.moves = moves;
  }
  fn new_game(&mut self) {
    self.initial_fen = STARTPOS_FEN.into();
    self.moves.clear();
    // Future: clear TT, history, etc.
  }
  fn build_position(&self) -> Position {
    let mut pos = Position::from_fen(&self.initial_fen);
    for &mv in &self.moves { pos.do_move(mv); }
    pos
  }
  fn handle_go(&mut self, args: &str) {
    // ---- parse options ----
    let mut infinite = false;
    let mut depth:    Option<i32> = None;
    let mut movetime: Option<u64> = None;
    let mut wtime:    Option<u64> = None;
    let mut btime:    Option<u64> = None;
    let mut winc:     Option<u64> = None;
    let mut binc:     Option<u64> = None;

    let mut it = args.split_whitespace();
    while let Some(tok) = it.next() {
      match tok {
        "infinite" => infinite = true,
        "depth"    => depth    = it.next().and_then(|s| s.parse().ok()),
        "movetime" => movetime = it.next().and_then(|s| s.parse().ok()),
        "wtime"    => wtime    = it.next().and_then(|s| s.parse().ok()),
        "btime"    => btime    = it.next().and_then(|s| s.parse().ok()),
        "winc"     => winc     = it.next().and_then(|s| s.parse().ok()),
        "binc"     => binc     = it.next().and_then(|s| s.parse().ok()),
        _ => {}
      }
    }

    let pos = self.build_position();
    let white = pos.side_to_move == Color::White;

    // ---- decide deadline ----
    let think_ms: Option<u64> = if infinite {
      None
    } else if let Some(mt) = movetime {
      Some(mt)
    } else {
      let remaining = if white { wtime } else { btime };
      let inc = if white { winc.unwrap_or(0) } else { binc.unwrap_or(0) };
      // Simple: 1/20th of remaining + increment, min 10ms
      remaining.map(|r| (r / 20).saturating_add(inc).max(10))
    };

    // ---- spawn search + timer + reporter ----
    let mut searcher = Searcher::new();
    let stop = searcher.stop_flag();
    searcher.start_search(pos, depth.unwrap_or(64));

    if let Some(ms) = think_ms {
      let stop_t = Arc::clone(&stop);
      thread::spawn(move || {
        thread::sleep(Duration::from_millis(ms));
        stop_t.store(true, Ordering::Relaxed);
      });
    }

    thread::spawn(move || {
      if let Some((mv, _score)) = searcher.wait() {
        send(&format!("bestmove {}", format_uci_move(mv)));
      }
    });

    self.stop_flag = Some(stop);
  }
  fn hello(&self) {
    send("id name MyEngine");
    send("id author me");
    send("uciok");
  }
  fn signal_stop(&mut self) {
    if let Some(flag) = self.stop_flag.take() {
      flag.store(true, Ordering::Relaxed);
      // Reporter thread handles printing bestmove.
    }
  }
}

fn send(msg: &str) {
  println!("{}", msg);
  // UCI GUIs talk to us over a pipe, so stdout is block-buffered by
  // default. Flush so the GUI sees each line immediately.
  let _ = stdout().flush();
}