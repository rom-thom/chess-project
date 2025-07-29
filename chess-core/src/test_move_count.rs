

use crate::{position::Position};


fn _perft(board: &mut Position, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = board.legal_moves();
    let mut nodes = 0;

    for m in moves.iter() {
        board.make_move(m);
        nodes += _perft(board, depth - 1);
        if let Err(e) = board.undo_move(){
            panic!("can't undo move... {}", e);
        }
    }

    nodes
}



#[test]
fn test_count(){
    let mut pos = Position::new(Some("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 "));
    dbg!(_perft(&mut pos, 4));

}