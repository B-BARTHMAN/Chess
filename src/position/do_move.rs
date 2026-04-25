use crate::bitboard::masks::square_bb;
use crate::bitboard::shift::shift_square;
use crate::board::file::file_of;
use crate::board::square::Square;
use crate::moves::chess_move::{Move};
use crate::moves::move_type::MoveType;
use crate::piece::color::Color;
use crate::piece::piece_type::{PieceType};
use crate::position::castling::CastlingRights;
use crate::position::position::Position;
use crate::position::state::State;
use crate::piece::piece::Piece;
use crate::position::zobrist::{CASTLING_KEYS, EP_FILE_KEYS, PIECE_SQUARE_KEYS, SIDE_TO_MOVE_KEY};

impl Position {
    pub fn do_move(&mut self, m: Move) {
        let from = m.from();
        let to = m.to();
        let us = self.side_to_move;
        let them = us.other();
        let moved = self.by_square[from];

        let prev = self.state();
        let mut hash = prev.zobrist;
        let old_rights = prev.castling_rights;
        let old_ep = prev.ep_square;

        // Create new state
        let mut new_state = State {
            ep_square: Square::None,
            castling_rights: old_rights,
            captured: None,
            zobrist: 0, // filled in at end
        };

        // Handle capture
        if self.by_color[them] & square_bb(to) != 0 {
            let captured = self.by_square[to];
            self.remove_piece(to, captured);
            hash ^= PIECE_SQUARE_KEYS[captured as usize][to as usize];
            new_state.captured = Some(captured);
        }

        // Handle Pawn
        if moved.piece_type() == PieceType::Pawn {
            match m.move_type() {
                MoveType::EnPassant => {
                    let dir = them.forward();
                    let cap_sq = shift_square(to, dir);
                    let captured = self.by_square[cap_sq];

                    debug_assert!(captured.piece_type() == PieceType::Pawn);

                    self.remove_piece(cap_sq, captured);
                    hash ^= PIECE_SQUARE_KEYS[captured as usize][cap_sq as usize];
                    new_state.captured = Some(captured);
                }
                _ if Square::rank_distance(from, to) == 2 => {
                    // Double push: set en-passant square on the skipped square
                    new_state.ep_square = Square::midpoint(from, to);
                }
                _ => {}
            }
        }

        // EP file diff
        if old_ep != Square::None {
            hash ^= EP_FILE_KEYS[file_of(old_ep) as usize];
        }
        if new_state.ep_square != Square::None {
            hash ^= EP_FILE_KEYS[file_of(new_state.ep_square) as usize];
        }

        // Move piece
        self.move_piece(from, to, moved);
        hash ^= PIECE_SQUARE_KEYS[moved as usize][from as usize];
        hash ^= PIECE_SQUARE_KEYS[moved as usize][to as usize];

        // Promotion
        if m.move_type() == MoveType::Promotion {
            self.remove_piece(to, moved);
            let promo = Piece::new(us, m.promotion_type().piece_type());
            self.add_piece(to, promo);
            hash ^= PIECE_SQUARE_KEYS[moved as usize][to as usize];
            hash ^= PIECE_SQUARE_KEYS[promo as usize][to as usize];
        }

        // Castling — move the rook
        if m.move_type() == MoveType::Castle {
            let rook = Piece::new(us, PieceType::Rook);
            let (rook_from, rook_to) = match to {
                Square::G1 => (Square::H1, Square::F1),
                Square::C1 => (Square::A1, Square::D1),
                Square::G8 => (Square::H8, Square::F8),
                Square::C8 => (Square::A8, Square::D8),
                _ => unreachable!(),
            };
            self.move_piece(rook_from, rook_to, rook);
            hash ^= PIECE_SQUARE_KEYS[rook as usize][rook_from as usize];
            hash ^= PIECE_SQUARE_KEYS[rook as usize][rook_to as usize];
        }

        // Update castling rights
        self.update_castling_rights(from, to, moved.piece_type(), us, &mut new_state);

        // Castling rights diff
        if new_state.castling_rights != old_rights {
            hash ^= CASTLING_KEYS[old_rights.bits() as usize];
            hash ^= CASTLING_KEYS[new_state.castling_rights.bits() as usize];
        }

        // Side to move toggles every move
        hash ^= SIDE_TO_MOVE_KEY;

        new_state.zobrist = hash;
        self.states.push(new_state);
        self.side_to_move = us.other();
    }
    pub fn undo_move(&mut self, m: Move) {
        let state = self.states.pop();
        let from = m.from();
        let to = m.to();
        let them = self.side_to_move;
        let us = them.other();

        self.side_to_move = us;

        // undo castling rook move
        if m.move_type() == MoveType::Castle {
            match to {
                Square::G1 => self.move_piece(Square::F1, Square::H1, Piece::new(us, PieceType::Rook)),
                Square::C1 => self.move_piece(Square::D1, Square::A1, Piece::new(us, PieceType::Rook)),
                Square::G8 => self.move_piece(Square::F8, Square::H8, Piece::new(us, PieceType::Rook)),
                Square::C8 => self.move_piece(Square::D8, Square::A8, Piece::new(us, PieceType::Rook)),
                _ => {}
            }
        }

        let moved = self.by_square[to];
        // move piece back
        self.move_piece(to, from, moved);

        // undo promotion
        if m.move_type() == MoveType::Promotion {
            self.remove_piece(from, moved);
            self.add_piece(from, Piece::new(us, PieceType::Pawn));
        }

        // restore captured piece
        if let Some(piece) = state.captured {
            let cap_sq = if m.move_type() == MoveType::EnPassant {
                shift_square(to, us.forward().opposite())
            } else {
                to
            };

            self.add_piece(cap_sq, piece);
        }
    }

