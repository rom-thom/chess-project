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







    pub fn think_iterative_deepening(&mut self, mut pos: &mut Position, mut limits: &mut SearchLimits) -> SearchResult{

        let max_depth = limits.max_depth.unwrap_or(64); // It wil never reach 64 in depth so that is safe

        let mut result = None;

        limits.start_new_search();
        self.tt.new_search();

        for depth in 1..=max_depth {
            let temp_result = self.negamax(&mut pos, depth, &mut limits);

            if temp_result.aborted{
                break;
            }
            else{
                result = Some(temp_result);
            }

        }
        result.expect("Here should be a move at this point")

    }
}





