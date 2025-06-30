use crate::position::Position;
use crate::{piece::PieceIndex, position::Color};
use crate::square::Square;
use std::{fmt::Debug, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not}};


pub const ROWS: usize = 8;
pub const COLS: usize = 8;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboards {
    pub boards: [Bitboard; 12],

    pub white_occupancy: Bitboard,
    pub black_occupancy: Bitboard,
    pub all_occupancy: Bitboard

}



impl Bitboards{
    #[inline]
    pub fn new_empty() -> Self{
        Bitboards { boards: [Bitboard(0);12], white_occupancy: Bitboard(0), black_occupancy: Bitboard(0), all_occupancy: Bitboard(0)}
    }

    // Get access to a bitboard (non mutable tho)
    #[inline]
    pub fn get_bitboard(&self, piece: PieceIndex) -> Bitboard{
        self.boards[piece as usize]
    }

    // Get access to a bitboard (mutable edition)
    #[inline]
    pub fn get_bitboard_mut(&mut self, piece: PieceIndex) -> &mut Bitboard{
        &mut self.boards[piece as usize]
    }

    #[inline]
    pub fn set(&mut self, piece: PieceIndex, square: Square){
        self.boards[piece as usize] |= square.to_bitboard();
        match piece.color() {
            Color::Black => self.black_occupancy |= square.to_bitboard(),
            Color::White => self.white_occupancy |= square.to_bitboard()
        }
        self.all_occupancy   |= square.to_bitboard();
    }

    pub fn remove(&mut self, piece: PieceIndex, square: Square){
        let all_but_square_mask = !(square.to_bitboard());
        self.boards[piece.index()] &= all_but_square_mask;
        self.all_occupancy &= all_but_square_mask;
        self.black_occupancy &= all_but_square_mask; // eg kan gjer dette med både kvit og svart da det alltid bare er ein av dei som kan vere 1
        self.white_occupancy &= all_but_square_mask;
    }

    // This is probably an expensive function, so don't use this to much
    pub fn uppdate_occupancy(&mut self){
        for (piece_nr, piece) in self.boards.into_iter().enumerate(){
            self.all_occupancy |= piece;

            if piece_nr < 6{
                self.white_occupancy |= piece;
            }
            else{
                self.black_occupancy |= piece;
            }
        }
    }
 


    #[inline] // TODO this can be sped up by using mailbox: [Option<PieceIndex>; 64], which allways has the pieces for every square stored
    pub fn piece_on_square(&self, square: Square) -> Option<PieceIndex>{

        for piece in 0..12{
            if self.boards[piece].intersects(square.index().into()){
                return Some(PieceIndex::try_from(piece).expect("cannot convert from piece to pieceindex in piece on square"));
            }
        }
        None
    }



}






#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitboard(u64);


impl Bitboard {
    #[inline]
    pub fn new_empty() -> Self{
        Bitboard(0)
    }


    #[inline]
    pub fn intersects(self, other: Self)-> bool{
        (self & other).0 != 0
    }

    #[inline]
    pub fn set(&mut self, square_index: u8){
        self.0 |= 1<<square_index;
    }

    #[inline]
    pub fn remove(&mut self, square_index: u8){
        self.0 &= !(1<<square_index);
    }


    #[inline]
    pub fn coord_to_index(row: usize, col: usize) -> u8 {
        ((row)*8 + col) as u8
    }
    #[inline]
    pub fn index_to_coord(index: u8) -> (usize, usize) {
        ((index / COLS as u8) as usize, (index%COLS as u8) as usize)
    }

    #[inline]
    pub fn is_occupied(&self, square: Square) -> bool {
        self.intersects((square as u8).into())
    }


