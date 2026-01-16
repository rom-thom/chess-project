use chess_core::moves::{BitMove, MovePath};




#[derive(Debug)]
pub struct SearchResult {
    pub best_move: Option<BitMove>,
    pub score: i32,
    pub pv: MovePath,   // starting with best_move and as long as posible. Not always to the end of the serch
    pub depth: usize,
    // pub nodes: u64, Should maby have this for debuging but i cant be bathered
}
