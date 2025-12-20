
use crate::attack::{get_attacks, get_moves};
use crate::board::{Bitboards, Bitboard};
use crate::moves::{BitMove, MoveList, MoveType};
use crate::piece::{Piece, PieceIndex};
use crate::square::{Square};
use crate::kastling::{Castling, CastlingSide, Imposter};
use crate::{attack, position};
use crate::bitboard_consts::{self, CORNERS};
use crate::position::{Color, Position};


#[derive(Clone, Debug, PartialEq)]
pub struct MoveGen; // Just like a namespace for move generation

impl MoveGen {


    // Move generation (finds only the one for the color that currently is to move)
    // Finds all the pseudo legal (legal except for checks) moves in that position
    pub fn pseudo_legal(pos: &Position, mut move_list: &mut MoveList){
        let color = pos.current.side_to_move;
        MoveGen::generate_group_pawn_moves(pos, color, &mut move_list);

        // This just takes kare of the Quiet moves for those pieces
        MoveGen::generate_normal_piece_moves(pos, Piece::Bishop, color, &mut move_list);
        MoveGen::generate_normal_piece_moves(pos, Piece::Knight, color, &mut move_list);
        MoveGen::generate_normal_piece_moves(pos, Piece::Rook, color, &mut move_list);
        MoveGen::generate_normal_piece_moves(pos, Piece::Queen, color, &mut move_list);

        MoveGen::generate_kastling_moves(pos, color, &mut move_list);
        MoveGen::generate_normal_piece_moves(pos, Piece::King, color, &mut move_list);
    }

    // q = Quiesence (fancy word for captures, promotions and all that is not quiet as in noone ca take and so on)
    pub fn pseudo_legal_q(pos: &Position, mut move_list: &mut MoveList){
        
    }









    // mutable so that i dont need to clone (it is not changed)
    pub fn makes_self_check(pos: &mut Position, mov: BitMove) -> bool{
        let moving_color = pos.current.side_to_move;


        pos.make_move(&mov);
        

        if pos.in_check(moving_color){
            pos.undo_move().expect("i did at the start of this fnction do a move, and so this should work");
            return true
        }

        // At this point we know the king is not in check after the move
        //  if it ain't castling, then it's now legal
        if mov.get_castle_side().is_none(){
            pos.undo_move().expect("i did at the start of this fnction do a move, and so this should work");
            return false;
        }

        // Finds the path the king has to move trough. Used for finding wether opponent attacks that square
        let mut kastle_path = mov.get_castle_side().map(|castle_side|{
            match (moving_color, castle_side) {
                (Color::White, Imposter::King)  => bitboard_consts::CASTLE_PATH_WHITE_KINGSIDE,
                (Color::White, Imposter::Queen) => bitboard_consts::CASTLE_PATH_WHITE_QUEENSIDE,
                (Color::Black, Imposter::King)  => bitboard_consts::CASTLE_PATH_BLACK_KINGSIDE,
                (Color::Black, Imposter::Queen) => bitboard_consts::CASTLE_PATH_BLACK_QUEENSIDE,
            }
        }).expect("At this point the move should be castling as i just checked for that"); 





        while let Some(kastle_path_idx) = kastle_path.pop_lsb() {
            if pos.is_square_attacked(Square::from_idx(kastle_path_idx).expect("Here should be an index of the board as i use pop_lsb"), !moving_color){
                pos.undo_move().expect("i did at the start of this fnction do a move, and so this should work");
                return true;
            }
        }

        pos.undo_move().expect("i did at the start of this fnction do a move, and so this should work");
        // it cant castle if it is attacked before the castling
        if pos.in_check(moving_color){
            return true
        }

        false
    }


    pub fn fill_legal(pos: &mut Position, move_list: &mut MoveList){
            move_list.clear();

            let mut list = MoveList::new_empty();
            MoveGen::pseudo_legal(pos, &mut list);

            for mov in list.iter(){
                if !MoveGen::makes_self_check(pos, *mov){
                    move_list.add(*mov);
                }
            }
        }

