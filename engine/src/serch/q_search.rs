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
        if search_limit.check_stop(){ return NegamaxHelperReturn::Abort }


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
                                        let stand_pat = self.eval.evaluate(pos);

                                        if stand_pat >= beta {
                                            self.tt.store(
                                                pos.zobrist_key(),
                                                0,
                                                stand_pat,
                                                Bound::Lower,
                                                None,
                                                ply,
                                                true,
                                            );

                                            return NegamaxHelperReturn::Score(stand_pat);
                                        }

                                        local_alpha = local_alpha.max(stand_pat);
                                        stand_pat
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
        if !legal_move_found{ // No legal q_moves moves in the position => q serch out "mike drop" // TODO: Check for stalemate!!!!!!!!!!!!!!
            return NegamaxHelperReturn::Score(local_alpha)
        }


        let bound = if best_score <= alpha_original{ Bound::Upper } 
            else if best_score >= beta { Bound::Lower } 
            else{ Bound::Exact };

        self.tt.store(pos.zobrist_key(), 0, best_score, bound, best_move, ply, true); // TODO: trans stuf here

        NegamaxHelperReturn::Score(local_alpha)

    }


}




// Qsearch checklist (in order) ✅

// Abort check

// If time is up / stop flag set → return Abort.

// If side to move is in check

// You must search evasion moves (king moves, blocks, captures).

// In-check qsearch is basically “search one more ply (or more) until not in check”.

// Still use alpha–beta.

// Stand pat

// Compute static eval: stand_pat = evaluate(pos).

// This is the score if you “do nothing tactical”.

// Beta cutoff using stand pat

// If stand_pat >= beta → return beta (or stand_pat, but beta is common).

// Raise alpha

// If stand_pat > alpha → set alpha = stand_pat.

// Generate q-moves only

// Captures ✅

// Promotions ✅ (include non-capture promotions too)

// (Optional later) checks ✅

// En passant counts as capture ✅

// Order the q-moves

// Simple first: MVV-LVA (most valuable victim / least valuable attacker)

// Or: promotions first, then winning captures.

// Loop through q-moves with alpha–beta
// For each q-move:

// Make move

// Recurse qsearch with negamax bounds: score = -qsearch(-beta, -alpha)

// Undo move

// If score >= beta → return beta (cutoff)

// If score > alpha → update alpha = score

// Return alpha

// When no more q-moves (or none improved alpha), return alpha.

