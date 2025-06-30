use std::ops::Not;

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

    pub fn push(&mut self) {
        self.history.push(self.current.clone());
    }

    pub fn pop(&mut self){
        self.history.pop().expect("You can't remove element from position history (it is probably empty)");
    }


    
}






#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub bitboards: Bitboards,
    pub side_to_move: Color,            
    pub castling: Castling,
    pub en_passant: Option<Square>,  
    pub halfmove_clock: u16,             
    pub fullmove_number: u16,
    //zobrist_key:     u64,  // TODO look up and make this later (I'm in neeed for speeed)
}




