use chess_core::position::Position;

use crate::piece_square_table;
use chess_core::piece::PieceIndex;
use super::Evaluator;


impl Evaluator{
    fn game_phase(&self, pos: &Position)-> (u32, u32){
        // weights: N=1, B=1, R=2, Q=4
        const MAX_PHASE: u32 = 24;

        let rook_count = pos.current.bitboards.get_bitboard(PieceIndex::WhiteRook).count_ones() + pos.current.bitboards.get_bitboard(PieceIndex::BlackRook).count_ones();
        let knight_count = pos.current.bitboards.get_bitboard(PieceIndex::WhiteKing).count_ones() + pos.current.bitboards.get_bitboard(PieceIndex::BlackKnight).count_ones();
        let bishop_count = pos.current.bitboards.get_bitboard(PieceIndex::WhiteBishop).count_ones() + pos.current.bitboards.get_bitboard(PieceIndex::BlackBishop).count_ones();
        let queen_count = pos.current.bitboards.get_bitboard(PieceIndex::WhiteQueen).count_ones() + pos.current.bitboards.get_bitboard(PieceIndex::BlackQueen).count_ones();

        (rook_count*2 + knight_count + bishop_count + queen_count*4, MAX_PHASE)

    }



    // start phase => phase = 0, endgame => phase = max_phase
    fn blend_phases(&self, mg: i32, eg: i32, phase: i32, max_phase: i32)->i32{
        (phase * (mg-eg))/max_phase + mg   // TODO this should be eg-mg, but for some rason that does not work.
    }

    pub fn evaluate_piece_pos(&self, pos: &Position)-> i32{
        let color = pos.current.side_to_move;
        let my_boards = pos.current.bitboards.color_slice(color);
        let opp_boards = pos.current.bitboards.color_slice(!color);


        let mut eg_score = 0; // endgame score
        let mut mg_score = 0; // midlegame_score

        // evaluating my own piece eval
        for (piece_type_idx, piece_bb) in my_boards.iter().enumerate(){
            let mut piece_bb_loop = *piece_bb;
            while let Some(lonly_piece_idx) = piece_bb_loop.pop_lsb() {
                mg_score += piece_square_table::SQUARE_VAL_TABLE_WHITE[0][piece_type_idx][lonly_piece_idx as usize] as i32;
                eg_score += piece_square_table::SQUARE_VAL_TABLE_WHITE[1][piece_type_idx][lonly_piece_idx as usize] as i32;
            }
        }
        for (piece_type_idx, piece_bb) in opp_boards.iter().enumerate(){
            let mut piece_bb_loop = *piece_bb;
            while let Some(lonly_piece_idx) = piece_bb_loop.pop_lsb() {
                mg_score -= piece_square_table::SQUARE_VAL_TABLE_WHITE[0][piece_type_idx][piece_square_table::blackify_idx(lonly_piece_idx as usize)] as i32;
                eg_score -= piece_square_table::SQUARE_VAL_TABLE_WHITE[1][piece_type_idx][piece_square_table::blackify_idx(lonly_piece_idx as usize)] as i32;
            }
        }
        let (phase, max_phase) = self.game_phase(pos);
        self.blend_phases(mg_score, eg_score, phase as i32, max_phase as i32)

    }
}