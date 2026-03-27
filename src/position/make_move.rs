use crate::bitboard::masks::square_bb;
use crate::board::square::Square;
use crate::chess_move::chess_move::{Move, MoveType};
use crate::piece::color::Color;
use crate::piece::piece_type::{PieceType, PIECE_TYPES};
use crate::position::castling::CastlingRights;
use crate::position::position::Position;
use crate::position::state::State;

impl Position {
  pub fn do_move(&mut self, m: Move) {
    let from = m.from();
    let to = m.to();
    let us = self.side_to_move;
    let them = Color::other(us);

    let from_bb = square_bb(from);
    let to_bb = square_bb(to);
    let moved = self.moved_piece(m);

    // Create new state
    let mut new_state = State {
      ep_square: Square::None,
      castling_rights: self.state().castling_rights,
      captured: None
    };

    // Handle capture
    if self.by_color(them) & to_bb != 0 {
      for pt in PIECE_TYPES {
        if self.pieces(pt, them) & to_bb != 0 {
          self.remove_piece(to, pt, them);
          new_state.captured = Some(pt);
          break;
        }
      }
    }

    // Handle Pawn
    if moved == PieceType::Pawn {
      // En Passant
      if m.mt() == MoveType::EnPassant {
        let cap_sq = Square::from_index(
          (to as i8 + if us == Color::White { -8 } else { 8 })
        );
        self.remove_piece(cap_sq, PieceType::Pawn, them);
        new_state.captured = Some(PieceType::Pawn);
      }
      // Double Pawn Push
      else if (from as i8 - to as i8).abs() == 16 {
        new_state.ep_square = Square::from_index((from as i8 + to as i8) / 2);
      }
    }

    // Move Piece
    self.move_piece(from, to, moved, us);

    // Promotion
    if m.mt() == MoveType::Promotion {
      self.remove_piece(from, PieceType::Pawn, us);
      self.add_piece(to, m.pt().pt(), us);
    }

    // Castling
    if m.mt() == MoveType::Castle {
      match to {
        Square::G1 => self.move_piece(Square::H1, Square::F1, PieceType::Rook, us),
        Square::C1 => self.move_piece(Square::A1, Square::D1, PieceType::Rook, us),
        Square::G8 => self.move_piece(Square::H8, Square::F8, PieceType::Rook, us),
        Square::C8 => self.move_piece(Square::A8, Square::D8, PieceType::Rook, us),
        _ => unreachable!(),
      }
    }

    // Update castling rights
    self.update_castling_rights(from, to, moved, us, &mut new_state);

    // Push new state
    self.states.push(new_state);

    // Switch side
    self.side_to_move = us.other();
  }
  pub fn undo_move(&mut self, m: Move) {
    let state = self.states.pop().unwrap();

    let from = m.from();
    let to = m.to();

    let them = self.side_to_move;
    let us = Color::other(them);

    let moved = self.moved_piece(Move::normal(to, from));

    self.side_to_move = us;

    // undo castling rook move
    if m.mt() == MoveType::Castle {
      match to {
        Square::G1 => self.move_piece(Square::F1, Square::H1, PieceType::Rook, us),
        Square::C1 => self.move_piece(Square::D1, Square::A1, PieceType::Rook, us),
        Square::G8 => self.move_piece(Square::F8, Square::H8, PieceType::Rook, us),
        Square::C8 => self.move_piece(Square::D8, Square::A8, PieceType::Rook, us),
        _ => {}
      }
    }

    // move piece back
    self.move_piece(to, from, moved, us);

    // undo promotion
    if m.mt() == MoveType::Promotion {
      self.remove_piece(from, moved, us);
      self.add_piece(from, PieceType::Pawn, us);
    }

    // restore captured piece
    if let Some(pt) = state.captured {
      let cap_sq = if m.0 & (MoveType::EnPassant as u16) != 0 {
        Square::from_index(
          (to as i8 + if us == Color::White { -8 } else { 8 })
        )
      } else {
        to
      };

      self.add_piece(cap_sq, pt, them);
    }
  }
  #[inline]
  const fn add_piece(&mut self, square: Square, pt: PieceType, color: Color) {
  let bb = square_bb(square);
  debug_assert!(self.by_color[color as usize] & bb == 0);
  debug_assert!(self.by_color[color.other() as usize] & bb == 0);
  debug_assert!(self.by_type[PieceType::Pawn as usize] & bb == 0);
  debug_assert!(self.by_type[PieceType::Knight as usize] & bb == 0);
  debug_assert!(self.by_type[PieceType::Bishop as usize] & bb == 0);
  debug_assert!(self.by_type[PieceType::Rook as usize] & bb == 0);
  debug_assert!(self.by_type[PieceType::Queen as usize] & bb == 0);
  debug_assert!(self.by_type[PieceType::King as usize] & bb == 0);
  self.by_color[color as usize] |= bb;
  self.by_type[pt as usize] |= bb;
}
  #[inline]
  const fn remove_piece(&mut self, square: Square, pt: PieceType, color: Color) {
    let bb = square_bb(square);
    debug_assert!(self.by_color[color as usize] & bb != 0);
    debug_assert!(self.by_type[pt as usize] & bb != 0);
    self.by_color[color as usize] &= !bb;
    self.by_type[pt as usize] &= !bb;
  }
  #[inline]
  const fn move_piece(&mut self, from: Square, to: Square, pt: PieceType, color: Color) {
    let from_bb = square_bb(from);
    let to_bb = square_bb(to);
    debug_assert!(self.by_color[color as usize] & to_bb == 0);
    debug_assert!(self.by_color[color.other() as usize] & to_bb == 0);
    debug_assert!(self.by_color[color as usize] & from_bb != 0);
    debug_assert!(self.by_color[color.other() as usize] & from_bb == 0);
    debug_assert!(self.by_type[PieceType::Pawn as usize] & to_bb == 0);
    debug_assert!(self.by_type[PieceType::Knight as usize] & to_bb == 0);
    debug_assert!(self.by_type[PieceType::Bishop as usize] & to_bb == 0);
    debug_assert!(self.by_type[PieceType::Rook as usize] & to_bb == 0);
    debug_assert!(self.by_type[PieceType::Queen as usize] & to_bb == 0);
    debug_assert!(self.by_type[PieceType::King as usize] & to_bb == 0);
    debug_assert!(self.by_type[pt as usize] & from_bb != 0);
    self.by_color[color as usize] &= !from_bb;
    self.by_type[pt as usize] &= !from_bb;
    self.by_color[color as usize] |= to_bb;
    self.by_type[pt as usize] |= to_bb;
  }
  #[inline]
  fn update_castling_rights(&mut self, from: Square, to: Square, pt: PieceType, us: Color, state: &mut State) {
    let mut rights = state.castling_rights;

    // --- King moved ---
    if pt == PieceType::King {
      match us {
        Color::White => {
          rights &= !(CastlingRights::WhiteKingside | CastlingRights::WhiteQueenside);
        }
        Color::Black => {
          rights &= !(CastlingRights::BlackKingside | CastlingRights::BlackQueenside);
        }
      }
    }

    // --- Rook moved ---
    if pt == PieceType::Rook {
      match from {
        Square::A1 => rights &= !CastlingRights::WhiteQueenside,
        Square::H1 => rights &= !CastlingRights::WhiteKingside,
        Square::A8 => rights &= !CastlingRights::BlackQueenside,
        Square::H8 => rights &= !CastlingRights::BlackKingside,
        _ => {}
      }
    }

    // --- Rook captured ---
    match to {
      Square::A1 => rights &= !CastlingRights::WhiteQueenside,
      Square::H1 => rights &= !CastlingRights::WhiteKingside,
      Square::A8 => rights &= !CastlingRights::BlackQueenside,
      Square::H8 => rights &= !CastlingRights::BlackKingside,
      _ => {}
    }

    state.castling_rights = rights;
  }
}