        #[inline]
    pub fn legal_moves(pos: &mut Position)->MoveList{
        let mut move_list = MoveList::new_empty();
        MoveGen::fill_legal(pos, &mut move_list);
        move_list
    }

      // returns the move from  // ? it is very slow so dont use it for timecritical things
    pub fn stringmove_to_bitmove(pos: &mut Position, moves_str: &str)->Result<BitMove, String>{
        // This finds the last move as that is all we need for the engine, and if the string is weird, it removes mitaken extra spaces
        let last_move = moves_str.split(" ")
                                       .filter(|s| !s.is_empty())
                                       .last()
                                       .ok_or("No valid move was found in the moves_string when converting movestring to bitmove")?;

        let chars: Vec<char> = last_move.chars().collect();
        if chars.len() < 4{return Err("the last move was to short to have first and last move".to_string())}
        if chars.len() > 5{return Err("the last move was to long to only have first and last move and promotion".to_string())}

        let start_square_str = chars[0].to_string() + &chars[1].to_string();
        let end_square_str = chars[2].to_string() +  &chars[3].to_string();
        let start_square = (&start_square_str).parse()?;
        let end_square = (&end_square_str).parse()?;

        let mut promotion_piece = None;

        if chars.len() == 5{
            promotion_piece = Some(Piece::from_char(chars[4])
                                         .ok_or(format!("The end of the last move was not convertable to a promotion piece. This is the move you gave ({})", chars.iter().collect::<String>()))?);
        };
        let candidate_move = pos.expand_move(start_square, end_square, promotion_piece);


        // Compares to all the legal moves to see if it exist there
        let is_legal = {
            let mut lm = MoveList::new_empty(); // ? This is realy slow
            MoveGen::fill_legal(pos,&mut lm);
            lm.iter().any(|m|(*m) == candidate_move)
        };


        if !is_legal{
            return Err(String::from("That is an ilegal move"));
        }

        Ok(pos.expand_move(start_square, end_square, promotion_piece))
    }

    

    pub fn captures(&self, out: &mut MoveList){
        // TODO add a list that only looks at captures
        out.clear();
        
    }
}




