use crate::board::square::Square;
use crate::position::castling::CastlingRights;
use crate::piece::piece::Piece;

pub struct State {
    pub ep_square: Square,
    pub castling_rights: CastlingRights,
    pub captured: Option<Piece>,
    pub zobrist: u64
}

pub struct StateStack(pub Vec<State>);

impl StateStack {
    pub fn push(&mut self, state: State) {self.0.push(state); }
    pub fn pop(&mut self) -> State {self.0.pop().expect("state stack underflow") }
    pub fn current(&self) -> &State {&self.0.last().expect("state stack empty") }
}