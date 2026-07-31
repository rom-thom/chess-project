use chess_core::{moves::{BitMove, MoveList}, piece::Piece, position::Position};

use crate::trans_pos_table::TT;


const TT_SCORE: i32 = 1_000_000;
const PROMOTION_SCORE: i32 = 900_000;
const CAPTURE_SCORE: i32 = 800_000;
const KILLER_SCORE: i32 = 700_000;

pub struct MoveOrderer;


impl MoveOrderer{
    pub fn sort(pos: &Position, moves: &mut MoveList, tt_move: Option<BitMove>){
        let mut scores = [0i32; 256];


        for (idx, &mov) in moves.iter().enumerate() {
            scores[idx] = Self::score_move(pos, mov, tt_move);
        }
        // TODO: Continue with the sorting
    }



    fn score_move(pos: &Position, mov: BitMove, tt_move: Option<BitMove>) -> i32{
        if Some(mov) == tt_move {
            return TT_SCORE;
        }

        if let Some(piece) = mov.get_premotion_piece(){
            return PROMOTION_SCORE - (Piece::Queen as i32 - promotion_score(piece));
        }

        // TODO Continue giving a scorre for differente types of move

        0
    }
}


fn promotion_score(piece: Piece) -> i32 {
    match piece {
        Piece::Queen  => 4,
        Piece::Rook   => 3,
        Piece::Bishop => 2,
        Piece::Knight => 1,
        _ => 0,
    }
}