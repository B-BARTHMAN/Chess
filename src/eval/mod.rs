pub mod material;

use crate::position::position::Position;

pub trait Evaluator {
  fn eval(&self, position: &Position) -> i32;
}