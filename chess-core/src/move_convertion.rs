use crate::movegen::MoveGen;
use crate::{kastling::Imposter, piece::Piece, position::Position};
use crate::moves::{BitMove, Move, MoveList, MoveType};
use crate::square::{Square};




impl Position{

    // For converting from a simple move input like from lichess "e4e5" or "e7e8r"
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

  

}
impl BitMove {
    pub fn to_string(&self) -> String{
        let from = self.get_start_square().square_str();
        let to = self.get_end_square().square_str();
        let mut stringmove = from + &to;
        match self.get_premotion_piece() {
            None => (),
            Some(promo_piece) => {stringmove.push(promo_piece.to_char())}
        }
        stringmove
    }
}






#[test]
fn test_position(){
    let pos = Position::new(None);
    let bitmove = MoveGen::stringmove_to_bitmove(&pos, "2 3  s e2e4").unwrap();
    dbg!(bitmove);
    dbg!(bitmove.to_string());

}