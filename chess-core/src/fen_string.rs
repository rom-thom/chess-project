use crate::position::{Position, Color, Snapshot};

use std::str::FromStr;

use crate::square::{Square};
use crate::board::{Bitboards, Bitboard};
use crate::piece::PieceIndex;
use crate::kastling::{Castling, CastlingSide};

impl Position{

    pub fn read_fen(fen_string: &str) -> Self {
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"   this is the starting fen string
        let mut board = Bitboards::new_empty();
        let mut side_to_move = Color::White;
        let mut castling = Castling::new();
        let mut en_passant = Some(Square::A1);
        let mut halfmove_clock = 0;
        let mut fullmove_clock = 0;

        for (index, info) in fen_string.split_whitespace().enumerate(){
            match index {
                0 => { // handtere brett
                    let rows = info.split('/');
                    for (row_nr, row) in rows.enumerate(){
                        let square_chars: Vec<char> = row.chars().collect();

                        let mut str_idx: usize = 0;
                        let mut col_nr: usize = 0;
                        
                        'rowy_loopy: while str_idx < square_chars.len() {


                            if square_chars[str_idx].is_ascii_digit(){
                                col_nr += square_chars[str_idx].to_digit(10).expect("read_fen: check said it should be a digit") as usize;
                                str_idx += 1;
                                continue 'rowy_loopy;
                            }
                            else{
                                let piece_type = PieceIndex::from_fen_char(square_chars[str_idx]);
                                match piece_type {
                                    Some(piece) => board.set(piece, Square::from_coords(7-row_nr, col_nr).expect("read_fen generates a row and square that is not suposed to work")),
                                    None => panic!("You can't have {} in the board-part of the fen representation.", square_chars[str_idx])
                                }

                            } 

                            str_idx += 1;
                            col_nr += 1;
                        }                   
                    }
                    board.uppdate_occupancy();
                }
                


                1 => { // Handtere side som skal flytte
                    match info {
                        "w" | "W" => side_to_move = Color::White,
                        "b" | "B" => side_to_move = Color::Black,
                        _ => panic!("Invalid side to move in fen string: {}", info)
                    }
                }


                2 => { // Handtere rokkering

                    for kastle_side in info.chars().filter(|c| !c.is_whitespace()){
                        match kastle_side {
                            'K' => castling.add_castle_right(CastlingSide::WK),
                            'Q' => castling.add_castle_right(CastlingSide::WQ),
                            'k' => castling.add_castle_right(CastlingSide::BK),
                            'q' => castling.add_castle_right(CastlingSide::BQ),
                            '-' => (),
                            _ => panic!("Rokkeringa må bare innehalde K, Q, k eller q, men du gav inn {}", kastle_side)
                        }
                    }
                }


                3 => { //Handtere en passant 
                    match info {
                        "-" => en_passant = None,
                        _ =>  {
                            match Square::from_str(info) {
                                Ok(square) => en_passant = Some(square),
                                Err(error_message) => panic!("Failed adding en passant to position: {}", error_message)

                            }
                        }
                        
                        
                    }
                }


                4 => { // handtere halv trekk klokke, for 50 trekk regel.
                    match info.parse::<u16>() {
                        Ok(value) => halfmove_clock = value,
                        Err(_) => panic!("Failed to parse halfmove clock as u16: {}", info),
                    }
                }


                5 => { // Handtere heil trekk klokke, for å ... ej veit egentlig ikkje ka
                    match info.parse::<u16>() {
                        Ok(value) => fullmove_clock = value,
                        Err(_) => panic!("Failed to parse fullmove clock as u16: {}", info),
                    }
                }


                _ => {}

            }
        }
        
        Position { current: Snapshot{bitboards: board, side_to_move: side_to_move, castling: castling, en_passant: en_passant, halfmove_clock: halfmove_clock, fullmove_number: fullmove_clock }, history: vec![]}
    }






    pub fn write_fen(&self) -> String {
        let mut fen = String::new();
        // Board
        let mut no_piece_counter = 0;
        let mut current_idx = Bitboard::coord_to_index(7, 0); // this will loop trough the board in the right order starting from upper left
        
        loop{
            
            let square = Square::from_idx(current_idx).expect("Square was not to find in the indexes i loop through to write fen board");
            let piece_opt = self.current.bitboards.piece_on_square(square);

            match piece_opt {
                Some(piece) => {
                    if no_piece_counter > 0{
                        fen.push_str(&no_piece_counter.to_string());
                    }
                    fen.push(piece.to_fen_char());
                    no_piece_counter = 0;
                },
                None => no_piece_counter += 1
            }



            if current_idx == 7{ // 7 is the last index
                break;
            }

            let (row, col) = Bitboard::index_to_coord(current_idx);
            if col < 7{
                current_idx =  Bitboard::coord_to_index(row, col + 1);
            }
            else{
                current_idx = Bitboard::coord_to_index(row - 1, 0);
                if no_piece_counter > 0{
                    fen.push_str(&no_piece_counter.to_string());
                    no_piece_counter = 0;
                }
                fen.push('/');
                
            }
        }

        fen.push(' ');


        // Side to move
        if self.current.side_to_move == Color::White{fen.push('w');}
        else{fen.push('b');}
        fen.push(' ');

        // rokkering
        if self.current.castling.can_castle(CastlingSide::WK){ fen.push('K');}
        if self.current.castling.can_castle(CastlingSide::WQ){ fen.push('Q');}
        if self.current.castling.can_castle(CastlingSide::BK){ fen.push('k');}
        if self.current.castling.can_castle(CastlingSide::BQ){ fen.push('q');}
        fen.push(' ');

        // EN passant
        match self.current.en_passant {
            Some(en_passant_square) => {fen.push_str(&en_passant_square.square_str())},
            None => fen.push('-')
        }
        fen.push(' ');


        // Halvtrekk
        fen.push_str(&self.current.halfmove_clock.to_string());
        fen.push(' ');


        // Heiltrekk
        fen.push_str(&self.current.fullmove_number.to_string());

        fen
    }
}











#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fen_read(){
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 1 0";
        let position = Position::read_fen(fen_string);
        dbg!(&position);
        
        dbg!(position.write_fen());

    }
}