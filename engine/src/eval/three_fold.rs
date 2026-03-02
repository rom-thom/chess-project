use chess_core::position::Position;

use crate::eval::Evaluator;





///! For some reason this doesn't work at all



impl Evaluator{
pub fn is_threefold(&self, pos: &Position) -> bool {
    let key = pos.zobrist_key();
    let mut count = 1;

    // number of past positions we are willing to check
    let max_back = pos.current.halfmove_clock as usize;

    // scan backwards through history, up to max_back entries
    for (j, snap) in pos.history.iter().rev().enumerate() {
        if j >= max_back { break; }

        if snap.zobrist_key == key {
            count += 1;
            if count >= 3 {
                return true;
            }
        }
    }

    false
}
}
