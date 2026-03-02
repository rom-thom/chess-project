use chess_core::{movegen::MoveGen, position::Position};

use crate::{engine::Engine, eval::Evaluator, serch::serch_structs::SearchLimits};
use crate::serch::serch::NegamaxHelperReturn; // <- add this






impl Engine{

    pub fn q_search(&self, pos: &mut Position, mut search_limit: &mut SearchLimits) -> NegamaxHelperReturn{
        // TODO: make a function that simply finds the q moves instead of just filtering out the non q ones



        // TODO: make dis funcy shuncy (funksion)
        // if search_limit.check_stop(){
        //     return NegamaxHelperReturn::Abort
        // }

        // let legal_moves = MoveGen::legal_moves(pos);

        return NegamaxHelperReturn::Score(self.eval.evaluate(pos))
        
        
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

