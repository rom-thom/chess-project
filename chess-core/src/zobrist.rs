use crate::position::{Position, ZobristKey};



impl Position{
    
    pub fn key(&self) -> ZobristKey{
        self.current.zobrist_key
    }
}


impl ZobristTable{

    


}

