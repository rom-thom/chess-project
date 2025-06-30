

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CastlingSide{
    WK = 1, // Kvit Konge
    WQ = 1 << 1, // Kvit Dronning
    BK = 1 << 2, // Svart Konge
    BQ = 1 << 3, // Svart Dronning
}



// side to move not included color
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Imposter{
King,
Queen
}

impl Imposter {
    #[inline]
    pub fn from_castling_side(castling_side: CastlingSide)->Self{
        match castling_side {
            CastlingSide::BK | CastlingSide::WK => Self::King,
            CastlingSide::BQ | CastlingSide::WQ => Self::Queen
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Castling{
    pub rights: u8,
}
 impl Castling{
    pub fn new()->Self{
        Self { rights: 0 }
    }
    pub fn add_castle_right(&mut self, castling_side: CastlingSide){
        self.rights |= castling_side as u8
    }
    pub fn remove_castling_right(&mut self, castling_side: CastlingSide){
        self.rights &= !(castling_side as u8)
    }
    pub fn can_castle(&self, castling_side: CastlingSide) -> bool{
        (self.rights & castling_side as u8) == castling_side as u8
    }
 }


 


















 #[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_board(){
        let mut castling = Castling::new();

        println!("After new: {:b}", castling.rights);
        
        castling.add_castle_right(CastlingSide::BQ);
        castling.add_castle_right(CastlingSide::WQ);
        println!("After adding: {:b}", castling.rights);

        assert!(castling.can_castle(CastlingSide::BQ));
        assert!(!castling.can_castle(CastlingSide::WK));
    }
}