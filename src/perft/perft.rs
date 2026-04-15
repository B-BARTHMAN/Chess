use crate::moves::movelist::MoveList;
use crate::movegen::movegen::generate;
use crate::position::position::Position;

pub fn perft(fen: &str, depth: u8) -> u64 {
    let mut pos = Position::from_fen(fen);
    perft_i(&mut pos, depth)
}

pub fn perft_divide(fen: &str, depth: u8) {
    let mut pos = Position::from_fen(fen);

    let mut move_list = MoveList::new();
    generate(&pos, &mut move_list);

    let mut total = 0u64;
    for m in move_list.iter() {
        pos.do_move(*m);
        if pos.is_legal() {
            let nodes = perft_i(&mut pos, depth - 1);
            total += nodes;
            println!("{}: {}", m, nodes);
        }
        pos.undo_move(*m);
    }
    println!("Total: {}", total);
}

fn perft_i(pos: &mut Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    generate(pos, &mut move_list);

    let mut total = 0u64;
    for m in move_list.iter() {
        pos.do_move(*m);
        if pos.is_legal() {
            total += perft_i(pos, depth - 1);
        }
        pos.undo_move(*m);
    }
    total
}