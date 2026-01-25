use std::char::MAX;
use std::env::consts;
use std::fmt::{Debug, DebugList};
use std::fs::OpenOptions;
use std::{iter, result};

use crate::kastling::{Castling, CastlingSide, Imposter};
use crate::movegen::MoveGen;
use crate::position::Position;
use crate::square::Square;
use crate::board::{Bitboards};
use crate::piece::{PieceIndex, Piece};


// masks for accessing parts of the BitMove
const FROM_SHIFT: u16 = 10;
const TO_SHIFT:   u16 = 4;
const CAPTURE_BIT: u16 = 1 << 3;
const FLAG_MASK:  u16 = 0b111;



#[derive(Clone, Copy, Default, PartialEq)]
pub struct BitMove(u16);

// BitMove representation: FFFFFFTTTTTTCMMM  F = From, T = To, C=Capture, M = Move-flags(Quiet, (doubepawn push and en_passant in one), Promo*4, kastle * 2)
impl BitMove{
    
    #[inline]
    pub fn new(from: Square, to: Square, is_capture: bool, move_type: MoveType)-> Self{
        BitMove::encode(Move{from, to, is_capture, move_type})
    }


    pub fn get_start_square(&self)->Square{
        Square::from_idx((self.0 >> FROM_SHIFT) as u8).expect("get_start_square (BitMove) finds an un squarable index from:")
    }
    pub fn get_end_square(&self)->Square{
        Square::from_idx(((self.0 >> TO_SHIFT) & 0b111111) as u8).expect("get end square (Bitmove) finds an un squarable index to:")
    }
    pub fn get_piece(&self, boards_before_move: &Bitboards)->PieceIndex{
        boards_before_move.piece_on_square(self.get_start_square()).unwrap_or_else(|| panic!("found no piece on square that the piece moved from. Here is the all ocupancy board before move: {:?}, and here is the move: {:?}", boards_before_move.all_occupancy, Move::from(*self)))
    }
    pub fn is_capture(&self)->bool{
        (self.0 & CAPTURE_BIT) != 0
    }
    pub fn is_quiet(&self)->bool{
        self.0 & 0b111 == 0
    }
    pub fn is_double_pawn_push(&self)->bool{
        self.0 & 0b111 == 1 && !self.is_capture()
    }
    pub fn is_en_passant(&self)->bool{
        self.0 & 0b111 == 1 && self.is_capture()
    }
    pub fn get_castle_side(&self) -> Option<Imposter> {
        match self.0 & 0b111 {
            2 => Some(Imposter::King),
            3 => Some(Imposter::Queen),
            _ => None
        }
    }
    pub fn get_premotion_piece(&self)->Option<Piece>{
        match self.0 & 0b111 {
            4 => Some(Piece::Queen),
            5 => Some(Piece::Rook),
            6 => Some(Piece::Knight),
            7 => Some(Piece::Bishop),
            _ => None
        }
    }


}




// Human readable version of BitMove
#[derive(Clone, Copy, Debug, Default)]
pub struct Move{
    from: Square,
    to: Square,
    is_capture: bool,
    move_type: MoveType,
}






// List of current posible moves
#[derive(Debug, Clone, Copy)]
pub struct MoveList{
    moves: [BitMove; 256],
    len: usize
}

impl MoveList{
    pub fn new_empty()-> Self{
        MoveList { 
            moves: [BitMove::default(); 256],
            len: 0 
        }
    }

