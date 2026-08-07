
use chess_core::{position::Position, log};

use crate::{constants::score, eval::Evaluator, serch::{move_ordering::MoveOrderer, serch_structs::{SearchLimits, SearchResult}}, stored_moves::trans_pos_table::TT};





pub struct Engine{
    pub tt: TT,
    pub eval: Evaluator,
    pub move_orderer: MoveOrderer,

    pub opening_book_enabled: bool,
}


impl Engine{
    pub fn new(tt_size: usize, opening_book_enabled: bool) -> Self{
        Self { tt: TT::new(tt_size), eval: Evaluator::default(), move_orderer: MoveOrderer::default(), opening_book_enabled}
    }







    pub fn think_iterative_deepening(&mut self, mut pos: &mut Position, mut limits: &mut SearchLimits) -> SearchResult{

        let max_depth = limits.max_depth.unwrap_or(64); // It wil never reach 64 in depth so that is safe (unless i become god or something)

        let mut result = None;

        limits.start_new_search();
        self.tt.new_search();
        self.move_orderer.new_search();

        for depth in 0..=max_depth {
            let temp_result = self.negamax(&mut pos, depth, &mut limits);

            if temp_result.aborted{
                log!(format!("depth: {depth}, Aborted"), path="search_scores.log").unwrap();
            }
            else if temp_result.score.abs() >= score::MATE_THRESHOLD{
                log!(format!("depth: {depth}, mate distance {}, move: {}", (score::MATE - temp_result.score.abs()) * temp_result.score.signum() as i32, &(temp_result.best_move.map(|m| m.to_string()).unwrap_or_else(|| "None".to_string()))), path="search_scores.log").unwrap();
            }  
            else{
                log!(format!("depth: {depth}, eval = {}, move: {}", temp_result.score, temp_result.best_move.expect("This should be a move as it wasnt aborted").to_string()), path="search_scores.log").unwrap();
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
    let mut engine = Engine::new(524288, false);
    let mut limits = SearchLimits::new(Some(8), None, None, None);
    dbg!(engine.think_iterative_deepening(&mut pos, &mut limits));

}
