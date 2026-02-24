use std::{cmp::{max, min}, fmt::Debug, i32, sync::atomic::AtomicBool};

use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::{Color, Position}};
use crate::{engine::Engine, score, serch::serch_structs::{SearchLimits, SearchResult}, trans_pos_table::Bound};
use crate::eval::Evaluator;






impl Engine{

    pub fn negamax(&mut self, pos: &mut Position, serch_depth: usize, mut search_limit: &mut SearchLimits) -> SearchResult{
        if search_limit.check_stop(){
            return SearchResult::abort();
        }
        let mut alpha = -score::INF; // start out small and increse it to approach beta 
        let beta = score::INF;

        let evaluator = Evaluator::default();


        // I want to find a move here at the root, but i dont need it else where
        let mut best_move = None;
        let mut best_score = -score::INF-1;
        let legal_moves = MoveGen::legal_moves(pos);

        

        if legal_moves.size() == 0 || serch_depth == 0{ // TODO Separarte these
            return SearchResult { score: self.eval.evaluate(pos), depth: 0, best_move: None, pv: MovePath::new_empty(), aborted: false};
        }

        for m in legal_moves.iter(){
            pos.make_move(m);
            let score = match self.negamax_helper(pos, serch_depth-1, &evaluator, &mut search_limit, -beta, -alpha){
                NegamaxHelperReturn::Score(score_) => -score_,
                NegamaxHelperReturn::Abort => return SearchResult::abort()
            };
            pos.undo_move().expect("I just made a move, so i should be good");
            if score > best_score {
                best_score = score;
                best_move = Some(*m);
            }
            alpha = alpha.max(score);
            if alpha >= beta { break; }
        }

        #[cfg(feature = "tt-stats")]
        self.tt.dump_stats(serch_depth);
        #[cfg(feature = "tt-stats")]
        self.tt.reset_stats();
        

        SearchResult { best_move: best_move, depth: serch_depth, score: best_score, pv: MovePath::new_empty(), aborted: false}
    }






    fn negamax_helper(&mut self, pos: &mut Position, serch_depth: usize, evaluator: &Evaluator, mut search_limit: &mut SearchLimits, alpha: i32, beta: i32) -> NegamaxHelperReturn{

        if serch_depth == 0{ // When i reach the end of a branch
            // TODO look for wether it can capture, and go deeper down that path until no one can capture


            return NegamaxHelperReturn::Score(evaluator.evaluate(pos));
        }
        
        let alpha_orig = alpha;



        // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do) I think i just had a stroke writing that
        let mut local_alpha = alpha;

        if pos.current.halfmove_clock >= 100{
            return NegamaxHelperReturn::Score(0);
        }

        if search_limit.check_stop(){
            return NegamaxHelperReturn::Abort
        }

        let mut legal_moves = MoveGen::legal_moves(pos);

        if legal_moves.is_empty() {
            if pos.in_check(pos.current.side_to_move){
                return NegamaxHelperReturn::Score(-score::INF)
            }
            else{
                return NegamaxHelperReturn::Score(0);
            }
        }


        // TT checking
        let tt_probe_result = self.tt.probe(pos.zobrist_key(), serch_depth as i8, local_alpha, beta);
        
        if let Some(tt_cutoff) = tt_probe_result.cutoff{
            return NegamaxHelperReturn::Score(tt_cutoff)
        }

        if let Some(ttm) = tt_probe_result.best {
            legal_moves.bring_to_front(ttm); // Speedboost (den blei heile 1% raskare) 
        }

        // I want to find the best move for the entry in TT
        let mut best_move = None;
        let mut best_score = -score::INF-1;


        for m in legal_moves.iter(){

            pos.make_move(m); 
            let current_score = match self.negamax_helper(pos, serch_depth-1, evaluator, &mut search_limit, -beta, -local_alpha){
                NegamaxHelperReturn::Score(score_) => -score_,
                NegamaxHelperReturn::Abort => return NegamaxHelperReturn::Abort
            };
            pos.undo_move().expect("I should be able to undo the move as it has just been done");


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


enum NegamaxHelperReturn{
    Score(i32),
    Abort
}



#[test]
fn test_serch(){
    let mut pos = Position::new(Some("r1bqk2r/pppp1ppp/2n2n2/2b1p3/4P3/2NP1N2/PPP2PPP/R1BQKB1R w KQkq - 2 5".to_string()));
    dbg!(&pos);
    let mut engine = Engine::new(8);
    dbg!(engine.negamax(&mut pos, 5, &mut SearchLimits::new(None, None, None)));

}
