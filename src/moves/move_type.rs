#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
  Normal = 0u16 << 12,
  Promotion = 1u16 << 12,
  EnPassant = 2u16 << 12,
  Castle = 3u16 << 12,
}