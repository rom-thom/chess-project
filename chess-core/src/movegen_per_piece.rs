
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

                move_list.add(BitMove::new(start_square, end_square, is_capture, MoveType::Normal)); // It is allways a Quiet move in this part (cant be pawn move or )
            }



        }


    }

    #[inline]
    pub fn generate_normal_piece_q_moves(pos: &Position, piece: Piece, color: Color, move_list: &mut MoveList){ // Generate moves for Rock, Bishop, Knight, King, Queen (no fancyful kastling and so on)


        debug_assert!(piece != Piece::Pawn); // Pawns should not be accesable in this function (in release i assume it is taken care of)

        let mut piece_bitboard = pos.current.bitboards.get_bitboard(PieceIndex::from_piece(piece, color));

        let all_occ = pos.current.bitboards.all_occupancy;
        let opponent_occ = match color {
            Color::White => pos.current.bitboards.black_occupancy,
            Color::Black => pos.current.bitboards.white_occupancy
        };


        while let Some(piece_idx) = piece_bitboard.pop_lsb(){ // loopes trough the indexes of each of that spesific piece
            let start_square = square::Square::from_idx(piece_idx).expect("there should be a bit on the bitboard if i am inside this function");
            let mut q_attacks = attack::get_attacks(piece, start_square, all_occ, color) & opponent_occ;

            // loops trough the indexes of each 
            while let Some(attack_idx) = q_attacks.pop_lsb() {
                let end_square = square::Square::from_idx(attack_idx).expect("there should be a bit on the bitboard if i am inside this function");

                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Normal)); // It is allways a Quiet move in this part (cant be pawn move or )
            }
        }
    }


    // Pawn move gens

    pub fn generate_group_pawn_moves(pos: &Position, color: Color, move_list: &mut MoveList){
        
        let opponents_bb = match color {
            Color::Black => pos.current.bitboards.white_occupancy,
            Color::White => pos.current.bitboards.black_occupancy
        };
        let en_passant_bb = pos.current.en_passant.map_or( Bitboard::new_empty(),|square|square.to_bitboard());

        let (mut single_push, mut double_push) = attack::pawn_groupe_straight_moves(pos, color);
        let mut left_attack = attack::pawn_groupe_attacks_left(pos, color) & (opponents_bb | en_passant_bb);
        let mut right_attack = attack::pawn_groupe_attacks_right(pos, color) & (opponents_bb | en_passant_bb);


        let end_row = match color {
                Color::Black => 0,
                Color::White => 7
            };

        while let Some(single_push_idx) = single_push.pop_lsb() { // Loop single push
            let end_square = Square::from_idx(single_push_idx).expect("i used pop_lsb");
            let start_square = match color {
                Color::Black => end_square.upp(),
                Color::White => end_square.down()
            }.expect("There should be a startingsquare, because that is where the attackmask generated the move from");
            if end_square.row() == end_row{
                move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Bishop)));
                move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Queen)));
                move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Rook)));
                move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Knight)));

                continue;
            };
            move_list.add(BitMove::new(start_square, end_square, false, MoveType::Normal));
        }
        while let Some(double_push_idx) = double_push.pop_lsb() { // Loop double push
            let end_square = Square::from_idx(double_push_idx).expect("i used pop_lsb");


            let start_square = match color {
                Color::Black => end_square.upp()
                        .expect("There should be a startingsquare, because that is where the attackmask generated the move from")
                        .upp(),    
                Color::White => end_square.down()
                                    .expect("There should be a startingsquare, because that is where the attackmask generated the move from")
                                    .down(),
            }.expect("There should be a starting square, because that is where the attackmask generated the move from");


            move_list.add(BitMove::new(start_square, end_square, false, MoveType::EnPassant));
        }
        while let Some(left_attack_idx) = left_attack.pop_lsb() { // Loop capture left
            let end_square = Square::from_idx(left_attack_idx).expect("i used pop_lsb");
            let start_square = match color { //This should be the oposite direction of where the capture was done
                Color::Black => end_square.upp_right(),
                Color::White => end_square.down_right()
            }.expect("There should be a startingsquare, because that is where the attackmask generated the move from");

            if end_square.row() == end_row{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Bishop)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Queen)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Rook)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Knight)));

                continue;
            };

            let is_en_passant = pos.current.en_passant.map_or(false, |en_pas_sqr|en_pas_sqr.index() == left_attack_idx);
            
            if is_en_passant{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::EnPassant));
                continue;
            }
            move_list.add(BitMove::new(start_square, end_square, true, MoveType::Normal));
        }
        while let Some(right_attack_idx) = right_attack.pop_lsb() { // Loop capture right
            let end_square = Square::from_idx(right_attack_idx).expect("i used pop_lsb");
            let start_square = match color {
                Color::Black => end_square.upp_left(),
                Color::White => end_square.down_left()
            }.expect("There should be a startingsquare, because that is where the attackmask generated the move from");

            if end_square.row() == end_row{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Bishop)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Queen)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Rook)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Knight)));

                continue;
            };

            let is_en_passant = pos.current.en_passant.map_or(false, |en_pas_sqr|en_pas_sqr.index() == right_attack_idx);
            
            if is_en_passant{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::EnPassant));
                continue;
            }
            move_list.add(BitMove::new(start_square, end_square, true, MoveType::Normal));
        }

    }


    pub fn generate_group_q_pawn_moves(pos: &Position, color: Color, move_list: &mut MoveList){
        let (opponents_bb, my_pawn_bb) = match color {
            Color::Black => (pos.current.bitboards.white_occupancy, pos.current.bitboards.boards[PieceIndex::from_piece(Piece::Pawn, color).index()]),
            Color::White => (pos.current.bitboards.black_occupancy, pos.current.bitboards.boards[PieceIndex::from_piece(Piece::Pawn, color).index()])
        };
        
        let en_passant_bb = pos.current.en_passant.map_or( Bitboard::new_empty(),|square|square.to_bitboard());

        let mut left_attack = attack::pawn_groupe_attacks_left(pos, color) & (opponents_bb | en_passant_bb);
        let mut right_attack = attack::pawn_groupe_attacks_right(pos, color) & (opponents_bb | en_passant_bb);

        
        let (promotion_row, last_pawn_row, end_row) = match color {
                Color::Black => (bitboard_consts::RANK_1, bitboard_consts::RANK_2, 0),
                Color::White => (bitboard_consts::RANK_8, bitboard_consts::RANK_7, 7)
            };
        let mut last_rank_pawns = my_pawn_bb & last_pawn_row;
        let last_rank_unocupied = !pos.current.bitboards.all_occupancy & promotion_row;

        match color {
            Color::White => last_rank_pawns.shift_up(),
            Color::Black => last_rank_pawns.shift_down()
        }
        last_rank_pawns &= last_rank_unocupied;

        while let Some(promotion_pawn_idx) = last_rank_pawns.pop_lsb() { // Loop promotion forward
            let end_square = Square::from_idx(promotion_pawn_idx).expect("i used pop_lsb");
            let start_square = match color {
                Color::Black => end_square.upp(),    
                Color::White => end_square.down(),
            }.expect("There should be a starting square, because that is where the attackmask generated the move from");


            move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Bishop)));
            move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Queen)));
            move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Rook)));
            move_list.add(BitMove::new(start_square, end_square, false, MoveType::Promotion(Piece::Knight)));

        }
        while let Some(left_attack_idx) = left_attack.pop_lsb() { // Loop capture left
            let end_square = Square::from_idx(left_attack_idx).expect("i used pop_lsb");
            let start_square = match color { //This should be the oposite direction of where the capture was done
                Color::Black => end_square.upp_right(),
                Color::White => end_square.down_right()
            }.expect("There should be a startingsquare, because that is where the attackmask generated the move from");

            if end_square.row() == end_row{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Bishop)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Queen)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Rook)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Knight)));

                continue;
            };

            let is_en_passant = pos.current.en_passant.map_or(false, |en_pas_sqr|en_pas_sqr.index() == left_attack_idx);
            
            if is_en_passant{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::EnPassant));
                continue;
            }
            move_list.add(BitMove::new(start_square, end_square, true, MoveType::Normal));
        }
        while let Some(right_attack_idx) = right_attack.pop_lsb() { // Loop capture right
            let end_square = Square::from_idx(right_attack_idx).expect("i used pop_lsb");
            let start_square = match color {
                Color::Black => end_square.upp_left(),
                Color::White => end_square.down_left()
            }.expect("There should be a startingsquare, because that is where the attackmask generated the move from");

            if end_square.row() == end_row{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Bishop)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Queen)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Rook)));
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::Promotion(Piece::Knight)));

                continue;
            };

            let is_en_passant = pos.current.en_passant.map_or(false, |en_pas_sqr|en_pas_sqr.index() == right_attack_idx);
            
            if is_en_passant{
                move_list.add(BitMove::new(start_square, end_square, true, MoveType::EnPassant));
                continue;
            }
            move_list.add(BitMove::new(start_square, end_square, true, MoveType::Normal));
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
    let pos = Position::new(Some("r7/8/3P4/1Pp5/1k6/8/8/7K w - c6 0 1".to_string()));
    dbg!(&pos);
    let mut list = MoveList::new_empty();
    MoveGen::generate_group_pawn_moves(&pos, Color::White, &mut list);
    dbg!(list.size());
    for i in list.iter(){
        dbg!(i.to_string());
    }
}