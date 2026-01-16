use crate::{eval::Evaluator, trans_pos_table::TT};





pub struct Engine{
    tt: TT,
    eval: Evaluator
}


impl Engine{
    pub fn new(tt_size: usize) -> Self{
        Self { tt: TT::new(tt_size), eval: Evaluator::default() }
    }
}