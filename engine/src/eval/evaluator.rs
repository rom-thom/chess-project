
use chess_core::position::Position;

use crate::debug_file::log_dbg;



#[derive(Clone)]
pub struct Evaluator {
    pub vals: [i32; 5], // P,N,B,R,Q
}

impl Default for Evaluator {
    fn default() -> Self {
        Self { vals: [100, 320, 330, 500, 900] }
    }
}


impl Evaluator{



    // TODO this should be a lot more extensive, looking at the positional state and so on
    pub fn evaluate(&self, pos: &Position) -> i32 { 
        if self.is_threefold(pos){
            log_dbg("/tmp/debug_file.log", "there was a threefold played", &"indeed it was", file!(), line!()).expect("ja de e feil me log fakstisk");
            return 0}
        
        self.evaluate_material(pos) + self.evaluate_piece_pos(pos)
    }

}