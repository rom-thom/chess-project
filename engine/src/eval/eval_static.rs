use std::i32;

// For static evaluation
use chess_core::position::{Color, Position};
use chess_core::piece::PieceIndex;
use chess_core::board::Bitboards;

use crate::constants::piece_square_table;
use super::Evaluator;


impl Evaluator{

    // Low-level, pure, easy to unit-test (chat did this function)
    #[inline(always)]
    fn material_from_bitboards(&self, bb: &Bitboards, color: Color) -> i32 {
        let pcs = bb.color_slice(color);           // [P,N,B,R,Q,K]
        let mut sum = 0i32;
        for (k, &b) in pcs.iter().take(5).enumerate() {
            sum += (b.count_ones() as i32) * self.vals[k]; // VALS = [100,320,330,500,900]
        }
        if matches!(color, Color::White) { sum } else { -sum }
    }


    #[inline]
    pub fn evaluate_material(&self, pos: &Position) -> i32 {
        // If i add a material cache (when i find out what that is), return it here:
        // return pos.material_cache[color as usize];

        let eval = self.material_from_bitboards(&pos.current.bitboards, pos.current.side_to_move) + 
        self.material_from_bitboards(&pos.current.bitboards, !pos.current.side_to_move);
        match pos.current.side_to_move { // This makes it advantage for side to move => positive number
            Color::Black => -eval,
            Color::White => eval
        }
    }


}


#[test]
fn test_eval(){
    let pos = Position::new(Some("8/3k4/5q2/8/5Q2/3K4/8/8 w - - 0 1".to_string()));
    let eval = Evaluator::default();
    dbg!(eval.evaluate(&pos));
}