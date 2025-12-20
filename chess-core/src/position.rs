use std::ops::Not;
use std::str::FromStr;

use crate::board::{Bitboards, Bitboard};
use crate::moves::{BitMove, Move, MoveList, MoveType};
use crate::piece::{Piece, PieceIndex};
use crate::square::{Square};
use crate::kastling::{Castling, CastlingSide, Imposter};
use crate::{attack, position};
use crate::bitboard_consts::{self, CORNERS};
pub use crate::zobrist::ZobristKey;



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

#[derive(Debug, PartialEq)] // Do not derive Clone, as i dont want to do that in a time critical way
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

}




#[derive(Copy, Clone, PartialEq)]
pub struct Snapshot {
    pub bitboards: Bitboards,
    pub side_to_move: Color,            
    pub castling: Castling,
    pub en_passant: Option<Square>,  
    pub halfmove_clock: u16,             
    pub fullmove_number: u16,
    pub zobrist_key: ZobristKey,
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


