use crate::board::square::Square;

impl Square {
  #[inline]
  pub fn ep_square(fen: &str) -> Self {
    debug_assert!(fen.len() <= 2);
    match fen {
      "a3" => Square::A3,
      "b3" => Square::B3,
      "c3" => Square::C3,
      "d3" => Square::D3,
      "e3" => Square::E3,
      "f3" => Square::F3,
      "g3" => Square::G3,
      "h3" => Square::H3,
      "a6" => Square::A6,
      "b6" => Square::B6,
      "c6" => Square::C6,
      "d6" => Square::D6,
      "e6" => Square::E6,
      "f6" => Square::F6,
      "g6" => Square::G6,
      "h6" => Square::H6,
      "-" => Square::None,
      _ => panic!(),
    }
  }
}