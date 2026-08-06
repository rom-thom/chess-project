use std::time;

use chess_core::position::Position;

use crate::{constants::score, debug_file, eval::Evaluator, opening, serch::{move_ordering::MoveOrderer, serch_structs::{SearchLimits, SearchResult}}, stored_moves::trans_pos_table::TT};





pub struct Engine{
    pub tt: TT,
    pub eval: Evaluator,
    pub move_orderer: MoveOrderer
}


impl Engine{
    pub fn new(tt_size: usize) -> Self{
        Self { tt: TT::new(tt_size), eval: Evaluator::default(), move_orderer: MoveOrderer::default()}
    }







    pub fn think_iterative_deepening(&mut self, mut pos: &mut Position, mut limits: &mut SearchLimits) -> SearchResult{

        let max_depth = limits.max_depth.unwrap_or(64); // It wil never reach 64 in depth so that is safe (unless i become god or something)

        let mut result = None;

        limits.start_new_search();
        self.tt.new_search();
        self.move_orderer.new_search();

        for depth in 0..=max_depth {
            let temp_result = self.negamax(&mut pos, depth, &mut limits);

            if temp_result.score.abs() >= score::MATE_THRESHOLD{
                debug_file::log_dbg("chess_log/search_scores.log", &format!("depth: {depth}, aborted = {}, mate distance {}", temp_result.aborted, (score::MATE - temp_result.score.abs()) * temp_result.score.signum() as i32), Some(&(temp_result.best_move.map(|m| m.to_string()).unwrap_or_else(|| "None".to_string()))), file!(), line!()).expect("dbg funkakje");
            }
            else if temp_result.aborted{
                debug_file::log_dbg("chess_log/search_scores.log", &format!("depth: {depth}, Aborted"), None::<&String>, file!(), line!()).expect("dbg funkakje");
            }
            else{
                debug_file::log_dbg("chess_log/search_scores.log", &format!("depth: {depth}, eval = {}", temp_result.score), Some(&(temp_result.best_move.map(|m| m.to_string()).unwrap_or_else(|| "None".to_string()))), file!(), line!()).expect("dbg funkakje");
            }


            if temp_result.aborted {
                break;
            }
            else{
                result = Some(temp_result);
            }

            // If i found the shoutest mate i want it now
            if let Some(result) = result.as_ref() {  
                if result.score.abs() >= score::MATE_THRESHOLD {
                    let mate_distance = score::MATE - result.score.abs();

                    if depth as i32 >= mate_distance - 1 {
                        break;
                    }
                }
            }

        }
        result.expect("the zero'th depth doesnt get interupted, so it always returns a valid non aborted result")

    }
}











#[test]
fn test_engine(){
    let mut pos = Position::new(Some("4b2k/6pr/8/q3b3/1p5N/3B4/p3K1Q1/8 w - - 0 1".to_string()));
    dbg!(&pos);
    let mut engine = Engine::new(524288);
    let mut limits = SearchLimits::new(Some(8), None, None, None);
    dbg!(engine.think_iterative_deepening(&mut pos, &mut limits));

}