    // Shift
    const EVERY_COL_BUT_A: u64 = 0xFE_FE_FE_FE_FE_FE_FE_FE; // To make shure it doesn't generate moves that leves the board and comes back on the other side
    const EVERY_COL_BUT_H: u64 = 0x7F_7F_7F_7F_7F_7F_7F_7F;
    #[inline] pub fn shift_right(&mut self){self.0  = (self.0 & Self::EVERY_COL_BUT_H) << 1;}
    #[inline] pub fn shift_left(&mut self){self.0  = (self.0 & Self::EVERY_COL_BUT_A) >> 1;}
    #[inline] pub fn shift_up(&mut self){self.0 = self.0 << 8;}
    #[inline] pub fn shift_down(&mut self){self.0  = self.0 >> 8;}
    #[inline] pub fn shift_upp_right(&mut self){self.shift_up();self.shift_right();}
    #[inline] pub fn shift_upp_left(&mut self){self.shift_up();self.shift_left();}
    #[inline] pub fn shift_down_right(&mut self){self.shift_down();self.shift_right();}
    #[inline] pub fn shift_down_left(&mut self){self.shift_down();self.shift_left();}


    // Move generation 

    #[inline] 
    pub fn pop_lsb(&mut self)-> Option<u8>{ // returns index of lsb (least significant bit)
        if self.0==0{
            return None;
        }
        let lsb: u32 = self.0.trailing_zeros(); // this is the amount of zeros behind the first 1
        self.0 &= self.0 - 1; // This is cooool, and works because: 
                                        // bb:           10110000
                                        // bb - 1:       10101111 
                                        // bb & (bb-1) = 10100000 which is bb without lsb
        Some(lsb as u8)
    }



    // i want to be able to make const bitboards for masking sertain squares
    pub const fn new_const(val: u64)-> Self{
        Self(val)
    }

}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}
impl From<u8> for Bitboard {
    fn from(value: u8) -> Self {
        assert!(value < 64, "Bit position must be less than 64");
        Bitboard(1u64<<value)
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0|rhs.0;
    }
}
impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0&rhs.0)
    }
}
impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0&rhs.0;
    }
}
impl Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        Self::from(!self.0)
    }
}
impl Debug for Bitboard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        writeln!(f)?;

        for row in (0..8).rev() {
            write!(f, "{}: ", row+1)?;

            for col in 0..8 {
                let idx:u8 = row*8+col;
                if self.intersects((1u64<<idx).into()){
                    write!(f, "1 ")?;
                }
                else{
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "   A B C D E F G H")?;
        Ok(())
    }

}













//             Board Indexation
//      ┌───┬───┬───┬───┬───┬───┬───┬───┐              1D array view:
//    8 │ 56│ 57│ 58│ 59│ 60│ 61│ 62│ 63│              [
//      ├───┼───┼───┼───┼───┼───┼───┼───┤                0,  1,  2,  3,  4,  5,  6,  7,
//    7 │ 48│ 49│ 50│ 51│ 52│ 53│ 54│ 55│                8,  9, 10, 11, 12, 13, 14, 15,
//      ├───┼───┼───┼───┼───┼───┼───┼───┤               16, 17, 18, 19, 20, 21, 22, 23,
//    6 │ 40│ 41│ 42│ 43│ 44│ 45│ 46│ 47│               24, 25, 26, 27, 28, 29, 30, 31,
//      ├───┼───┼───┼───┼───┼───┼───┼───┤               32, 33, 34, 35, 36, 37, 38, 39,
//    5 │ 32│ 33│ 34│ 35│ 36│ 37│ 38│ 39│               40, 41, 42, 43, 44, 45, 46, 47,
//      ├───┼───┼───┼───┼───┼───┼───┼───┤               48, 49, 50, 51, 52, 53, 54, 55,
//    4 │ 24│ 25│ 26│ 27│ 28│ 29│ 30│ 31│               56, 57, 58, 59, 60, 61, 62, 63
//      ├───┼───┼───┼───┼───┼───┼───┼───┤              ]
//    3 │ 16│ 17│ 18│ 19│ 20│ 21│ 22│ 23│
//      ├───┼───┼───┼───┼───┼───┼───┼───┤
//    2 │  8│  9│ 10│ 11│ 12│ 13│ 14│ 15│
//      ├───┼───┼───┼───┼───┼───┼───┼───┤
//    1 │  0│  1│  2│  3│  4│  5│  6│  7│
//      └───┴───┴───┴───┴───┴───┴───┴───┘
//        A   B   C   D   E   F   G   H
//
// Example: (row, col) = (3, 2) = index 26 = C4





#[test]
fn test_bitboard(){
    let pos = Position::new(None);
    dbg!(pos);
}   