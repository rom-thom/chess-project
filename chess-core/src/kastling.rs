



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


#[derive(Clone, Copy, PartialEq)]
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


 impl std::fmt::Debug for Castling{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output= String::new();
        if self.can_castle(CastlingSide::WK){
            output += "WK ";
        }
        if self.can_castle(CastlingSide::WQ){
            output += "WQ ";
        }
        if self.can_castle(CastlingSide::BK){
            output += "BK ";
        }
        if self.can_castle(CastlingSide::BQ){
            output += "BQ ";
        }
        write!(f, "{}", output)
    }
 }