impl Position{
    // Here is a checklist of things to check:
    /*
        Legality check
        Piece move (from→to)
        Capture / en passant remove
        Promotion replace
        Castling rook move
        Update castling rights
        Update en passant square
        Update halfmove clock
        Update fullmove number
        Flip side-to-move
        Update king square // TODO maybe for later
        Update bitboards / piece lists
        Update hash (Zobrist) // TODO maybe for later
        Push move history 
        Update Mailbox // TODO for later
    */
    // Changes the position according to the move
    pub fn make_move(&mut self, mov: &BitMove){

        self.history.push(self.current);

        // predefined variables
        let color = self.current.side_to_move;

        let all_bit_boards = &self.current.bitboards;

        let start_square = mov.get_start_square();
        let end_square = mov.get_end_square();
        let piece_index = mov.get_piece(&all_bit_boards);
        let piece = Piece::from_piece_index(&piece_index);



        self.current.halfmove_clock += 1; // this is always incremented unles a pawn move or a capture is made
        if color == Color::Black{
            self.current.fullmove_number += 1;
        }

        self.current.bitboards.remove(piece_index, start_square); // removes the starting square piece

        

        let mut captured_piece = None;
        if mov.is_capture(){ // Remove the captured piece and square
            self.current.halfmove_clock = 0;

            if !mov.is_en_passant(){

                captured_piece = Some(self.current.bitboards.piece_on_square(end_square).expect("didnt find captured piece on square in make_move"));
                self.current.bitboards.remove(captured_piece.expect("I just sat this as a Some"), end_square);
            }
            else{
                let (enemy_pawn_rank, captured_piece) = match color {
                    Color::White => (4, PieceIndex::BlackPawn),
                    Color::Black => (3, PieceIndex::WhitePawn),
                };
                let captured_square = Square::from_coords(enemy_pawn_rank, end_square.to_coord().1).expect("Make_move: didnt find a piece on square that is suposed to be enemy piece captured, during en-passant");
                self.current.bitboards.remove(captured_piece, captured_square); // removes the end square
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


    pub fn undo_move(&mut self) -> Result<(), &'static str> {

        if let Some(previous) = self.history.pop(){
            self.current = previous;
            return Ok(());
        }
        Err("There where no previous snapshot in the position history, aka you haven't done a move yet, there are no move to undo you idiot")
    }


    #[inline(always)]
    pub fn in_check(&self, color: Color) -> bool { // TODO Add attack table to loop trough what the opponent attack
        let ksq = self.current
            .bitboards
            .get_bitboard(PieceIndex::from_piece(Piece::King, color))
            .to_square()
            .unwrap_or_else(|err| {
                panic!("there should always exist a king with that color; something is wrong: {:?}", err)});// single bit

        self.is_square_attacked(ksq, !color)
    }

    pub fn attacked_squares_bb(&self, by: Color) -> Bitboard{
        let all_occ   = self.current.bitboards.all_occupancy;
        let by_slice  = self.current.bitboards.color_slice(by);

        let mut attacking_bb = Bitboard::new_empty();

        for (nr, bb) in by_slice.iter().enumerate(){
            let mut loop_bb = bb.clone();
            while let Some(board_idx) = loop_bb.pop_lsb(){
                attacking_bb |= get_attacks(Piece::try_from(nr as u8).expect("The index that was suposed to be piece number exceded it"), Square::from_idx(board_idx).expect("Board idx from pop_lsb gave an index out of reach"), all_occ, by);
            }
        }
        attacking_bb
        
    }


    #[inline]
    pub fn is_square_attacked(&self, sq: Square, by: Color) -> bool {
        // Pseudolegal (pins/king-safety ignored)
        let all_occ   = self.current.bitboards.all_occupancy;
        let by_slice  = self.current.bitboards.color_slice(by);

        let pawn_occ   = by_slice[Piece::Pawn.to_index()];
        let bishop_occ = by_slice[Piece::Bishop.to_index()];
        let knight_occ = by_slice[Piece::Knight.to_index()];
        let rook_occ   = by_slice[Piece::Rook.to_index()];
        let queen_occ  = by_slice[Piece::Queen.to_index()];
        let king_occ   = by_slice[Piece::King.to_index()];

        // Looks if attacks from the square can attack one of the opponents
        if attack::rook_attacks(sq, all_occ).intersects(rook_occ | queen_occ)   { return true; }
        if attack::bishop_attacks(sq, all_occ).intersects(bishop_occ | queen_occ){ return true; }


        if attack::knight_attacks(sq).intersects(knight_occ) { return true; }
        if attack::king_attacks(sq).intersects(king_occ)     { return true; }

        // Pawns: from target `sq`, attackers are in the opposite-direction mask => `!by`.

        if attack::pawn_attacks(sq, !by).intersects(pawn_occ) { return true; }

        false
    }
    
    #[inline]
    pub fn is_check_mate(&mut self) -> bool{

        self.in_check(self.current.side_to_move) && !self.can_move()
    }

    pub fn can_move(&mut self) -> bool{
        !MoveGen::legal_moves(self).is_empty()
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
            let moves = MoveGen::legal_moves(&mut position);
            let nr = rng.random_range(0..moves.size());
            position.make_move(moves.get(nr).unwrap());
            dbg!(position.current.bitboards.all_occupancy);
        }
    }
}

#[test]
fn test_attack(){

    let mut position = Position::new(Some("8/6k1/4p3/8/2b3Q1/2Kr4/8/8 w - - 0 1"));
    dbg!(&position);

    let b = position.is_square_attacked(Square::F5, Color::Black);
    dbg!(b);

}