use std::{cmp::{max, min}, fmt::Debug, i32};

use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::{Color, Position}};
use crate::{engine::Engine, score, serch::serch_result::SearchResult};
use crate::eval::Evaluator;







impl Engine{

    pub fn negamax(&self, pos: &mut Position, serch_depth: usize) -> SearchResult{
        let mut move_path = MovePath::new_empty();
        let alpha = -score::INF; // start out small and increse it to approach beta 
        let beta = score::INF;

        let evaluator = Evaluator::default();

        let (path, score) = self.negamax_helper(pos, serch_depth, &evaluator, &mut move_path, alpha, beta);
        SearchResult { best_move: path.get(0), score, pv: path, depth: serch_depth}
    }



    fn negamax_helper(&self, mut pos: &mut Position, serch_depth: usize, evaluator: &Evaluator, move_path: &mut MovePath,alpha: i32, beta: i32) -> (MovePath, i32){
        // TODO: later i should make it so that i dont pass down the move path, but rather just the eval, and then i can use the TT to see what was the best move from there
        
        
        let legal_moves = MoveGen::legal_moves(&mut pos);

        let mut best_evaluation = None; // i want it to change imedeately after finding the first move without needig to manualy find an eval of the position in the depth i am serching // TODO Maybe change to option later  for clarity


        // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do) I think i just had a stroke writing that
        let mut local_alpha = alpha;
        let mut best_pv = MovePath::new_empty(); // Best prinsipal value from all childrens tree

        if pos.current.halfmove_clock == 50{
            return (best_pv, 0);
        }
        if legal_moves.is_empty() {
            if pos.in_check(pos.current.side_to_move){
                return (best_pv, -score::INF)
            }
            else{
                return (best_pv, 0);
            }
        }
        if serch_depth == 0{ // When i reach the end of a branch
            // TODO look for wether it can capture, and go deeper down that path until no one can capture


            return (best_pv, evaluator.evaluate(&pos));
        }

        for m in legal_moves.iter(){

            pos.make_move(m);
            let (mut child_pv, temp_current_eval) = self.negamax_helper(pos, serch_depth-1, evaluator, move_path, -beta, -local_alpha);
            pos.undo_move().expect("I should be able to undo the move as it has just been done");


            let current_eval = -temp_current_eval;

            if best_evaluation.map_or(true, |best_eval| current_eval > best_eval) {
                best_pv.clear();
                best_pv.push(*m);
                best_pv.append(&mut child_pv);
                best_evaluation = Some(current_eval);


                local_alpha = max(local_alpha, current_eval);
                if local_alpha >= beta{break;}
            }


        }  

        (best_pv, best_evaluation.expect("Here should only be a none if there were no legal moves, which i have checked for i think"))

    }

}








#[test]
fn test_serch(){
    let mut pos = Position::new(Some("r1bqk2r/pppp1ppp/2n2n2/2b1p3/4P3/2NP1N2/PPP2PPP/R1BQKB1R w KQkq - 2 5"));
    dbg!(&pos);
    let engine = Engine::new(8);
    dbg!(engine.negamax(&mut pos, 5));

}
