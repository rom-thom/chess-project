
use crate::board::{Bitboards, Bitboard};
use crate::moves::{BitMove, MoveList, MoveType};
use crate::piece::{Piece, PieceIndex};
use crate::square::{Square};
use crate::kastling::{Castling, CastlingSide, Imposter};
use crate::attack;
use crate::bitboard_consts::{self, CORNERS};
use crate::position::{Color, Position};

impl Position{
    // Move generation (finds only the one for the color that currently is to move)
    // Finds all the pseudo legal (legal except for checks) moves in that position
        // TODO loop through all pieces and their moves for the color to move and put them into MoveList
    pub fn pseudo_legal(&self, move_list: &mut MoveList){
        let all_occ = self.current.bitboards.all_occupancy;
        let color = self.current.side_to_move;
        
        let (my_occ, opponent_occ) = match self.current.side_to_move {
            Color::White => (self.current.bitboards.white_occupancy, self.current.bitboards.black_occupancy),
            Color::Black => (self.current.bitboards.black_occupancy, self.current.bitboards.white_occupancy)
        };

        let mut my_occ_loop = my_occ; // represents the pieces that i haven't accesed yet



        'start_loopy: loop{
            let idx = match my_occ_loop.pop_lsb() {
                Some(index) => index,
                None=> break 'start_loopy // no name needed, but loopy is a cute name so i'll keep it
            };
            let start_square = Square::from_idx(idx).expect("Nr 1. Position::pseudo_legal finds an index outside of the square.");
            let (start_row, start_col) = start_square.to_coord();

            // TODO This should probably be changed when implementing Mailbox (to only look for the color you are searching)
            let piece_index = self.current.bitboards.piece_on_square(start_square)
                                                          .expect("Position::pseudo_legal does not find a piece where it should be, as the index should be where the piece is.");

            let piece = Piece::from_piece_index(&piece_index);

            let mut attacks = attack::get_attacks(piece_index, start_square, all_occ, color);
            attacks &= !my_occ;


            let mut has_castled = false;

            'attack_loop: loop{
                let attack_idx = match attacks.pop_lsb() {
                    Some(attack_index) => attack_index,
                    None => break 'attack_loop
                };
                
                let end_square = Square::from_idx(attack_idx).expect("Nr 2. Position::pseudo_legal finds an index outside of the square.");
                let (end_row, end_col) = end_square.to_coord();

                let mut is_capture = opponent_occ.is_occupied(end_square);

                let mut move_type = MoveType::Quiet;

                if piece_index == PieceIndex::WhitePawn || piece_index == PieceIndex::BlackPawn{
                    if start_col != end_col{

                        'diagonal_pawn: {
                        match self.current.en_passant {
                            Some(en_passant_square) => {
                                if end_square == en_passant_square{
                                    is_capture = true;
                                    move_type = MoveType::EnPassant;
                                    break 'diagonal_pawn;
                                }

                            },
                            None=>()
                        }
                        if !opponent_occ.is_occupied(end_square){
                            continue 'attack_loop;
                        }
                        }
                    }
                    if start_col == end_col {
                        if piece_index == PieceIndex::WhitePawn && start_row == 1 && end_row == 3 {
                            move_type = MoveType::EnPassant;
                        } else if piece_index == PieceIndex::BlackPawn && start_row == 6 && end_row == 4 {
                            move_type = MoveType::EnPassant;
                        }
                    }
                    
                    

                    if end_row == 0 || end_row == 7{ // Premotion

                        let promo_piece_list = [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen];

                        for promo_piece in promo_piece_list{
                            move_type = MoveType::Promotion(promo_piece);
                            move_list.add(BitMove::new(start_square, end_square, is_capture, move_type));
                        }
                        continue 'attack_loop;
                    }

                }

