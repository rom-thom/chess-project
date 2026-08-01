use chess_core::{moves::{BitMove, MoveList}, piece::Piece, position::Position};

use crate::{eval::Evaluator, trans_pos_table::TT};


const TT_SCORE: i32 = 1_000_000;
const PROMOTION_SCORE: i32 = 900_000;
const CAPTURE_SCORE: i32 = 800_000;
const KILLER_SCORE: i32 = 700_000;

pub struct MoveOrderer;


impl MoveOrderer{
    pub fn sort(pos: &Position, moves: &mut MoveList, tt_move: Option<BitMove>){
        // pos: position before the move is taken

        let mut scores = [0i32; 256];


        for (idx, &mov) in moves.iter().enumerate() {
            scores[idx] = Self::score_move(pos, mov, tt_move);
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



    pub fn score_move(pos: &Position, mov: BitMove, tt_move: Option<BitMove>) -> i32{
        if Some(mov) == tt_move {
            return TT_SCORE;
        }

        let boards = &pos.current.bitboards;

        if let Some(piece) = mov.get_premotion_piece(){
            let mut score = PROMOTION_SCORE + Evaluator::piece_value(piece);

            if mov.is_capture(){
                if let Some(captured_piece) = mov.get_captured_piece(boards) {
                    score += Evaluator::piece_value(captured_piece.to_piece());
                }
            }
            return score
        }

        if mov.is_capture(){
            let capture_value = 16 * Evaluator::piece_value(mov.get_captured_piece(boards).expect("I have seen that it is a capture").to_piece()) - Evaluator::piece_value(mov.get_moving_piece(boards).to_piece());

            return CAPTURE_SCORE + capture_value;

        }


        // TODO Continue giving a score for differente types of move like the killer ones and history good ones

        0
    }
}
