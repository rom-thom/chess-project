
use chess_core::{piece::Piece, position::Position};

use crate::{constants::score};




#[derive(Clone)]
pub struct Evaluator {
    pub vals: [i32; 5], // P,N,B,R,Q
}


impl Default for Evaluator {
    fn default() -> Self {
        Self { vals: [score::PAWN_VAL, score::KNIGHT_VAL, score::BISHOP_VAL, score::ROOK_VAL, score::QUEEN_VAL] }
    }
}


impl Evaluator{
    pub fn piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Queen  => score::QUEEN_VAL,
            Piece::Rook   => score::ROOK_VAL,
            Piece::Bishop => score::BISHOP_VAL,
            Piece::Knight => score::KNIGHT_VAL,
            Piece::Pawn => score::PAWN_VAL,
            Piece::King => score::KING_VAL
        }
    }


    // TODO this should be a lot more extensive, looking at the positional state and so on
    pub fn evaluate(&self, pos: &Position) -> i32 { 
        if self.is_threefold(pos){return 0}
        
        self.evaluate_material(pos) + self.evaluate_piece_pos(pos)
    }

}