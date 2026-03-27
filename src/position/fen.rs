use crate::bitboard::bitboard::Bitboard;
use crate::board::square::Square;
use crate::piece::color::{Color, COLOR_COUNT};
use crate::piece::piece_type::{PieceType, PIECE_TYPE_COUNT};
use crate::position::castling::CastlingRights;
use crate::position::position::Position;
use crate::position::state::State;

impl Position {
  pub fn from_fen(fen: &str) -> Self {
    let mut by_type: [Bitboard; PIECE_TYPE_COUNT] = [0u64; PIECE_TYPE_COUNT];
    let mut by_color: [Bitboard; COLOR_COUNT] = [0u64; COLOR_COUNT];

    let fen_parts = fen.split_whitespace().collect::<Vec<&str>>();

    let mut sq: i8 = Square::A8 as i8;
    for c in fen_parts[0].chars() {
      match c {
        '/' => { sq -= 16; }
        '1'..='8' => { sq += c.to_digit(10).unwrap() as i8; }
        _ => {
          let bb = 1u64 << sq;

          by_type[PieceType::from_fen(c) as usize] |= bb;
          by_color[Color::from_fen(c) as usize] |= bb;
          sq += 1;

        }
      }
    }

    let side_to_move = Color::from_fen(fen_parts[1].chars().last().unwrap());
    let castling_rights = CastlingRights::from_fen(fen_parts[2]);
    let ep_square = Square::ep_square(fen_parts[3]);

    Position {
      by_type,
      by_color,
      states: vec![State {
        ep_square,
        castling_rights,
        captured: None,
      }],
      side_to_move
    }
  }
}