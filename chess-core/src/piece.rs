use num_enum::TryFromPrimitive;

use crate::position::Color;


#[repr(u8)]
#[derive(Copy, Clone, Debug, TryFromPrimitive, Default, PartialEq, Eq)]
pub enum Piece {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
impl Piece{
    pub fn from_piece_index(piece_index: &PieceIndex) -> Self{
        match piece_index {
            PieceIndex::BlackPawn | PieceIndex::WhitePawn => Self::Pawn,
            PieceIndex::BlackBishop | PieceIndex::WhiteBishop => Self::Bishop,
            PieceIndex::BlackKnight | PieceIndex::WhiteKnight => Self::Knight,
            PieceIndex::BlackRook | PieceIndex::WhiteRook => Self::Rook,
            PieceIndex::BlackQueen | PieceIndex::WhiteQueen => Self::Queen,
            PieceIndex::BlackKing | PieceIndex::WhiteKing => Self::King

        }
    }
}



#[repr(usize)]
#[derive(Copy, Clone, Debug, TryFromPrimitive, Default, PartialEq, Eq)]
pub enum PieceIndex {
    #[default]
    WhitePawn   = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook   = 3,
    WhiteQueen  = 4,
    WhiteKing   = 5,

    BlackPawn   = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook   = 9,
    BlackQueen  = 10,
    BlackKing   = 11,
}


impl PieceIndex {
    pub fn to_fen_char(self) -> char {
        match self {
            PieceIndex::WhitePawn   => 'P',
            PieceIndex::WhiteKnight => 'N',
            PieceIndex::WhiteBishop => 'B',
            PieceIndex::WhiteRook   => 'R',
            PieceIndex::WhiteQueen  => 'Q',
            PieceIndex::WhiteKing   => 'K',
            PieceIndex::BlackPawn   => 'p',
            PieceIndex::BlackKnight => 'n',
            PieceIndex::BlackBishop => 'b',
            PieceIndex::BlackRook   => 'r',
            PieceIndex::BlackQueen  => 'q',
            PieceIndex::BlackKing   => 'k',
        }
    }
    pub fn from_fen_char(c: char) -> Option<PieceIndex> {
        Some(match c {
            'P' => PieceIndex::WhitePawn,
            'N' => PieceIndex::WhiteKnight,
            'B' => PieceIndex::WhiteBishop,
            'R' => PieceIndex::WhiteRook,
            'Q' => PieceIndex::WhiteQueen,
            'K' => PieceIndex::WhiteKing,
            'p' => PieceIndex::BlackPawn,
            'n' => PieceIndex::BlackKnight,
            'b' => PieceIndex::BlackBishop,
            'r' => PieceIndex::BlackRook,
            'q' => PieceIndex::BlackQueen,
            'k' => PieceIndex::BlackKing,
             _  => return None,
        })
    }

    #[inline] pub fn index(self)-> usize{
        self as usize
    }

    
    // This relies on Piece and PieceIndex being the same "rekjefølge"
    #[inline] pub fn from_piece(piece: Piece, color: Color) -> Self {
        let base = piece as usize;
        let offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };
        PieceIndex::try_from_primitive(base + offset).expect("Invalid piece/color combination")
    }
    
    pub fn to_piece(&self)->Piece{
        Piece::from_piece_index(self)
    }

    pub fn color(&self) -> Color {
        if (*self as usize) < PieceIndex::BlackPawn as usize {
            Color::White
        } else {
            Color::Black
        }  
    }

}