                // Handle castling (it can castle if can_castle variable is set for that side and pieces are cleared)
                if piece == Piece::King && !has_castled {
                    has_castled = true;
                    const BB_MASKS: [(CastlingSide, Square, Bitboard); 4] = [
                        (CastlingSide::WK, Square::G1, Bitboard::new_const(0x60)), // The bitboards represent the squares between king and rook
                        (CastlingSide::WQ, Square::C1, Bitboard::new_const(0x0E)),
                        (CastlingSide::BK, Square::G8, Bitboard::new_const(0x6000000000000000)),
                        (CastlingSide::BQ, Square::C8, Bitboard::new_const(0x0E00000000000000)),
                    ];

                    for (side, target_square, mask) in BB_MASKS.iter().cloned() {
                        let is_right_color = match (color, side) {
                            (Color::White, CastlingSide::WK | CastlingSide::WQ) => true,
                            (Color::Black, CastlingSide::BK | CastlingSide::BQ) => true,
                            _ => false,
                        };

                        if is_right_color
                            && self.current.castling.can_castle(side)
                            && !all_occ.intersects(mask) // makes shure no piece is between rook and king
                        {
                            move_list.add(BitMove::new(
                                start_square,
                                target_square,
                                is_capture,
                                MoveType::Castling(Imposter::from_castling_side(side)),
                            ));
                        }
                    }
                }



