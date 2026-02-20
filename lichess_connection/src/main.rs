use std::io::{self, BufRead, Write};

pub mod game;

use chess_core::{movegen::MoveGen, moves, position::Position};
use engine::{engine::Engine, serch::serch_structs::SearchLimits};
use game::Game;

use std::fs; // For printing the prosess to a fil
use std::io::BufWriter;



fn log_line<W: Write>(logger: &mut W, line: &str){
    writeln!(logger, "{line}").ok();
    logger.flush().ok(); 
}
fn log_and_send<W: Write>(logger: &mut W, sender: &mut io::Stdout, line: &str){
    writeln!(logger, "Sendt: {line}").ok();
    logger.flush().ok(); 
    writeln!(sender, "{line}").expect(&format!("Couldn't send line {line}"));
    sender.flush().ok();
}

fn main() {
    let tt_size = 4;
    // let limits = SearchLimits::new(Some(3), None);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut io_file = BufWriter::new(fs::File::create("/home/thomas-paulen/Dokument/Privat/programering/rust/chess/chess-project/lichess_connection/lichess_log.txt").expect("make this a path where yyou want to debug the lichess conversation"));

    // let mut game = Game::new(None, tt_size);
    // log_line(&mut io_file, "test");


    let mut pos = Position::new(None);

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let cmd = line.trim();
        println!("recieved = {cmd}");
        log_line(&mut io_file, &format!("recieved: {cmd}"));

        if cmd == "uci" {
            log_and_send(&mut io_file, &mut stdout, "id name rust_bot_2");
            log_and_send(&mut io_file, &mut stdout, "id author Thomas");
            log_and_send(&mut io_file, &mut stdout, "uciok");

        } else if cmd == "isready" {
            log_and_send(&mut io_file, &mut stdout, "readyok");
            
        } else if cmd == "ucinewgame" {
            // game = Game::new(None, tt_size); // reset game/TT etc.
            pos = Position::new(None);

        } else if cmd == "quit" {
            break;
        } else if cmd.starts_with("position ") {
            // TODO: Make this not rebuild from scratch for every move that is made

            let history = parse_position_moves(cmd);
            pos = build_current_pos(history)

        } else if cmd.starts_with("go") {
            let best_move = alpha_beta_generator(pos.clone());

            if let Some(mv) = best_move {
            log_and_send(&mut io_file, &mut stdout, &format!("bestmove {}", mv));

            } else {
            log_and_send(&mut io_file, &mut stdout, &format!("bestmove 0000"));
            }
        }

    }
}


fn alpha_beta_generator(mut pos: Position) -> Option<String>{

    let mut legal_moves = moves::MoveList::new_empty();
    MoveGen::fill_legal(&mut pos, &mut legal_moves);

    let mut engine = Engine::new(524288); // 2**19
    let serch_result = engine.negamax(&mut pos, 5);
    let best_move = serch_result.best_move;

    if let Some(bm) = best_move{
        return Some(bm.to_string());
    }
    else{
        return None;
    }
}


fn build_current_pos(history:  Vec<&str>) -> Position{
    let mut pos = Position::new(None); // We are starting from the starting_ position
    for mov in history{
        let str_move = MoveGen::stringmove_to_bitmove(&mut pos,mov).expect("Couldn't convert move recieved from liches to bitmove");
        pos.make_move(&str_move);
    };
    pos
}


fn parse_position_moves(cmd: &str) -> Vec<&str> {
    // cmd is the full line, e.g. "position startpos moves e2e4 e7e5"
    if let Some(idx) = cmd.find(" moves ") {
        let moves_part = &cmd[idx + " moves ".len()..];
        moves_part.split_whitespace().collect()
    } else {
        Vec::new()
    }
}