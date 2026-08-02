use chess_core::{moves::{BitMove, MoveList}, piece::Piece, position::Position};

use crate::{eval::Evaluator, stored_moves::{trans_pos_table::TT}, constants::{max_values::MAX_PLY, score}};



pub struct MoveOrderer{
    killer_moves: [[Option<BitMove>; 2]; MAX_PLY]
}


impl MoveOrderer{
    pub fn new() -> Self{
        Self{killer_moves: [[None, None]; MAX_PLY]}
    }
    pub fn new_search(&mut self) {
        self.killer_moves = [[None; 2]; MAX_PLY];
    }

    pub fn on_beta_cutoff(&mut self, mov: &BitMove, ply: i32){
        let Ok(ply) = usize::try_from(ply) else {
            return;
        };

        let Some(killers) = self.killer_moves.get_mut(ply) else {
            return;
        };

        if killers[0] != Some(*mov) {
            killers[1] = killers[0];
            killers[0] = Some(*mov);
        }
    }

    pub fn sort(&self, pos: &Position, moves: &mut MoveList, tt_move: Option<BitMove>, ply: i32){
        // pos: position before the move is taken

        let mut scores = [0i32; 256];


        for (idx, &mov) in moves.iter().enumerate() {
            scores[idx] = self.score_move(pos, mov, tt_move, ply);
        }

        for current in 0..moves.size(){
            let mut best = current;

            for candidate in current+1..moves.size(){
                if scores[candidate] > scores[best]{
                    best = candidate;
                }
            }
            moves.swap(current, best);
            scores.swap(current, best);
        }
    }



    pub fn score_move(&self, pos: &Position, mov: BitMove, tt_move: Option<BitMove>, ply: i32) -> i32{
        if Some(mov) == tt_move {
            return score::TT_SCORE;
        }

        let boards = &pos.current.bitboards;

        if let Some(piece) = mov.get_premotion_piece(){
            let mut score = score::PROMOTION_SCORE + Evaluator::piece_value(piece);

            if mov.is_capture(){
                if let Some(captured_piece) = mov.get_captured_piece(boards) {
                    score += Evaluator::piece_value(captured_piece.to_piece());
                }
            }
            return score
        }

        if mov.is_capture(){
            let capture_value = 16 * Evaluator::piece_value(mov.get_captured_piece(boards).expect("I have seen that it is a capture").to_piece()) - Evaluator::piece_value(mov.get_moving_piece(boards).to_piece());

            return score::CAPTURE_SCORE + capture_value;

        }

        if Some(mov) == self.killer_moves[ply as usize][0]{return score::KILLER_SCORE + 100} 
        if Some(mov) == self.killer_moves[ply as usize][1]{return score::KILLER_SCORE - 100}

        if let Ok(ply) = usize::try_from(ply) {
            if let Some(killers) = self.killer_moves.get(ply) {
                if Some(mov) == killers[0] {
                    return score::KILLER_SCORE + 100;
                }
                if Some(mov) == killers[1] {
                    return score::KILLER_SCORE - 100;
                }
            }
        }

        // TODO Continue giving a score for differente types of move like the killer ones and history good ones

        0
    }
}

impl Default for MoveOrderer {
    fn default() -> Self {
        Self::new()
    }
}