    // To looptrough it (only the once that is important)
    pub fn iter(&self) -> std::slice::Iter<'_, BitMove> {
        self.moves[0..self.size()].iter()
    }
    pub fn dbg(&self){
        for mov in self.iter(){
            dbg!(mov);
        }
    }

    pub fn as_slice(&self) -> &[BitMove] { &self.moves[..self.len] }


    #[inline]
    pub fn add(&mut self, mov: BitMove){
        if self.len < 256{
            self.moves[self.len] = mov;
            self.len += 1;
        }
        else{panic!("Nooo, i can't add more stuf to the Movelist. Somehow i added ilegaly many moves")} 
    }

    #[inline]
    pub fn get(&self, idx: usize)-> Option<&BitMove>{
        if idx < self.len{
            return Some(&self.moves[idx])
        }
        None
    }

    #[inline]
    pub fn size(&self) -> usize{
        self.len
    }

    #[inline]
    pub fn clear(&mut self){
        self.len = 0
    }

    #[inline]
    pub fn is_empty(&self) -> bool{
        self.len == 0
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MoveType {
    #[default]
    Quiet,
    Promotion(Piece),
    EnPassant, // this also covers making an en passant aka going to squares forword
    Castling(Imposter)

}


impl BitMove{
    // Make a 16 bit u16 number representing a move
    #[inline] fn encode(mov: Move)->Self{
        let move_type_bits: u16 = match mov.move_type {
            MoveType::Quiet => 0,
            MoveType::EnPassant => 1,
            MoveType::Castling(side) => {
                match side {
                    Imposter::King => 2,
                    Imposter::Queen => 3,
                }
            }
            MoveType::Promotion(piece) =>{
                match piece {
                    Piece::Queen => 4,
                    Piece::Rook => 5,
                    Piece::Knight => 6,
                    Piece::Bishop => 7,
                    _ => panic!("Piece can not be premoted to {:?}", &piece)
                }
            }
        };

        let capture_bit = if mov.is_capture {CAPTURE_BIT} else {0};


        let from_bits = (mov.from.index() as u16) << FROM_SHIFT;

        let to_bits = (mov.to.index() as u16) << TO_SHIFT;

        Self(move_type_bits | capture_bit | from_bits | to_bits)
    }
    // FFFFFFTTTTTTCMMM
    fn decode(bit_move: BitMove) -> Move{
        let from = Square::from_idx((bit_move.0 >> FROM_SHIFT) as u8).expect("decode move finds an un squarable index from:");
        let to = Square::from_idx(((bit_move.0 >> TO_SHIFT) & 0b111111) as u8).expect("decode move finds an un squarable index to:");
        let is_capture = (bit_move.0 & CAPTURE_BIT) != 0;
        let move_flag = match bit_move.0 & 0b111 {
            0 => MoveType::Quiet,
            1 => MoveType::EnPassant,
            2 => MoveType::Castling(Imposter::King),
            3 => MoveType::Castling(Imposter::Queen),
            4 => MoveType::Promotion(Piece::Queen),
            5 => MoveType::Promotion(Piece::Rook),
            6 => MoveType::Promotion(Piece::Knight),
            7 => MoveType::Promotion(Piece::Bishop),
            _ => panic!("math ein't mathing in move decoding")
        };
        Move {from: from, to: to, is_capture: is_capture, move_type: move_flag}

    }
}



impl From<Move> for BitMove{
    fn from(value: Move) -> Self {
        BitMove::encode(value)
    }
}
impl From<BitMove> for Move{
    fn from(value: BitMove) -> Self {
        BitMove::decode(value)
    }
}


impl Debug for BitMove{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let move_version = Move::from(*self);
        move_version.fmt(f)
    }
}


const MAX_DEPTH: usize = 128;
pub struct MovePath{
    moves: [BitMove; MAX_DEPTH],
    len: usize
}

impl Default for MovePath {
    fn default() -> Self {
        Self {
            moves: [BitMove::default(); MAX_DEPTH],
            len: 0,
        }
    }
}


impl MovePath{
    pub fn new_empty()-> Self{
        Self { moves: [BitMove::default(); MAX_DEPTH], len: 0 }
    }
    
    pub fn len(&self)->usize{
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool{
        self.len == 0
    }
    #[inline]
    pub fn is_full(&self) -> bool{
        self.len >= MAX_DEPTH
    }
    pub const fn capacity(&self) -> usize{
        MAX_DEPTH
    }

    pub fn get(&self, idx: usize) -> Option<BitMove>{
        if idx >= self.len{
            return None
        }
        Some(self.moves[idx])
    }


    pub fn push(&mut self, mov: BitMove){ // false == full
        debug_assert!(self.is_full(), "MovePath overflow, tried to push to full movepath");

        self.moves[self.len] = mov;
        self.len += 1;
    }

    pub fn append(&mut self, other: &mut Self){ 
        let other_len = other.len();

        debug_assert!(self.len + other_len >= MAX_DEPTH, "MovePath overflow");

        // Copy the other's moves into our storage at the current end
        if !other.is_empty() {
            let start = self.len;
            let end = start + other_len;
            self.moves[start..end].copy_from_slice(&other.moves[..other_len]);
            self.len = end;
        }

    }

    pub fn pop(&mut self) -> Option<BitMove>{
        if self.is_empty(){
            return None
        }
        self.len -= 1;
        Some(self.moves[self.len])
    }
    pub fn clear(&mut self){
        self.len = 0
    }
    pub fn last(&self)->Option<BitMove>{
        if self.is_empty(){
            return None
        }
        Some(self.moves[self.len-1])
    }

}




impl Debug for MovePath{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move path: ")?;
        if self.len() == 0{
            write!(f, "No move")?;
        }
        for idx in 0..self.len(){
            write!(f, "  {}  ", self.get(idx).expect("I move trough moves in the length of the path, so this should be legal").to_string())?;
        }
        Ok(())
    }
}





#[test]
fn test_moves(){
    let mut pos = Position::new(None);
    let legal_moves = MoveGen::legal_moves(&mut pos);
    let mut move_path = MovePath::new_empty();
    move_path.push(*legal_moves.get(2).unwrap());
    

    dbg!(move_path);
    

}