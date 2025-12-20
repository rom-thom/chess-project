use chess_core::{moves::BitMove, position::ZobristKey};



struct TransPosition{
    zobrist_key: ZobristKey,
    best_move: BitMove,
    score: i32,
    depth: u8
}


struct TT{
    table: [TransPosition; 500]
}