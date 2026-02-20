

use crate::{movegen::MoveGen, moves::{BitMove, Move, MoveList}, position::{self, Color, Position}};
use std::io::{self, Write};
use rand::Rng;

fn _outer_perft(pos: &mut Position, depth: u32) -> u64{
    _perft(pos, depth, depth)
}

fn _perft(mut pos: &mut Position, depth: u32, max_depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    

    let moves = MoveGen::legal_moves(&mut pos);
    let mut nodes = 0;

    for m in moves.iter() {
        pos.make_move(m);
        let perft = _perft(pos, depth - 1, max_depth);
        // if depth == max_depth{dbg!(m.to_string(), perft);}// For debugging (prints out each move, and theyer own perft)
        nodes += perft;
        if let Err(e) = pos.undo_move(){
            panic!("can't undo move... {}", e);
        }
    }

    nodes
}

// This must be runn using this comand in the terminal "cargo test test_count -- --nocapture"
fn play_random_engine(starting_fen_string: Option<String>, your_color: Color){
    let mut pos = Position::new(starting_fen_string);
    let mut user_move_string = String::new();
    let mut user_move_bitmove;


    match your_color {
        Color::White => {
            print!("You are white, play a move in the format startsquare endsquare potential promotion (example: e2e4 or later e7f8r for promoting to rook): ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut user_move_string).expect("Failed to read line");
            user_move_bitmove = MoveGen::stringmove_to_bitmove(&mut pos, &user_move_string.trim()).expect("First move was wrong/invalid start the program again, but now with a valid move");
            pos.make_move(&user_move_bitmove);  
        }
        Color::Black =>{
            println!("You are black, so i start, after that, play a move in the format startsquare endsquare potential promotion (example: e2e4 or later e7f8r for promoting to rook): ");
            println!("btw, i couldn't bather making it flip the board, so screw you, you are playing upside down");

        }
    }
    let mut legal_moves = MoveList::new_empty();
    let mut rng = rand::rng();
    let mut move_index;
    loop{
        legal_moves.clear();
        MoveGen::fill_legal(&mut pos, &mut legal_moves);
        if legal_moves.size() == 0{
            println!("I have no legal moves, so you determine if you win or it is remis/pat/draw, or just that the chess engine is broken (but that is unposible. After all i did make it)");
            break;
        }
        move_index = rng.random_range(0..legal_moves.size());

        pos.make_move(legal_moves.get(move_index).expect("the random generator, generated a number that is outside the size, which shouldnt happen, but here we are, in this cruel wourld"));

        print!("{:?}", pos.current);

        'wrong_input_move_loop: loop{
            print!("Your turn what move do you want to play: ");
            user_move_string.clear();
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut user_move_string).expect("Failed to read line");

            match MoveGen::stringmove_to_bitmove(&mut pos, &user_move_string.trim()) {
                Ok(user_move) =>{
                    user_move_bitmove = user_move;
                    break 'wrong_input_move_loop
                }
                Err(e)=>{
                    println!("that was an invalid move idiot. You have to understand that this would happen: {}", e);
                }
            }
        }
        pos.make_move(&user_move_bitmove);
    }
}






#[test]
fn test_count(){
    let mut pos = Position::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ".to_string()));
    assert_eq!(197281  , _outer_perft(&mut pos, 4));
    pos = Position::new(Some("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ".to_string()));
    assert_eq!(4085603 , _outer_perft(&mut pos, 4));
    pos = Position::new(Some("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ".to_string()));
    assert_eq!(43238 , _outer_perft(&mut pos, 4));
    pos = Position::new(Some("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string()));
    assert_eq!(422333 , _outer_perft(&mut pos, 4));
    pos = Position::new(Some("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string()));
    assert_eq!(62379 , _outer_perft(&mut pos, 3));
}

#[test]
fn test_speed(){
    dbg!("Previous results: ");
    dbg!("23 s");
    dbg!("13 s");
    dbg!("3.9 s");
    let mut pos = Position::new(Some("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ".to_string()));
    dbg!(&pos);
    dbg!(_outer_perft(&mut pos, 4));
}