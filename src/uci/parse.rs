use crate::board::file::file_of;
use crate::board::rank::rank_of;
use crate::board::square::Square;
use crate::movegen::movegen::generate;
use crate::moves::chess_move::Move;
use crate::moves::move_type::MoveType;
use crate::moves::movelist::MoveList;
use crate::moves::promotion_type::PromotionType;
use crate::position::position::Position;

fn parse_square(s: &str) -> Option<Square> {
  let b = s.as_bytes();
  if b.len() != 2 { return None; }
  if !(b'a'..=b'h').contains(&b[0]) || !(b'1'..=b'8').contains(&b[1]) { return None; }
  let idx = (b[1] - b'1') as i8 * 8 + (b[0] - b'a') as i8;
  Some(Square::from_index(idx))
}

fn square_to_uci(sq: Square) -> String {
  let file = file_of(sq) as u8 + b'a';
  let rank = rank_of(sq) as u8 + b'1';
  format!("{}{}", file as char, rank as char)
}

pub fn format_uci_move(m: Move) -> String {
  let mut s = format!("{}{}", square_to_uci(m.from()), square_to_uci(m.to()));
  if m.move_type() == MoveType::Promotion {
    s.push(match m.promotion_type() {
      PromotionType::Queen  => 'q',
      PromotionType::Rook   => 'r',
      PromotionType::Bishop => 'b',
      PromotionType::Knight => 'n',
    });
  }
  s
}

pub fn parse_uci_move(pos: &mut Position, s: &str) -> Option<Move> {
  if s.len() < 4 { return None; }
  let from = parse_square(&s[0..2])?;
  let to   = parse_square(&s[2..4])?;
  let promo_char: Option<char> = s.chars().nth(4);

  let mut ml = MoveList::new();
  generate(pos, &mut ml);

  for i in 0..ml.len() {
    let m = ml[i];
    if (m.from() as i8) != (from as i8) { continue; }
    if (m.to()   as i8) != (to   as i8) { continue; }

    let is_promo = m.move_type() == MoveType::Promotion;
    match (is_promo, promo_char) {
      (true, Some(c)) => {
        let ok = matches!((m.promotion_type(), c),
            (PromotionType::Queen,  'q') |
            (PromotionType::Rook,   'r') |
            (PromotionType::Bishop, 'b') |
            (PromotionType::Knight, 'n'));
        if !ok { continue; }
      }
      (false, None) => {}
      _ => continue,
    }

    // pseudo-legal match — confirm it's actually legal
    pos.do_move(m);
    let legal = pos.is_legal();
    pos.undo_move(m);
    if legal { return Some(m); }
  }
  None
}