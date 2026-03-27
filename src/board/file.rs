#[repr(i8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum File {
  None = -1,
  A = 0, B, C, D, E, F, G, H
}

impl File {
  #[inline]
  pub const fn from_index(idx: i8) -> Self {
    match idx {
      0 => File::A, 1 => File::B, 2 => File::C, 3 => File::D, 4 => File::E, 5 => File::F, 6 => File::G, 7 => File::H,
      _ => File::None
    }
  }
}