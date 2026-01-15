use chess_core::{moves::MoveList, position::Position};
use engine::engine::Engine;



pub struct Game{
    pos: Position,
    engine: Engine,

    history:  MoveList,
}


impl Game{

}