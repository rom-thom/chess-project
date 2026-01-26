use std::time;

use chess_core::position::Position;

use crate::{eval::Evaluator, serch::serch_structs::{SearchLimits, SearchResult}, trans_pos_table::TT};





pub struct Engine{
    pub tt: TT,
    pub eval: Evaluator
}


impl Engine{
    pub fn new(tt_size: usize) -> Self{
        Self { tt: TT::new(tt_size), eval: Evaluator::default() }
    }







    pub fn think_iterative_deepening(&mut self, pos: &mut Position, limits: SearchLimits) -> SearchResult{

        let max_depth = limits.max_depth.unwrap_or(64); // It wil never reach 64 in depth so that is safe

        let mut result = SearchResult::default();

        let start_time = time::Instant::now();

        for depth in 1..=max_depth {
            result = self.negamax(pos, depth);

            if let Some(time_ms) = limits.max_time_ms {
                if time::Instant::now() - start_time <= time::Duration::from_millis(time_ms as u64){
                    return result
                }
            }
        }
        result

    }
}





