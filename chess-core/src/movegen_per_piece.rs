
use crate::board::{Bitboards, Bitboard};
use crate::movegen::MoveGen;
use crate::moves::{BitMove, MoveList, MoveType};
use crate::piece::{Piece, PieceIndex};
use crate::square::{self, Square};
use crate::kastling::{Castling, CastlingSide, Imposter};
use crate::{attack, position};
use crate::bitboard_consts::{self, CASTLE_EMPTY_BLACK_KINGSIDE, CASTLE_EMPTY_BLACK_QUEENSIDE, CASTLE_EMPTY_WHITE_KINGSIDE, CASTLE_EMPTY_WHITE_QUEENSIDE, CORNERS, WHITE_KING};
use crate::position::{Color, Position};





impl MoveGen{ // generates pseudo legal piece moves 
    #[inline]
    pub fn generate_normal_piece_moves(pos: &Position, piece: Piece, color: Color, move_list: &mut MoveList){ // Generate moves for Rock, Bishop, Knight, King, Queen (no fancyful kastling and so on)


        debug_assert!(piece != Piece::Pawn); // Pawns should not be accesable in this function (in release i assume it is taken care of)

        let mut piece_bitboard = pos.current.bitboards.get_bitboard(PieceIndex::from_piece(piece, color));
        let all_occ = pos.current.bitboards.all_occupancy;
        let (my_occ, opponent_occ) = match color {
            Color::White => (pos.current.bitboards.white_occupancy, pos.current.bitboards.black_occupancy),
            Color::Black => (pos.current.bitboards.black_occupancy, pos.current.bitboards.white_occupancy)
        };


        while let Some(piece_idx) = piece_bitboard.pop_lsb(){ // loopes trough the indexes of each rock 
            let start_square = square::Square::from_idx(piece_idx).expect("there should be a bit on the bitboard if i am inside this function");
            let attacks = attack::get_attacks(piece, start_square, all_occ, color);
            let mut attacks_not_on_mine = attacks & (!my_occ);

            // loops trough the indexes of each 
            while let Some(attack_idx) = attacks_not_on_mine.pop_lsb() {
                let end_square = square::Square::from_idx(attack_idx).expect("there should be a bit on the bitboard if i am inside this function");


                let is_capture = end_square.to_bitboard().intersects(opponent_occ);

                move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Quiet)); // It is allways a Quiet move in this part (cant be pawn move or )
            }



        }


    }

    // Pawn move gens

    pub fn generate_pawn_moves(pos: &Position, color: Color, move_list: &mut MoveList){ // TODO Change this to act on every pawn at once
        let all_occ = pos.current.bitboards.all_occupancy;
        let mut pawn_bitboard = pos.current.bitboards.get_bitboard(PieceIndex::from_piece(Piece::Pawn, color));

        let (las_rank, start_rank, double_pawn_push_end_rank) = match color {
            Color::Black => (bitboard_consts::RANK_1, bitboard_consts::RANK_7, bitboard_consts::RANK_5),
            Color::White => (bitboard_consts::RANK_8, bitboard_consts::RANK_2, bitboard_consts::RANK_4)
        };

        let (my_occ, opponent_occ) = match color {
            Color::White => (pos.current.bitboards.white_occupancy, pos.current.bitboards.black_occupancy),
            Color::Black => (pos.current.bitboards.black_occupancy, pos.current.bitboards.white_occupancy)
        };

        while let Some(pawn_idx) = pawn_bitboard.pop_lsb() {
            let start_square = square::Square::from_idx(pawn_idx).expect("there should be a bit on the bitboard if i am inside this function");
            let start_bb = start_square.to_bitboard();
            let mut pawn_moves = attack::pawn_moves(start_square, all_occ, color);
            
            while let Some(end_idx) = pawn_moves.pop_lsb() {
                let end_square = square::Square::from_idx(end_idx).expect("there should be a bit on the bitboard if i am inside this function");
                let end_bb = end_square.to_bitboard();
                let is_capture = end_bb.intersects(opponent_occ);

                
                if Some(end_square) == pos.current.en_passant{
                    move_list.add(BitMove::new(start_square, end_square, true, MoveType::EnPassant));
                    continue;
                }

                
                if end_square.col() != start_square.col() && !is_capture{
                    continue; // This is if the attack function for the pawn just gave the correct diagonals, there wernt any pieces there
                }
                
                if end_square.to_bitboard().intersects(las_rank){
                    move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Promotion(Piece::Queen)));
                    move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Promotion(Piece::Knight)));
                    move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Promotion(Piece::Rook)));
                    move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Promotion(Piece::Bishop)));
                    continue;
                }
                
                if end_bb.intersects(double_pawn_push_end_rank) && start_bb.intersects(start_rank){
                    move_list.add(BitMove::new(start_square, end_square, false, MoveType::EnPassant)); // I have made the en passant for deskribing both making and taking en passant square
                    continue;
                }

                move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Quiet));

            }
        }
    }





     const CASTLE_WHITE: [(CastlingSide, Bitboard, Bitboard); 2] = [
        (CastlingSide::WK, bitboard_consts::CASTLE_EMPTY_WHITE_KINGSIDE, bitboard_consts::CASTLE_PATH_WHITE_KINGSIDE),
        (CastlingSide::WQ, bitboard_consts::CASTLE_EMPTY_WHITE_QUEENSIDE, bitboard_consts::CASTLE_PATH_WHITE_QUEENSIDE),
    ];

    const CASTLE_BLACK: [(CastlingSide, Bitboard, Bitboard); 2] = [
        (CastlingSide::BK, bitboard_consts::CASTLE_EMPTY_BLACK_KINGSIDE, bitboard_consts::CASTLE_PATH_BLACK_KINGSIDE),
        (CastlingSide::BQ, bitboard_consts::CASTLE_EMPTY_BLACK_QUEENSIDE, bitboard_consts::CASTLE_PATH_BLACK_QUEENSIDE),
    ];

    pub fn generate_kastling_moves(pos: &Position, color: Color, move_list: &mut MoveList){
        let all_occ = pos.current.bitboards.all_occupancy;
        let castling = pos.current.castling;

        let start_square = match color {
            Color::White => Square::E1,
            Color::Black => Square::E8,
        };

        let castling_side_and_bb = match color {
            Color::Black => MoveGen::CASTLE_BLACK,
            Color::White => MoveGen::CASTLE_WHITE
        };
        for (side, empty_squares, full_castle_path) in castling_side_and_bb{
            if !all_occ.intersects(empty_squares) && castling.can_castle(side){
                let end_square = match side {
                    CastlingSide::WK => Square::G1,
                    CastlingSide::WQ => Square::C1,
                    CastlingSide::BK => Square::G8,
                    CastlingSide::BQ => Square::C8,
                };

                // TODO check if the any squares in the path is attacked (apparently we do that here)

                move_list.add(BitMove::new(start_square, end_square, false, MoveType::Castling(Imposter::from_castling_side(side))));
            }
        }
    }
}







#[test]
fn test_piece_movegen(){
    let pos = Position::new(Some("8/4K3/1k6/3r4/8/8/1R6/4R3 w - - 0 1"));

    let mut list = MoveList::new_empty();
    MoveGen::generate_normal_piece_moves(&pos, Piece::Pawn, Color::White, &mut list);
    dbg!(list.size());
}