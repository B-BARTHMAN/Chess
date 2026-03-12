use crate::bitboard::pop_lsb;
use crate::movegen::MoveGenType;
use crate::types::{Bitboard, Move, PromotionType, Square};

const MAX_MOVES: usize = 128;
pub struct MoveList {
    moves: [Move; MAX_MOVES],
    count: usize
}
impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: [Move(0); MAX_MOVES],
            count: 0
        }
    }

    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        debug_assert!(self.count < MAX_MOVES);
        self.moves[self.count] = m;
        self.count += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.count].iter()
    }
    
    pub fn as_slice(&self) -> &[Move] {
        &self.moves[..self.count]
    }

    pub fn fill_normal(&mut self, from: Square, mut to_bb: Bitboard) {
        while to_bb != 0 {
            let to = pop_lsb(&mut to_bb);
            self.push(Move::normal(from, to));
        }
    }

    pub fn fill_pawns(&mut self, mut to_bb: Bitboard, step: i8) {
        while to_bb != 0 {
            let to = pop_lsb(&mut to_bb);
            let from = Square::from_index((to as i8) - step);
            self.push(Move::normal(from, to));
        }
    }

    pub fn fill_promotions<const T: u8>(&mut self, mut to_bb: Bitboard, step: i8) {
        while to_bb != 0 {
            let to = pop_lsb(&mut to_bb);
            let from = Square::from_index((to as i8) - step);
            if T != (MoveGenType::Quiet as u8) {
                self.push(Move::promotion(from, to, PromotionType::Queen));
            }
            if T != (MoveGenType::Capture as u8) {
                self.push(Move::promotion(from, to, PromotionType::Knight));
                self.push(Move::promotion(from, to, PromotionType::Bishop));
                self.push(Move::promotion(from, to, PromotionType::Rook));
            }
        }
    }

    pub fn fill_enpassants(&mut self, mut from_bb: Bitboard, to: Square) {
        while from_bb != 0 {
            let sq = pop_lsb(&mut from_bb);
            self.push(Move::enpassant(sq, to));
        }
    }
}