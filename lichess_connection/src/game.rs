use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use chess_core::{moves::{BitMove, MoveList}, position::{Color, Position}};
use engine::{engine::Engine, serch::serch_structs::{SearchLimits, SearchResult}, opening::opening::{OpeningBook, WHITE_BOOK, BLACK_BOOK}, eval::evaluator::Evaluator};


pub struct Game{
    pub pos: Position,
    engine: Engine,

    moves_played:  Vec<BitMove>,

}


impl Game{
    pub fn new(fen_string:Option<String>,  tt_size: usize, opening_book_enabled: bool)->Self{
        Self { pos: Position::new(fen_string), engine: Engine::new(tt_size, opening_book_enabled), moves_played: Vec::new()}
    }

    pub fn make_move(&mut self, mov: &BitMove){
        self.pos.make_move(mov);
        

        self.moves_played.push(*mov);

        //TODO: Chat wanted me to add this as wel to be able to change age stuf and so on in the tt
        //self.engine.on_new_root();
    }


    fn opening_move(&self) -> Option<BitMove> {
        let mut rng = rand::rng();
        let book: &'static OpeningBook = match self.pos.current.side_to_move {
            Color::White => &WHITE_BOOK,
            Color::Black => &BLACK_BOOK,
        };
        book.pick(&self.pos, &mut rng)
    }

    pub fn think(&mut self, limits: &mut SearchLimits) -> SearchResult{
        if self.engine.opening_book_enabled{
            if let Some(book_move) = self.opening_move() {
                let mut result = SearchResult::default();
                result.best_move = Some(book_move);

                return result;
            }
        }
        self.engine.think_iterative_deepening(&mut self.pos, limits)
    }


    pub fn undo_move(&mut self) -> Result<(), &'static str> {
        self.pos.undo_move()?;
        self.moves_played.pop();
        Ok(())
    }


    pub fn sync_moves(&mut self, move_list: &Vec<BitMove>)->Result<(), &'static str> {
        let new_size = move_list.len();
        let old_size = self.moves_played.len();
        if self.moves_played == *move_list{
            return Ok(());
        }
        if old_size < new_size{
            if &move_list.as_slice()[0..self.moves_played.len()] == self.moves_played.as_slice(){
                for bit_move in move_list.iter().skip(self.moves_played.len()){
                    self.make_move(bit_move);
                }
            }
        }
        else if old_size > new_size{
            for _ in 0..(old_size - new_size){
                self.undo_move()?;
            }
        }
        if self.moves_played == *move_list{
            return Ok(());
        }
        Err("Unable to sync the moves as they are somehow incompatable")
    }

    pub fn set_position(&mut self, fen: Option<String>, moves: &Vec<BitMove>) -> Result<(), &'static str> {
        // Reset the board, but preserve the Engine and its allocated TT.
        self.pos = Position::new(fen);
        self.moves_played = Vec::new();

        self.sync_moves(moves)
    }


}