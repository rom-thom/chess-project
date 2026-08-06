use std::i32;

use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::{Color, Position}};
use crate::{engine::Engine, constants::score, serch::{move_ordering::MoveOrderer, serch_structs::{SearchLimits, SearchResult}}, stored_moves::trans_pos_table::Bound};
use crate::eval::Evaluator;

#[cfg(feature = "progress")]
use indicatif::{ProgressBar, ProgressStyle};

use std::time::Duration;




impl Engine{

    pub fn negamax(
        &mut self,
        pos: &mut Position,
        serch_depth: usize,
        mut search_limit: &mut SearchLimits,
    ) -> SearchResult {
        if serch_depth != 0 && search_limit.check_stop(){
            return SearchResult::abort();
        }

        let mut alpha = -score::INF;
        let beta = score::INF;
        let ply = 0;

        let mut best_move = None;
        let mut best_score = -score::INF - 1;
        let mut legal_moves = MoveGen::legal_moves(pos);

        if legal_moves.size() == 0{
            if pos.in_check(pos.current.side_to_move){ // This means mate i think
                return SearchResult { best_move, score: -score::MATE, pv:MovePath::new_empty(), depth: 0, aborted: false }
            }
            return SearchResult { best_move, score: 0, pv:MovePath::new_empty(), depth: 0, aborted: false }
        }

        if serch_depth == 0{
            return SearchResult {score: self.eval.evaluate(pos), depth: 0, best_move: Some(*legal_moves.get(0).expect("there should be a legal move as i just checked wether it was empty")), pv: MovePath::new_empty(), aborted: false,};
        }
        
        let tt_result = self.tt.probe(pos.zobrist_key(), serch_depth as i8, alpha, beta,ply, false);
        self.move_orderer.sort(pos, &mut legal_moves, tt_result.best, ply);



        // -------- progress (feature gated) --------
        #[cfg(feature = "progress")]
        let pb = {
            let pb = ProgressBar::new(legal_moves.size() as u64);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} root {pos}/{len} [{elapsed_precise}] {wide_bar} {percent}% ETA {eta_precise} {msg}",
                )
                .unwrap()
                .progress_chars("=>-"),
            );

            // Force immediate visible activity:
            pb.set_message("starting…");
            pb.enable_steady_tick(Duration::from_millis(100)); // updates even before first inc()
            pb.tick(); // draw ASAP (extra nudge)

            pb
        };
        #[cfg(feature = "progress")]
        pb.set_message(format!("depth: {}", serch_depth));
        // -----------------------------------------

        for m in legal_moves.iter() {
            pos.make_move(m);

            let return_nega = self.negamax_helper(
                pos,
                serch_depth - 1,
                &mut search_limit,
                -beta,
                -alpha,
                ply + 1,
                
            );

            pos.undo_move()
                .expect("I just made a move, so i should be good");

            let score = match return_nega {
                NegamaxHelperReturn::Score(score_) => -score_,
                NegamaxHelperReturn::Abort => {
                    #[cfg(feature = "progress")]
                    pb.finish_and_clear();
                    return SearchResult::abort();
                }
            };

            if score > best_score {
                best_score = score;
                best_move = Some(*m);
            }

            alpha = alpha.max(score);


            #[cfg(feature = "progress")]
            {
                pb.inc(1);
            }

            if alpha >= beta {
                self.move_orderer.on_beta_cutoff(m, ply, &pos.current.bitboards, serch_depth);
                break;
            }

        }

        #[cfg(feature = "progress")]
        pb.finish_and_clear();

        SearchResult {
            score: best_score,
            depth: serch_depth,
            best_move,
            pv: MovePath::new_empty(),
            aborted: false,
        }
    }




    fn negamax_helper(&mut self, pos: &mut Position, serch_depth: usize, mut search_limit: &mut SearchLimits, alpha: i32, beta: i32, ply: i32) -> NegamaxHelperReturn{

        // Checks to end the loops
        
        if self.eval.is_threefold(pos) { return NegamaxHelperReturn::Score(0); }
        if pos.current.halfmove_clock >= 100{ return NegamaxHelperReturn::Score(0); }
        if search_limit.check_stop(){ return NegamaxHelperReturn::Abort }
        if serch_depth == 0{ return self.q_search(pos, search_limit, alpha, beta, ply); }



        // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do) I think i just had a stroke writing that
        let alpha_orig = alpha;
        let mut local_alpha = alpha;
        let moving_color = pos.current.side_to_move;
        let mut pseudo_legal = MoveGen::pseudo_legal(pos);



        // TT checking
        let tt_probe_result = self.tt.probe(pos.zobrist_key(), serch_depth as i8, alpha, beta, ply, false);
        
        if let Some(tt_cutoff) = tt_probe_result.cutoff{ return NegamaxHelperReturn::Score(tt_cutoff) }

        self.move_orderer.sort(pos, &mut pseudo_legal, tt_probe_result.best, ply); // Sorts the moves to hopefully speed up the alpha beta


        // I want to find the best move for the entry in TT
        let mut best_move: Option<BitMove> = None;
        let mut best_score = -score::INF-1;
        let mut legal_move_found = false;


        for m in pseudo_legal.iter(){
            if !MoveGen::castle_checks(pos, m, moving_color) {continue;}

            pos.make_move(m); 

            if pos.in_check(moving_color){ 
                pos.undo_move().expect("I should be able to undo the move as it has just been done");
                continue;
            }
            legal_move_found = true;

            let return_nega = self.negamax_helper(pos, serch_depth-1, &mut search_limit, -beta, -local_alpha, ply + 1);
            pos.undo_move().expect("I should be able to undo the move as it has just been done");
            
            let current_score = match return_nega{
                NegamaxHelperReturn::Score(score_) => -score_,
                NegamaxHelperReturn::Abort => return NegamaxHelperReturn::Abort
            };


            if current_score > best_score{
                best_score = current_score;
                best_move = Some(*m);
            }

            local_alpha = local_alpha.max(current_score);

            if local_alpha >= beta{
                
                self.move_orderer.on_beta_cutoff(m, ply, &pos.current.bitboards, serch_depth);
                
                break;
            }
            
        }

        
        if !legal_move_found{ // No legal moves in the position
            if pos.in_check(pos.current.side_to_move){ return NegamaxHelperReturn::Score(-score::MATE + ply) }
            else{ return NegamaxHelperReturn::Score(0); }
        }


        let bound = if best_score <= alpha_orig{ Bound::Upper } 
            else if best_score >= beta { Bound::Lower } 
            else{ Bound::Exact };

        self.tt.store(pos.zobrist_key(), serch_depth as i8, best_score, bound, best_move, ply, false);

        NegamaxHelperReturn::Score(best_score)

    }

}


pub enum NegamaxHelperReturn{
    Score(i32),
    Abort
}



#[test]
fn test_serch(){
    let mut pos = Position::new(Some("7k/5bpr/4R3/q3b3/1p5N/3B4/p3K1Q1/8 w - - 0 1".to_string()));
    dbg!(&pos);
    let mut engine = Engine::new(524288);
    dbg!(engine.negamax(&mut pos, 9, &mut SearchLimits::new(None, None, None, None)));

}