    // ---------------------------------------------------------------------------
    // Private helpers
    // ---------------------------------------------------------------------------

    #[inline]
    fn add_piece(&mut self, square: Square, piece: Piece) {
        let bb = square_bb(square);
        let color = piece.color();
        let pt = piece.piece_type();
        debug_assert!(self.by_color[color] & bb == 0);
        debug_assert!(self.by_color[color.other()] & bb == 0);
        debug_assert!(self.by_type[PieceType::Pawn] & bb == 0);
        debug_assert!(self.by_type[PieceType::Knight] & bb == 0);
        debug_assert!(self.by_type[PieceType::Bishop] & bb == 0);
        debug_assert!(self.by_type[PieceType::Rook] & bb == 0);
        debug_assert!(self.by_type[PieceType::Queen] & bb == 0);
        debug_assert!(self.by_type[PieceType::King] & bb == 0);
        self.by_color[color] |= bb;
        self.by_type[pt] |= bb;
        self.by_square[square] = piece;
    }
    #[inline]
    fn remove_piece(&mut self, square: Square, piece: Piece) {
        let bb = square_bb(square);
        let color = piece.color();
        let pt = piece.piece_type();
        debug_assert!(self.by_color[color] & bb != 0);
        debug_assert!(self.by_type[pt] & bb != 0);
        self.by_color[color] &= !bb;
        self.by_type[pt] &= !bb;
        self.by_square[square] = Piece::NoPiece;
    }
    #[inline]
    fn move_piece(&mut self, from: Square, to: Square, piece: Piece) {
        let from_bb = square_bb(from);
        let to_bb = square_bb(to);
        let color = piece.color();
        let pt = piece.piece_type();
        debug_assert!(self.by_color[color] & to_bb == 0);
        debug_assert!(self.by_color[color.other()] & to_bb == 0);
        debug_assert!(self.by_color[color] & from_bb != 0);
        debug_assert!(self.by_color[color.other()] & from_bb == 0);
        debug_assert!(self.by_type[PieceType::Pawn] & to_bb == 0);
        debug_assert!(self.by_type[PieceType::Knight] & to_bb == 0);
        debug_assert!(self.by_type[PieceType::Bishop] & to_bb == 0);
        debug_assert!(self.by_type[PieceType::Rook] & to_bb == 0);
        debug_assert!(self.by_type[PieceType::Queen] & to_bb == 0);
        debug_assert!(self.by_type[PieceType::King] & to_bb == 0);
        debug_assert!(self.by_type[pt] & from_bb != 0);
        self.by_color[color] &= !from_bb;
        self.by_type[pt] &= !from_bb;
        self.by_square[from] = Piece::NoPiece;
        self.by_color[color] |= to_bb;
        self.by_type[pt] |= to_bb;
        self.by_square[to] = piece;
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
