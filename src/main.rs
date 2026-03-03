use crate::magic::{find_magic, find_magic_square};
use crate::types::{PieceType, Square};

mod position;
mod types;
mod bitboard;
mod movegen;
mod movelist;
mod magic;

fn main() {
    let x = find_magic_square::<{PieceType::Bishop as u8}>(Square::E4, 9);
    if x.is_some() {
        println!("{}", x.unwrap());
    }
}
