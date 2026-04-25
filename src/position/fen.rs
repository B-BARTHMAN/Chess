use crate::bitboard::bitboard::Bitboard;
use crate::board::file::file_of;
use crate::board::square::{SQUARE_COUNT, Square};
use crate::piece::color::{COLOR_COUNT, Color};
use crate::piece::piece::Piece;
use crate::piece::piece::Piece::NoPiece;
use crate::piece::piece_type::{PIECE_TYPE_COUNT, PieceType};
use crate::position::castling::CastlingRights;
use crate::position::position::Position;
use crate::position::state::{State, StateStack};
use crate::position::zobrist::{CASTLING_KEYS, EP_FILE_KEYS, PIECE_SQUARE_KEYS, SIDE_TO_MOVE_KEY};
use crate::util::by::By;

impl Position {
    pub fn from_fen(fen: &str) -> Self {
        let mut by_type: By<PieceType, Bitboard, PIECE_TYPE_COUNT> = By::new(0);
        let mut by_color: By<Color, Bitboard, COLOR_COUNT> = By::new(0);
        let mut by_square: By<Square, Piece, SQUARE_COUNT> = By::new(Piece::NoPiece);

        let fen_parts = fen.split_whitespace().collect::<Vec<&str>>();

        let mut sq: i8 = Square::A8 as i8;
        for c in fen_parts[0].chars() {
            match c {
                '/' => {
                    sq -= 16;
                }
                '1'..='8' => {
                    sq += c.to_digit(10).unwrap() as i8;
                }
                _ => {
                    let bb = 1u64 << sq;

                    let piece = Piece::from_fen(c);

                    by_type[piece.piece_type()] |= bb;
                    by_color[piece.color()] |= bb;
                    by_square[Square::from_index(sq)] = piece;
                    sq += 1;
                }
            }
        }

        let side_to_move = Color::from_fen(fen_parts[1].chars().last().unwrap());
        let castling_rights = CastlingRights::from_fen(fen_parts[2]);
        let ep_square = Square::ep_square(fen_parts[3]);

        // Compute Zobrist Hash
        let mut zobrist: u64 = 0;
        for sq_idx in 0..(SQUARE_COUNT as i8) {
            let sq = Square::from_index(sq_idx);
            let piece = by_square[sq];
            if piece != Piece::NoPiece {
                zobrist ^= PIECE_SQUARE_KEYS[piece as usize][sq as usize];
            }
        }
        // Castling Rights Zobrist
        zobrist ^= CASTLING_KEYS[castling_rights.bits() as usize];
        if side_to_move == Color::Black {
            zobrist ^= SIDE_TO_MOVE_KEY;
        }
        // EP Square Zobrist
        if ep_square != Square::None {
            zobrist ^= EP_FILE_KEYS[file_of(ep_square) as usize];
        }

        Position {
            by_type,
            by_color,
            by_square,
            states: StateStack(
                vec![State { 
                    ep_square,
                    castling_rights,
                    captured: None,
                    zobrist,
                }]
            ),
            side_to_move,
        }
    }
}
