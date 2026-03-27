pub const COLOR_COUNT: usize = 2;
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
  White = 0,
  Black = 1,
}

impl Color {
  #[inline]
  pub const fn other(self) -> Self {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }

  pub fn from_fen(fen: char) -> Self {
    debug_assert!("pPnNbBrRqQkKw".contains(fen));
    match fen {
      'p' | 'n' | 'b' | 'r' | 'q' | 'k'       => Color::Black,
      'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'w' => Color::White,
      _ => panic!("Invalid fen string encountered: '{}'", fen),
    }
  }
}