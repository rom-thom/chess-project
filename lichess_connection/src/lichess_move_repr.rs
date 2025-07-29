use std::str::FromStr;

use chess_core::moves::{self, BitMove};
use chess_core::moves::Move;
use chess_core::piece;
use chess_core::square::Square;




// returns the move from 
pub fn read_lichess_move(moves_str: &str)->Option<moves::BitMove>{
    // This finds the last move as that is all we need for the engine, and if the string is weird, it removes mitaken extra spaces (chat said i should do that)
    let last_move = moves_str.split(" ").filter(|s| !s.is_empty()).last()?;
    let chars: Vec<char> = last_move.chars().collect();

    let start_square_str = chars[0].to_string() + &chars[1].to_string();
    let end_square_str = chars[2].to_string() +  &chars[3].to_string();
    let start_square = Square::from_str(&start_square_str);
    let end_square = Square::from_str(&end_square_str);

    let is_promotion = chars.len() == 5;
    let mut promotion_piece = None;

    if is_promotion{
        promotion_piece = Some(chars[4]);
        
    }

    // BitMove::new(start_square, end_square, is_capture, move_type)
    todo!("find out wat move liches wrote for me");
}