use crate::{eval::Evaluator, serch::serch_structs::{SearchLimits, SearchResult}, trans_pos_table::TT};





pub struct Engine{
    tt: TT,
    eval: Evaluator
}


impl Engine{
    pub fn new(tt_size: usize) -> Self{
        Self { tt: TT::new(tt_size), eval: Evaluator::default() }
    }







    pub fn think(&mut self, limits: SearchLimits){

        let best = SearchResult::default();
        let max_depth = limits.max_depth.unwrap_or(64);

        for depth in 1..=max_depth{

            

        }
    }
}





