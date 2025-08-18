use std::{cmp::{max, min}, fmt::Debug, i32};

use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::{Color, Position}};

use crate::{serch, static_eval::evaluate};





pub fn serch_alpha_beta<const DEPTH: usize>(pos: &mut Position, debug_depth: usize) -> (MovePath<DEPTH>, i32){
    let mut move_path = MovePath::<DEPTH>::new_empty();
    let alpha = i32::MIN;
    let beta = i32::MAX;

    serch_alpha_beta_helper(pos, DEPTH, debug_depth, &mut move_path, alpha, beta)
}

fn serch_alpha_beta_helper<const DEPTH: usize>(pos: &mut Position, depth: usize, debug_depth: usize, move_path: &mut MovePath<DEPTH>,alpha: i32, beta: i32) -> (MovePath<DEPTH>, i32){
    let legal_moves = MoveGen::legal_moves(&pos);
    let mut best_move_path = MovePath::<DEPTH>::new_empty();
    let mut best_evaluation = None; // i want it to change imedeately after finding the first move without needig to manualy find an eval of the position in the depth i am serching // TODO Maybe change to option later  for clarity

    let (mut local_alpha, mut local_beta) = (alpha, beta);


    if depth == 0 || legal_moves.is_empty(){ // When i reach the end of a branch
        return (move_path.clone(), evaluate(&pos));
    }

    if (DEPTH - depth) < (debug_depth+1) {//?? For debugging
        dbg!(&move_path);
    }

    let maximizing = pos.current.side_to_move == Color::White;

    for m in legal_moves.iter(){
        pos.make_move(m);
        move_path.push(*m);
        let (current_move_path, current_eval) = serch_alpha_beta_helper::<DEPTH>(pos, depth-1, debug_depth, move_path, local_alpha, local_beta);
        move_path.pop();
        pos.undo_move().expect("I should be able to undo the move as it has just been done");

        

        // Compare with previous moves:
        let better = if maximizing {
            current_eval > best_evaluation.unwrap_or(i32::MIN) // This becomes true if there arent alredy a checkmate inbound i32::MIN and this move is beter than the previous
        } else {
            current_eval < best_evaluation.unwrap_or(i32::MAX)
        };

        if better{
            best_evaluation = Some(current_eval);
            best_move_path = current_move_path;
        }

        let best_eval = best_evaluation.expect("I should be able to get that, as it is has just bean desided above");


        if maximizing{
            local_alpha = max(local_alpha, best_eval);
        } 
        else{
            local_beta = min(local_beta, best_eval);
        }
        if local_beta <= local_alpha{
            break
        }

    }  


    (best_move_path, best_evaluation.expect("Here should only be a wrong none if there were no legal moves, which i have checkded for i think"))

}






#[test]
fn test_serch(){
    let mut pos = Position::new(Some("r1bqk2r/pppp1ppp/2n2n2/2b1p3/4P3/2NP1N2/PPP2PPP/R1BQKB1R w KQkq - 2 5"));
    dbg!(&pos);
    dbg!(serch_alpha_beta::<5>(&mut pos, 0));

}
