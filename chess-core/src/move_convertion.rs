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

    // returns the move from  // ? it is very slow so dont use it for timecritical things
    pub fn stringmove_to_bit_move(&self, moves_str: &str)->Result<BitMove, String>{
        // This finds the last move as that is all we need for the engine, and if the string is weird, it removes mitaken extra spaces
        let last_move = moves_str.split(" ")
                                       .filter(|s| !s.is_empty())
                                       .last()
                                       .ok_or("No valid move was found in the moves_string when converting movestring to bitmove")?;

        let chars: Vec<char> = last_move.chars().collect();
        if chars.len() < 4{return Err("the last move was to short to have first and last move".to_string())}
        if chars.len() > 5{return Err("the last move was to long to only have first and last move and promotion".to_string())}

        let start_square_str = chars[0].to_string() + &chars[1].to_string();
        let end_square_str = chars[2].to_string() +  &chars[3].to_string();
        let start_square = (&start_square_str).parse()?;
        let end_square = (&end_square_str).parse()?;

        let mut promotion_piece = None;

        if chars.len() == 5{
            promotion_piece = Some(Piece::from_char(chars[4])
                                         .ok_or(format!("The end of the last move was not convertable to a promotion piece. This is the move you gave ({})", chars.iter().collect::<String>()))?);
        };
        let candidate_move = self.expand_move(start_square, end_square, promotion_piece);


        // Compares to all the legal moves to see if it exist there
        let is_legal = {
            let mut lm = MoveList::new_empty(); // ? This is realy slow
            self.fill_legal(&mut lm);
            lm.iter().any(|m|(*m) == candidate_move)
        };

        if !is_legal{
            return Err(String::from("That is an ilegal move"));
        }


        Ok(self.expand_move(start_square, end_square, promotion_piece))
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
    let mut position = Position::new(None);
    let bitmove = position.stringmove_to_bit_move("2 3  s e2e4").unwrap();
    dbg!(bitmove);
    dbg!(bitmove.to_string());

}