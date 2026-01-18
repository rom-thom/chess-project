use chess_core::{moves::{BitMove, MoveList}, position::Position};
use engine::{engine::Engine, serch::serch_result::SearchResult};



pub struct Game{
    pos: Position,
    engine: Engine,

    moves_played:  MoveList,
}


impl Game{
    pub fn new(fen_string:Option<&str>,  tt_size: usize)->Self{
        Self { pos: Position::new(fen_string), engine: Engine::new(tt_size), moves_played: MoveList::new_empty()}
    }

    pub fn make_move(&mut self, mov: &BitMove){
        self.pos.make_move(mov);
        self.moves_played.add(*mov);

        //TODO: Chat wanted me to add this as wel to be able to change age stuf and so on in the tt
        //self.engine.on_new_root();
    }


    pub fn think(&mut self, depth: usize) -> SearchResult{
        self.engine.negamax(&mut self.pos, depth)
    }

}