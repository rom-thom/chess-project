use std::ops::Not;
use std::str::FromStr;

use crate::board::{Bitboards, Bitboard};
use crate::moves::{BitMove, MoveList, MoveType};
use crate::piece::{Piece, PieceIndex};
use crate::square::{Square};
use crate::kastling::{Castling, CastlingSide, Imposter};
use crate::attack;
use crate::bitboard_consts::{self, CORNERS};



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {White, Black}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }
}
impl std::fmt::Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position{
    pub current: Snapshot,

    pub history: Vec<Snapshot>
}




impl Position {
    
    pub fn new(fen_string: Option<&str>) -> Self{
        match fen_string {
            Some(str_val) => return Self::read_fen(str_val),
            None => return Self::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        }
    }


    // For converting from a simple move input like from lichess "e4e5" or "e7e8r". // !!! this is not fast so don't use it on time-critical stuff.
    pub fn expand_move(&self, from: Square, to: Square, promo: Option<Piece>)->BitMove{
        let piece = self.current.bitboards.piece_on_square(from).expect("there should be a piece on the square that is moved from in the expand_move function, you twat");
        let mut move_type = match promo {
            Some(piece) => MoveType::Promotion(piece),
            None => MoveType::Quiet // Quiet for now
        };
        
        let mut is_capture =  to.to_bitboard().intersects(self.current.bitboards.all_occupancy);
        if piece.to_piece() == Piece::Pawn{
            if to.to_coord().1 != from.to_coord().1{ // den er ikkje på same colonne
                is_capture = true;
                if !self.current.bitboards.all_occupancy.intersects(to.to_bitboard()){
                    move_type = MoveType::EnPassant;
                }
            }
            if (to.to_coord().0 as isize - from.to_coord().0 as isize).abs() == 2 { // See if it is moved 2 squares forward to see if it makes an en passant oportunity
                move_type = MoveType::EnPassant;
            }
        }
        if piece.to_piece() == Piece::King{
            let from_col = from.to_coord().1 as isize;
            let to_col = to.to_coord().1 as isize;
            if (to_col - from_col).abs() == 2{
                // Determine direction
                if to_col > from_col {
                    move_type = MoveType::Castling(Imposter::King);
                } else {
                    move_type = MoveType::Castling(Imposter::Queen);
                }
            }
        }

        BitMove::new(from, to, is_capture, move_type)
    }

    // returns the move from 
    pub fn movestring_to_bit_move(moves_str: &str)->Result<BitMove, String>{
        // This finds the last move as that is all we need for the engine, and if the string is weird, it removes mitaken extra spaces
        let last_move = moves_str.split(" ")
                                       .filter(|s| !s.is_empty())
                                       .last()
                                       .ok_or("No valid move was found in the moves_string when converting movestring to bitmove")?;

        let chars: Vec<char> = last_move.chars().collect();

        let start_square_str = chars[0].to_string() + &chars[1].to_string();
        let end_square_str = chars[2].to_string() +  &chars[3].to_string();
        let start_square = Square::from_str(&start_square_str)?;
        let end_square = Square::from_str(&end_square_str)?;

        let mut promotion_piece = None;

        if chars.len() == 5{
            promotion_piece = Some(Piece::from_char(chars[4]));
            
        };
        Self::expand_move(&self, start_square, end_square, promotion_piece)
}
}






#[derive(Copy, Clone, PartialEq)]
pub struct Snapshot {
    pub bitboards: Bitboards,
    pub side_to_move: Color,            
    pub castling: Castling,
    pub en_passant: Option<Square>,  
    pub halfmove_clock: u16,             
    pub fullmove_number: u16,
    //zobrist_key:     u64,  // TODO look up and make this later (I'm in neeed for speeed)
}









impl std::fmt::Debug for Snapshot{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        writeln!(f)?;

        for row in (0..8).rev() {
            write!(f, "{}: ", row+1)?;

            for col in 0..8 {
                let idx:u8 = row*8+col;
                let temp_square = (1u64<<idx).into();
                if self.bitboards.all_occupancy.intersects(temp_square){
                    // Find what type ocupies that square
                    for (piece_type_index, piece_type_board) in self.bitboards.boards.iter().enumerate(){
                        if piece_type_board.intersects(temp_square){
                            // for this to work the boards indexing must match the Piece indexing
                            let piece_type = PieceIndex::from_index(piece_type_index).expect("could not convert from the index looped trough to PieceIndex, when looping trough the differente boards in the position debuging");
                            write!(f, "{} ", piece_type.to_fen_char().to_string())?;
                        }
                    }
                }
                else{
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "   A B C D E F G H")?;
        writeln!(f)?;
        writeln!(f, "Moving color: {}", self.side_to_move)?;
        writeln!(f, "Sides that can castle: {:?}", self.castling)?; // TODO implement Debug for castling
        match self.en_passant {
            None => writeln!(f, "En-passant square: None")?,
            Some(sqr) => writeln!(f, "En-passant square: {}", sqr.square_str())?
        };
        writeln!(f, "Halfmove clock: {}", self.halfmove_clock)?;
        writeln!(f, "Fullmove number: {}", self.fullmove_number)?;
        Ok(())
    }

}
