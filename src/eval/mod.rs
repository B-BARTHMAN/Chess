pub mod material;
pub mod score;

use crate::eval::material::MaterialEvaluator;
use crate::eval::score::Score;
use crate::position::position::Position;

pub trait Evaluator {
  fn eval(&self, position: &Position) -> Score;
}

pub fn evaluate(position: &Position) -> Score {
  let material : MaterialEvaluator = MaterialEvaluator {};
  material.eval(position)
}