use crate::magic::{find_magic};
use crate::types::{PieceType};

mod position;
mod types;
mod bitboard;
mod movegen;
mod movelist;
mod magic;

use std::thread;

fn main() {
    let rook_handle = thread::spawn(|| {
        find_magic::<{ PieceType::Rook as u8 }>("magic_rook.txt");
    });

    let bishop_handle = thread::spawn(|| {
        find_magic::<{ PieceType::Bishop as u8 }>("magic_bishop.txt");
    });

    rook_handle.join().unwrap();
    bishop_handle.join().unwrap();
}
