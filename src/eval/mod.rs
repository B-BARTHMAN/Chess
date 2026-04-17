pub mod material;
pub mod score;
pub mod psqt;

use crate::eval::material::PSQTEvaluator;
use crate::eval::score::Score;
use crate::position::position::Position;

pub trait Evaluator {
  fn eval(&self, position: &Position) -> Score;
}

pub fn evaluate(position: &Position) -> Score {
  let material : PSQTEvaluator = PSQTEvaluator {};
  material.eval(position)
}