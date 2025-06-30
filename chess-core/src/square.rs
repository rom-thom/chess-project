use num_enum::TryFromPrimitive;
use crate::{board::Bitboard, position::Color};


#[derive(Copy, Clone, Debug, PartialEq, Default, TryFromPrimitive)]
#[repr(u8)]
pub enum Square {
    #[default] // For fast iteration of not needed ellements
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8, // = 63
}

impl std::str::FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A1" => Ok(Square::A1), "B1" => Ok(Square::B1), "C1" => Ok(Square::C1), "D1" => Ok(Square::D1),
            "E1" => Ok(Square::E1), "F1" => Ok(Square::F1), "G1" => Ok(Square::G1), "H1" => Ok(Square::H1),
            "A2" => Ok(Square::A2), "B2" => Ok(Square::B2), "C2" => Ok(Square::C2), "D2" => Ok(Square::D2),
            "E2" => Ok(Square::E2), "F2" => Ok(Square::F2), "G2" => Ok(Square::G2), "H2" => Ok(Square::H2),
            "A3" => Ok(Square::A3), "B3" => Ok(Square::B3), "C3" => Ok(Square::C3), "D3" => Ok(Square::D3),
            "E3" => Ok(Square::E3), "F3" => Ok(Square::F3), "G3" => Ok(Square::G3), "H3" => Ok(Square::H3),
            "A4" => Ok(Square::A4), "B4" => Ok(Square::B4), "C4" => Ok(Square::C4), "D4" => Ok(Square::D4),
            "E4" => Ok(Square::E4), "F4" => Ok(Square::F4), "G4" => Ok(Square::G4), "H4" => Ok(Square::H4),
            "A5" => Ok(Square::A5), "B5" => Ok(Square::B5), "C5" => Ok(Square::C5), "D5" => Ok(Square::D5),
            "E5" => Ok(Square::E5), "F5" => Ok(Square::F5), "G5" => Ok(Square::G5), "H5" => Ok(Square::H5),
            "A6" => Ok(Square::A6), "B6" => Ok(Square::B6), "C6" => Ok(Square::C6), "D6" => Ok(Square::D6),
            "E6" => Ok(Square::E6), "F6" => Ok(Square::F6), "G6" => Ok(Square::G6), "H6" => Ok(Square::H6),
            "A7" => Ok(Square::A7), "B7" => Ok(Square::B7), "C7" => Ok(Square::C7), "D7" => Ok(Square::D7),
            "E7" => Ok(Square::E7), "F7" => Ok(Square::F7), "G7" => Ok(Square::G7), "H7" => Ok(Square::H7),
            "A8" => Ok(Square::A8), "B8" => Ok(Square::B8), "C8" => Ok(Square::C8), "D8" => Ok(Square::D8),
            "E8" => Ok(Square::E8), "F8" => Ok(Square::F8), "G8" => Ok(Square::G8), "H8" => Ok(Square::H8),
            _ => Err(format!("Invalid square convertion from: {}", s)),
        }
    }

}



impl Square {

    pub fn square_str(self) -> String{
        let square_str = match self {
            Square::A1 => "a1", Square::B1 => "b1", Square::C1 => "c1", Square::D1 => "d1",
            Square::E1 => "e1", Square::F1 => "f1", Square::G1 => "g1", Square::H1 => "h1",
            Square::A2 => "a2", Square::B2 => "b2", Square::C2 => "c2", Square::D2 => "d2",
            Square::E2 => "e2", Square::F2 => "f2", Square::G2 => "g2", Square::H2 => "h2",
            Square::A3 => "a3", Square::B3 => "b3", Square::C3 => "c3", Square::D3 => "d3",
            Square::E3 => "e3", Square::F3 => "f3", Square::G3 => "g3", Square::H3 => "h3",
            Square::A4 => "a4", Square::B4 => "b4", Square::C4 => "c4", Square::D4 => "d4",
            Square::E4 => "e4", Square::F4 => "f4", Square::G4 => "g4", Square::H4 => "h4",
            Square::A5 => "a5", Square::B5 => "b5", Square::C5 => "c5", Square::D5 => "d5",
            Square::E5 => "e5", Square::F5 => "f5", Square::G5 => "g5", Square::H5 => "h5",
            Square::A6 => "a6", Square::B6 => "b6", Square::C6 => "c6", Square::D6 => "d6",
            Square::E6 => "e6", Square::F6 => "f6", Square::G6 => "g6", Square::H6 => "h6",
            Square::A7 => "a7", Square::B7 => "b7", Square::C7 => "c7", Square::D7 => "d7",
            Square::E7 => "e7", Square::F7 => "f7", Square::G7 => "g7", Square::H7 => "h7",
            Square::A8 => "a8", Square::B8 => "b8", Square::C8 => "c8", Square::D8 => "d8",
            Square::E8 => "e8", Square::F8 => "f8", Square::G8 => "g8", Square::H8 => "h8",
        };
        square_str.to_string()
        
    }

    /// Konverterar (rad, colonne) til Square.
    /// (0, 0) = A1, (0, 1) = B1, ..., (7, 7) = H8
    pub fn from_coords(row: usize, col: usize) -> Option<Square> {
        if row < 8 && col < 8 {
            let idx = row * 8 + col;
            Some(unsafe { std::mem::transmute(idx as u8) })
        } else {
            None
        }
    }

    pub fn from_idx(idx: u8) -> Option<Square> {
        if idx < 64 {
            Some(unsafe { std::mem::transmute(idx as u8) })
        } else {
            None
        }
    }

    #[inline]
    pub fn index(self)->u8{
        self as u8
    }

    #[inline]
    pub fn to_coord(&self)->(usize, usize){
        Bitboard::index_to_coord(self.index())
    }

    #[inline]
    pub fn to_bitboard(&self) -> Bitboard{
        Bitboard::from(self.index())
    }

    

}





#[test]
fn test_square(){
    dbg!(Square::A8 as u8);
}