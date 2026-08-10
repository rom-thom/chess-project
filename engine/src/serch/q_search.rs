use chess_core::moves::BitMove;
use chess_core::{movegen::MoveGen, position::Position};

use crate::constants::score;
use crate::serch::move_ordering::MoveOrderer;
use crate::stored_moves::trans_pos_table::Bound;
use crate::{engine::Engine, eval::Evaluator, serch::serch_structs::SearchLimits};
use crate::serch::serch::NegamaxHelperReturn; // <- add this






impl Engine{



    pub fn q_search(&mut self, pos: &mut Position, search_limit: &mut SearchLimits, alpha: i32, beta: i32, ply: i32) -> NegamaxHelperReturn{


        // Checks to end the loops
        
        if self.eval.is_threefold(pos) { return NegamaxHelperReturn::Score(0); }
        if pos.current.halfmove_clock >= 100{ return NegamaxHelperReturn::Score(0); }
        if search_limit.visit_node_and_check_stop(){ return NegamaxHelperReturn::Abort }


        // alpha is pased down as -beta and vise versa, and we only ever change the alpha as we just change what we can improve ourselves (we cant change what the opponent else where can do) I think i just had a stroke writing that
        let alpha_original = alpha;
        let mut local_alpha = alpha;
        let moving_color = pos.current.side_to_move;

        let in_check = pos.in_check(pos.current.side_to_move);


        let mut pseudo_legal = if in_check{ MoveGen::pseudo_legal(pos) }
                                    else { MoveGen::pseudo_legal_q_moves(pos) };



        // TT checking
        let tt_probe_result = self.tt.probe(pos.zobrist_key(), 0, alpha, beta, ply, true);
        
        if let Some(tt_cutoff) = tt_probe_result.cutoff{ return NegamaxHelperReturn::Score(tt_cutoff) }


        self.move_orderer.sort(pos, &mut pseudo_legal, tt_probe_result.best, ply);

        // I want to find the best move for the entry in TT
        let mut best_move: Option<BitMove> = None;

        let mut best_score = if in_check {-score::INF} 
                                  else {
                                        let stand_by = self.eval.evaluate(pos);

                                        if stand_by >= beta {
                                            if !MoveGen::has_legal_move(pos) {return NegamaxHelperReturn::Score(0);}

                                            self.tt.store( pos.zobrist_key(), 0, stand_by, Bound::Lower, None, ply, true,);

                                            return NegamaxHelperReturn::Score(stand_by);
                                        }

                                        local_alpha = local_alpha.max(stand_by);
                                        stand_by
                                    };



        let mut legal_move_found = false;


        for m in pseudo_legal.iter(){
            if !MoveGen::castle_checks(pos, m, moving_color) {continue;}

            pos.make_move(m); 

            if pos.in_check(moving_color){ 
                pos.undo_move().expect("I should be able to undo the move as it has just been done");
                continue;
            }
            legal_move_found = true;

            let return_q_nega = self.q_search(pos, &mut *search_limit, -beta, -local_alpha, ply + 1);
            pos.undo_move().expect("I should be able to undo the move as it has just been done");
            
            let current_score = match return_q_nega{
                NegamaxHelperReturn::Score(score_) => -score_,
                NegamaxHelperReturn::Abort => return NegamaxHelperReturn::Abort
            };

            if current_score > best_score{
                best_score = current_score;
                best_move = Some(*m);
            }

            local_alpha = local_alpha.max(current_score);

            if current_score >= beta { 
                self.tt.store(pos.zobrist_key(), 0, current_score, Bound::Lower, Some(*m), ply, true);
                return NegamaxHelperReturn::Score(current_score); 
            }
            
        }

        if in_check && !legal_move_found {
            return NegamaxHelperReturn::Score(-score::MATE + ply);
        }
        if !legal_move_found{ // No legal q_moves moves in the position => q serch out "mike drop" 
            if MoveGen::has_legal_move(pos){ return NegamaxHelperReturn::Score(local_alpha) }
            else{return NegamaxHelperReturn::Score(0)}
        }


        let bound = if best_score <= alpha_original{ Bound::Upper } 
            else if best_score >= beta { Bound::Lower } 
            else{ Bound::Exact };

        self.tt.store(pos.zobrist_key(), 0, best_score, bound, best_move, ply, true);

        NegamaxHelperReturn::Score(local_alpha)

    }


}


