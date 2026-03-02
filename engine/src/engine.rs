use std::time;

use chess_core::position::Position;

use crate::{eval::Evaluator, serch::serch_structs::{SearchLimits, SearchResult}, trans_pos_table::TT, debug_file};





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

            debug_file::log_dbg("/tmp/debug_file.log", &format!("depth: {}, aborted = {}, eval = {}", depth, temp_result.aborted, temp_result.score), &(temp_result.best_move.map(|m| m.to_string()).unwrap_or_else(|| "None".to_string())), file!(), line!()).expect("dbg funkakje");


            if temp_result.aborted{
                break;
            }
            else{
                result = Some(temp_result);
            }

        }
        result.expect("Here i should make a fallback move instead of relying on the first loop finishing")

    }
}











#[test]
fn test_engine(){
    let mut pos = Position::new(Some("4b2k/6pr/8/q3b3/1p5N/3B4/p3K1Q1/8 w - - 0 1".to_string()));
    dbg!(&pos);
    let mut engine = Engine::new(524288);
    let mut limits = SearchLimits::new(Some(6), None, None);
    dbg!(engine.think_iterative_deepening(&mut pos, &mut limits));

}
