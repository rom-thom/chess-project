use std::{cmp::{max, min}, fmt::Debug, i32};

use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::{Color, Position}};

use crate::{serch, static_eval::{evaluate, INF}};








pub fn serch_alpha_beta_negamax<const DEPTH: usize>(pos: &mut Position, debug_depth: usize) -> (MovePath<DEPTH>, i32){
    let mut move_path = MovePath::<DEPTH>::new_empty();
    let alpha = -INF; // start out small and increse it to approach beta 
    let beta = INF;

    serch_alpha_beta_negamax_helper(pos, DEPTH, debug_depth, &mut move_path, alpha, beta)
}

fn serch_alpha_beta_negamax_helper<const DEPTH: usize>(mut pos: &mut Position, depth: usize, debug_depth: usize, move_path: &mut MovePath<DEPTH>,alpha: i32, beta: i32) -> (MovePath<DEPTH>, i32){
    let legal_moves = MoveGen::legal_moves(&mut pos);
    let mut best_move_path = MovePath::<DEPTH>::new_empty();
    let mut best_evaluation = None; // i want it to change imedeately after finding the first move without needig to manualy find an eval of the position in the depth i am serching // TODO Maybe change to option later  for clarity


    // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do)
    let mut local_alpha = alpha;


    if legal_moves.is_empty(){
        if pos.in_check(pos.current.side_to_move){
            return (move_path.clone(), -INF)
        }
        else{
            return (move_path.clone(), 0);
        }
    }
    if depth == 0{ // When i reach the end of a branch
        // TODO look for wether it can capture, and go deeper down that path until nåwån can capture


        return (move_path.clone(), evaluate(&pos));
    }

    for m in legal_moves.iter(){
        pos.make_move(m);
        move_path.push(*m);


        if (DEPTH - depth) < (debug_depth) {//?? For debugging
            dbg!(&move_path);
        }


        let (current_move_path, temp_current_eval) = serch_alpha_beta_negamax_helper::<DEPTH>(pos, depth-1, debug_depth, move_path, -beta, -local_alpha);


        if (DEPTH - depth) < (debug_depth) {//?? For debugging
            dbg!(-temp_current_eval);
        }


        let current_eval = -temp_current_eval;
        move_path.pop();
        pos.undo_move().expect("I should be able to undo the move as it has just been done");

        

        // Compare with previous moves:
        let better = best_evaluation.map_or(true, |best_eval| current_eval > best_eval);


        if better{
            best_evaluation = Some(current_eval);
            best_move_path = current_move_path;


            local_alpha = max(local_alpha, current_eval);
            if local_alpha >= beta{break;}
        }

    }  

    (best_move_path, best_evaluation.expect("Here should only be a wrong none if there were no legal moves, which i have checked for i think"))

}

















pub fn serch_alpha_beta_negamax_deeper_capture<const DEPTH: usize>(pos: &mut Position, debug_depth: usize) -> (MovePath<DEPTH>, i32){
    let mut move_path = MovePath::<DEPTH>::new_empty();
    let alpha = -INF; // start out small and increse it to approach beta     
    let beta = INF;

    serch_alpha_beta_negamax_helper_deeper_capture(pos, DEPTH, debug_depth, &mut move_path, alpha, beta)
}

fn serch_alpha_beta_negamax_helper_deeper_capture<const DEPTH: usize>(mut pos: &mut Position, depth: usize, debug_depth: usize, move_path: &mut MovePath<DEPTH>,alpha: i32, beta: i32) -> (MovePath<DEPTH>, i32){
    let legal_moves = MoveGen::legal_moves(&mut pos);
    let mut best_move_path = MovePath::<DEPTH>::new_empty();
    let mut best_evaluation = None; // i want it to change imedeately after finding the first move without needig to manualy find an eval of the position in the depth i am serching // TODO Maybe change to option later  for clarity


    // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do)
    let mut local_alpha = alpha;


    if legal_moves.is_empty(){
        if pos.in_check(pos.current.side_to_move){
            return (move_path.clone(), -INF)
        }
        else{
            return (move_path.clone(), 0);
        } 
    }
    if depth == 0{ // When i reach the end of a branch

        // TODO look for wether it can capture, and go deeper down that path until nåwån can capture
        let current_eval = evaluate(&pos);
        'capture: {






            local_alpha = max(local_alpha, current_eval);

            if local_alpha >= beta{break 'capture;}




        }

        return (move_path.clone(), current_eval);
    }

    for m in legal_moves.iter(){
        pos.make_move(m);
        move_path.push(*m);


        if (DEPTH - depth) < (debug_depth) {//?? For debugging
            dbg!(&move_path);
        }


        let (current_move_path, temp_current_eval) = serch_alpha_beta_negamax_helper_deeper_capture::<DEPTH>(pos, depth-1, debug_depth, move_path, -beta, -local_alpha);


        if (DEPTH - depth) < (debug_depth) {//?? For debugging
            dbg!(-temp_current_eval);
        }


        let current_eval = -temp_current_eval;
        move_path.pop();
        pos.undo_move().expect("I should be able to undo the move as it has just been done");

        

        // Compare with previous moves:
        let better = best_evaluation.map_or(true, |best_eval| current_eval > best_eval);


        if better{
            best_evaluation = Some(current_eval);
            best_move_path = current_move_path;


            local_alpha = max(local_alpha, current_eval);
            if local_alpha >= beta{break;}
        }

    }  

    (best_move_path, best_evaluation.expect("Here should only be a wrong none if there were no legal moves, which i have checked for i think"))

}





#[test]
fn test_serch(){
    let mut pos = Position::new(Some("r1bqk2r/pppp1ppp/2n2n2/2b1p3/4P3/2NP1N2/PPP2PPP/R1BQKB1R w KQkq - 2 5"));
    dbg!(&pos);
    dbg!(serch_alpha_beta_negamax::<5>(&mut pos, 0));

}
