use chess_core::{movegen::{self, MoveGen}, moves::{BitMove, Move, MoveList, MovePath}, position::Color};

use crate::{serch, static_eval::evaluate};





pub fn serch_brute_force<const DEPTH: usize>(move_gen: &mut MoveGen) -> Option<BitMove>{
    let legal_moves = move_gen.legal_moves();
    if legal_moves.size() == 0{
        return None
    }
    let mut best_move = legal_moves.get(0).expect("Unposablet. I saw that the size was big enough");
    
    move_gen.pos.make_move(best_move);
    let mut best_eval = serch_brute_force_helper::<DEPTH>(move_gen, DEPTH-1); // al these 3 lines to just give a strating move
    move_gen.pos.undo_move().expect("I just did a move, so i should be able to undo later");

    for m in legal_moves.iter().skip(1){
        move_gen.pos.make_move(m);
        let eval = serch_brute_force_helper::<DEPTH>(move_gen, DEPTH-1);
        move_gen.pos.undo_move().expect("I just did a move, so i should be able to undo later");

        match move_gen.pos.current.side_to_move {
            Color::White => {
                if eval > best_eval {
                    best_eval = eval;
                    best_move = m;
                }
            }
            Color::Black => {
                if eval < best_eval {
                    best_eval = eval;
                    best_move = m;
                }
            }
        }

    }
    Some(*best_move)
}

fn serch_brute_force_helper<const DEPTH: usize>(move_gen: &mut MoveGen, depth: usize) -> i32{
    let legal_moves = move_gen.legal_moves();
    if depth == 0 || legal_moves.size() == 0{
        return evaluate(&move_gen.pos);
    }

    let mut moves_to_evaluate = [0; 218]; // 218 is the most moves posible in a chess position

    for (idx, m) in legal_moves.iter().enumerate(){
        move_gen.pos.make_move(m);
        moves_to_evaluate[idx] = serch_brute_force_helper::<DEPTH>(move_gen, depth-1);
        move_gen.pos.undo_move().expect("I should be able to undo the move as it has just been done");
    }  

    let mut best_move_eval = moves_to_evaluate[0];

    for eval in moves_to_evaluate[0..legal_moves.size()].iter() {
        // If it is the opponent he wants to return the best for him
        match move_gen.pos.current.side_to_move {
            Color::White => {
                if *eval > best_move_eval {
                    best_move_eval = *eval;
                }
            }
            Color::Black => {
                if *eval < best_move_eval {
                    best_move_eval = *eval;
                }
            }
        }
    }
    best_move_eval
}


#[test]
fn test_serch(){
    let mut move_gen = MoveGen::from_fen(Some("8/7r/1k4b1/8/7R/n4Q2/PP5P/K7 b - - 0 1"));
    dbg!(&move_gen.pos);
    dbg!(serch_brute_force::<3>(&mut move_gen));
}