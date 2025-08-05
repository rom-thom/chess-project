

use crate::{moves::Move, position::Position};


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
    let mut pos = Position::new(Some("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b Kkq e4"));
    dbg!(pos.current);
    dbg!(_perft(&mut pos, 2));

}