                move_list.add(BitMove::new(start_square, end_square, is_capture, move_type));
                }
            }
        }
    

    // Changes the position according to the move  // TODO find a beter way, i just did what my first instingt was
    pub fn make_move(&mut self, mov: BitMove){// TODO Mailbox must be updated here when implemented

        self.history.push(self.current);

        // predefined variables
        let color = self.current.side_to_move;
        
        let (my_occ, opponent_occ) = match self.current.side_to_move {
            Color::White => (self.current.bitboards.white_occupancy, self.current.bitboards.black_occupancy),
            Color::Black => (self.current.bitboards.black_occupancy, self.current.bitboards.white_occupancy)
        };
        let all_occ = self.current.bitboards.all_occupancy;

        let all_bit_boards = self.current.bitboards;

        let start_square = mov.get_start_square();
        let end_square = mov.get_end_square();
        let piece_index = mov.get_piece(&all_bit_boards);
        let piece = Piece::from_piece_index(&piece_index);
        

        let mut captured_piece = None;
        if mov.is_capture(){
            if !mov.is_en_passant(){
                captured_piece = Some(self.current.bitboards.piece_on_square(end_square).expect("didnt find captured piece on square in make_move") )
            }
            else{
                captured_piece = match color {
                    Color::Black => Some(PieceIndex::BlackPawn),
                    Color::White => Some(PieceIndex::WhitePawn),
                } 
            }
        }
        

        self.current.bitboards.remove(piece_index, start_square);


        self.current.halfmove_clock += 1; // this is always incremented unles a pawn move or a capture is made
        if color == Color::Black{
            self.current.fullmove_number += 1;
        }

        // Remove the captured piece
        if mov.is_capture(){
            self.current.halfmove_clock = 0;

            if mov.is_en_passant(){
                let (enemy_pawn_rank, captured_piece) = match color {
                    Color::White => (4, PieceIndex::BlackPawn),
                    Color::Black => (3, PieceIndex::WhitePawn),
                };
                let captured_square = Square::from_coords(enemy_pawn_rank, end_square.to_coord().1).expect("Make_move: didnt find a piece on square that is suposed to be enemy piece captured, during en-passant");
                self.current.bitboards.remove(captured_piece, captured_square);
            }
            else{
                let captured_piece = self.current.bitboards.piece_on_square(end_square).expect("Make_move: didnt find a piece on square that is suposed to be enemy piece captured");
                self.current.bitboards.remove(captured_piece, end_square);
            }
        }

        // Setting the end square (both pawn premotion and normal)
        match mov.get_premotion_piece(){ // This must be after capture, otherwise we might screw with the bitboards (set a bit before removing others)
            Some(promo_piece) => self.current.bitboards.set(PieceIndex::from_piece(promo_piece, color), end_square),
            None => self.current.bitboards.set(piece_index, end_square)
        }

        self.current.en_passant = None;
        if piece == Piece::Pawn{
            self.current.halfmove_clock = 0;
            if mov.is_double_pawn_push(){
                self.current.en_passant = match color {
                    Color::Black => Some(Square::from_coords(5, end_square.to_coord().1).expect("Make_move: en_passant square was not correct")),
                    Color::White => Some(Square::from_coords(2, end_square.to_coord().1).expect("Make_move: en_passant square was not correct"))
                };
            }
        }
        

        

        // ej etter ej skreiv dinna koda:
        //  (×_×)
        //   /|\
        //   / \
        // Den må forenklast og forbedrast
        match mov.get_castle_side(){
            Some(side) => {
                
                let (rock_piece, rock_row, start_rock_col, end_rook_col) = match color {
                    Color::Black => {
                        if side == Imposter::King{
                            self.current.castling.remove_castling_right(CastlingSide::BK);
                            (PieceIndex::BlackRook, 7, 7, 5)
                        }
                        else {
                            self.current.castling.remove_castling_right(CastlingSide::BQ);
                            (PieceIndex::BlackRook, 7, 0, 3)
                        }
                    },
                    Color::White => {
                        if side == Imposter::King{
                            self.current.castling.remove_castling_right(CastlingSide::WK);
                            (PieceIndex::WhiteRook, 0, 7, 5)
                        }
                        else {
                            self.current.castling.remove_castling_right(CastlingSide::WQ);
                            (PieceIndex::WhiteRook, 0, 0, 3)
                        }
                    }
                };

                let rook_square_start = Square::from_coords(rock_row, start_rock_col).expect("make_move: Invalid rook square during castling");
                self.current.bitboards.remove(rock_piece, rook_square_start);
                let rook_square_end = Square::from_coords(rock_row, end_rook_col).expect("make_move: Invalid rook square during castling");
                self.current.bitboards.set(rock_piece, rook_square_end);
            },
            None => ()
        }

        fn castling_side_for_corner(sq: Square) -> Option<CastlingSide> {
                match sq {
                    Square::A1 => Some(CastlingSide::WQ),
                    Square::H1 => Some(CastlingSide::WK),
                    Square::A8 => Some(CastlingSide::BQ),
                    Square::H8 => Some(CastlingSide::BK),
                    _       => None,
                }
            }

        if let Some(cap_piece) = captured_piece { // See if anything captures the rock in the corner
            if cap_piece.to_piece() == Piece::Rook && end_square.to_bitboard().intersects(bitboard_consts::CORNERS){
                if let Some(side) = castling_side_for_corner(end_square) {
                    self.current.castling.remove_castling_right(side);
                }
            }
        }
        if piece == Piece::Rook && start_square.to_bitboard().intersects(CORNERS){ // See if rook moves
            if let Some(side) = castling_side_for_corner(start_square) {
                self.current.castling.remove_castling_right(side);
            }
        }
    
        if piece == Piece::King{
            match color {
                Color::White => {self.current.castling.remove_castling_right(CastlingSide::WK); 
                                 self.current.castling.remove_castling_right(CastlingSide::WQ)},
                Color::Black => {self.current.castling.remove_castling_right(CastlingSide::BK); 
                                 self.current.castling.remove_castling_right(CastlingSide::BQ)},
            }
        }
        


        self.current.side_to_move = !self.current.side_to_move;

        
    }

    // TODO I should swap this out for faster check for wether it is ilegal or not
    // checks if the move generates a check and thus is legal or not
    pub fn makes_self_check(&self, mov: BitMove) -> bool{
        let mut temp_pos = self.clone();
        temp_pos.make_move(mov);

        let mut move_list = MoveList::new_empty();

        let self_king = match !temp_pos.current.side_to_move {
            Color::Black => PieceIndex::BlackKing,
            Color::White => PieceIndex::WhiteKing
        };

        temp_pos.pseudo_legal(&mut move_list);
        for moves in move_list.iter(){
            if temp_pos.current.bitboards.piece_on_square(moves.get_end_square()) == Some(self_king){
                return true
            }
        }
        false
    }

    // TODO This should probably change to a faster way, but for now i am to lacy
    pub fn fill_legal(&self, move_list: &mut MoveList){
        move_list.clear();

        let mut list = MoveList::new_empty();
        self.pseudo_legal(&mut list);

        for mov in list.iter(){
            if !self.makes_self_check(*mov){
                move_list.add(*mov);
            }
        }
    }

    // !!! Only for debuging
    pub fn legal_moves(&self)->MoveList{
        let mut move_list = MoveList::new_empty();
        self.fill_legal(&mut move_list);
        move_list
    }



}






#[cfg(test)]
mod test{
    use std::ptr::dangling;

    use crate::moves::Move;
    use rand::Rng;
    

    use super::*;
    
    #[test]
    fn test_board(){
        let mut position = Position::new(Some("8/P7/8/8/8/8/5k2/7K w - - 0 1"));
        dbg!(&position);
        let mut rng = rand::rng();
        for i in 0..50{
            let moves = position.legal_moves();
            let nr = rng.random_range(0..moves.size());
            position.make_move(*moves.get(nr).unwrap());
            dbg!(position.current.bitboards.all_occupancy);
        }



        
        // dbg!(&position.current);



    }
}