#[repr(i8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Rank {
    None = -1,
    R1 = 0,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}
impl Rank {
    #[inline]
    pub const fn from_index(idx: i8) -> Self {
        match idx {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => Rank::None,
        }
    }
}
