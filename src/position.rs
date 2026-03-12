use crate::types::{Color, Piece, PieceType, Square, Bitboard};

pub struct PositionState {
    ep_square: Square,
}
pub struct Position {
    board: [Piece; Square::SquareNb as usize],
    by_type: [Bitboard; PieceType::PieceTypeNb as usize],
    by_color: [Bitboard; Color::ColorNb as usize],

    state: Vec<PositionState>,
}

impl Position {
    #[inline(always)]
    pub fn pieces<const PT: u8, const US: u8>(&self) -> Bitboard {
        self.by_type[PT as usize] & self.by_color[US as usize]
    }

    #[inline(always)]
    pub fn all_pieces(&self) -> Bitboard {
        self.by_color[Color::White as usize] | self.by_color[Color::Black as usize]
    }

    #[inline(always)]
    pub fn by_color<const C: u8>(&self) -> Bitboard {
        self.by_color[C as usize]
    }

    #[inline(always)]
    pub fn enemies<const C: u8>(&self) -> Bitboard {
        self.by_color[1 - (C as usize)]
    }

    #[inline(always)]
    pub fn ep_square(&self) -> Square {
        self.state.last().unwrap().ep_square
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board: [Piece; Square::SquareNb as usize] = [Piece::NoPiece; Square::SquareNb as usize];
        let mut by_type: [Bitboard; PieceType::PieceTypeNb as usize] = [0u64; PieceType::PieceTypeNb as usize];
        let mut by_color: [Bitboard; Color::ColorNb as usize] = [0u64; Color::ColorNb as usize];

        let fen_parts = fen.split_whitespace().collect::<Vec<&str>>();

        let mut sq: i8 = Square::A8 as i8;
        for c in fen_parts[0].chars() {
            match c {
                '/' => { sq -= 16; }
                '1'..='8' => { sq += c.to_digit(10).unwrap() as i8; }
                _ => {
                    let piece = Piece::from_fen(c);
                    let bb = 1u64 << sq;

                    board[sq as usize] = piece;
                    by_type[piece.piece_type() as usize] |= bb;
                    by_color[piece.color() as usize] |= bb;
                    sq += 1;

                }
            }
        }

        let ep_square = Square::from_fen(fen_parts[3]);

        Position {
            board,
            by_type,
            by_color,
            state: vec![PositionState {
                ep_square
            }]
        }
    }
}