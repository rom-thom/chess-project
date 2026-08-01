
use chess_core::{kastling::Imposter::King, piece::Piece, position::Position};

use crate::debug_file::log_dbg;

const KING_VAL: i32 = 100_000; // "My lord ..., he is priceless! Don't you dare put a value on him", "Well i just did bitch"
const QUEEN_VAL: i32 = 900;
const ROOK_VAL: i32 = 500;
const BISHOP_VAL: i32 = 330;
const KNIGHT_VAL: i32 = 320;
const PAWN_VAL: i32 = 100;




#[derive(Clone)]
pub struct Evaluator {
    pub vals: [i32; 5], // P,N,B,R,Q
}


impl Default for Evaluator {
    fn default() -> Self {
        Self { vals: [PAWN_VAL, KNIGHT_VAL, BISHOP_VAL, ROOK_VAL, QUEEN_VAL] }
    }
}


impl Evaluator{
    pub fn piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Queen  => QUEEN_VAL,
            Piece::Rook   => ROOK_VAL,
            Piece::Bishop => BISHOP_VAL,
            Piece::Knight => KNIGHT_VAL,
            Piece::Pawn => PAWN_VAL,
            Piece::King => KING_VAL
        }
    }


    // TODO this should be a lot more extensive, looking at the positional state and so on
    pub fn evaluate(&self, pos: &Position) -> i32 { 
        if self.is_threefold(pos){
            log_dbg("/tmp/debug_file.log", "there was a threefold played", &"indeed it was", file!(), line!()).expect("ja de e feil me log fakstisk");
            return 0}
        
        self.evaluate_material(pos) + self.evaluate_piece_pos(pos)
    }

}