
use chess_core::position::Position;



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
        
        self.evaluate_material(pos) + self.evaluate_piece_pos(pos)
    }

}