use std::{cmp::{max, min}, fmt::Debug, i32, sync::atomic::AtomicBool};

use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::{Color, Position}};
use crate::{engine::Engine, score, serch::serch_structs::{SearchLimits, SearchResult}, trans_pos_table::Bound};
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
        if search_limit.check_stop() {
            return SearchResult::abort();
        }

        let mut alpha = -score::INF;
        let beta = score::INF;


        let mut best_move = None;
        let mut best_score = -score::INF - 1;
        let legal_moves = MoveGen::legal_moves(pos);

        if legal_moves.size() == 0 || serch_depth == 0 { // TODO: This is wrong as if size is 0 it has to check for either mate or stalemate (either of which evaluate doesn't capture)
            return SearchResult {
                score: self.eval.evaluate(pos),
                depth: 0,
                best_move: None,
                pv: MovePath::new_empty(),
                aborted: false,
            };
        }

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
                break;
            }
        }

        #[cfg(feature = "progress")]
        pb.finish_and_clear();

        #[cfg(feature = "tt-stats")]
        self.tt.dump_stats(serch_depth);
        #[cfg(feature = "tt-stats")]
        self.tt.reset_stats();

        SearchResult {
            score: best_score,
            depth: serch_depth,
            best_move,
            pv: MovePath::new_empty(),
            aborted: false,
        }
    }




    fn negamax_helper(&mut self, pos: &mut Position, serch_depth: usize, mut search_limit: &mut SearchLimits, alpha: i32, beta: i32) -> NegamaxHelperReturn{

        
        let alpha_orig = alpha;

        // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do) I think i just had a stroke writing that
        let mut local_alpha = alpha;

        let moving_color = pos.current.side_to_move;

        if pos.current.halfmove_clock >= 100{
            return NegamaxHelperReturn::Score(0);
        }

        if search_limit.check_stop(){
            return NegamaxHelperReturn::Abort
        }

        let mut legal_moves = MoveGen::legal_moves(pos); // TODO: remove this after pseudolegal has been achieved
        let mut pseudo_legal = MoveGen::pseudo_legal(pos);


        if legal_moves.is_empty() {
            if pos.in_check(pos.current.side_to_move){
                return NegamaxHelperReturn::Score(-score::INF)
            }
            else{
                return NegamaxHelperReturn::Score(0);
            }
        }

        if serch_depth == 0{ // When i reach the end of a branch
            return self.q_search(pos, search_limit);
        }


        // TT checking
        let tt_probe_result = self.tt.probe(pos.zobrist_key(), serch_depth as i8, local_alpha, beta);
        
        if let Some(tt_cutoff) = tt_probe_result.cutoff{
            return NegamaxHelperReturn::Score(tt_cutoff)
        }

        if let Some(ttm) = tt_probe_result.best {
            pseudo_legal.bring_to_front(ttm); // Speedboost 
        }

        // I want to find the best move for the entry in TT
        let mut best_move: Option<BitMove> = None;
        let mut best_score = -score::INF-1;


        for m in pseudo_legal.iter(){
            if !MoveGen::castle_checks(pos, m, moving_color) {continue;}

            pos.make_move(m); 

            if pos.in_check(moving_color){ 
                pos.undo_move().expect("I should be able to undo the move as it has just been done");
                continue;
            }

            let return_nega = self.negamax_helper(pos, serch_depth-1, &mut search_limit, -beta, -local_alpha);
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

            if local_alpha >= beta{break;}
            
        }
        let bound = if best_score <= alpha_orig{
            Bound::Upper
        } else if best_score >= beta {
            Bound::Lower
        } else{
            Bound::Exact
        };

        self.tt.store(pos.zobrist_key(), serch_depth as i8, best_score, bound, best_move);

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
    dbg!(engine.negamax(&mut pos, 9, &mut SearchLimits::new(None, None, None)));

}
