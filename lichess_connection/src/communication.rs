





use std::io::{self, BufRead, Write};


use chess_core::moves::{BitMove, MoveList};
use chess_core::{movegen::MoveGen, moves, position::Position};
use engine::{engine::Engine, serch::serch_structs::SearchLimits};
use crate::Game;

use std::{fs, vec}; // For printing the prosess to a fil
use std::io::BufWriter;




#[cfg(feature = "log")]
pub fn install_panic_log(path: &str) {
    use std::panic;

    let path = path.to_string();
    panic::set_hook(Box::new(move |info| {
        use std::fs::OpenOptions;

        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(f, "\n=== PANIC ===\n{info}\n");
        }
    }));
}
#[cfg(not(feature = "log"))]
pub fn install_panic_log(_path: &str) {
    // no-op in release
}




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




pub fn com_loop<W: Write>(line: String, mut logger: &mut W, mut sender: &mut io::Stdout)->ComOutput{
    let cmd = line.trim();
    log_line(&mut logger, &format!("recieved: {cmd}"));

    if cmd == "uci" {
        log_and_send(&mut logger, &mut sender, "id name rust_bot_2");
        log_and_send(&mut logger, &mut sender, "id author Thomas");
        log_and_send(&mut logger, &mut sender, "uciok");
        return ComOutput::Nada

    } else if cmd == "isready" {
        log_and_send(&mut logger, &mut sender, "readyok");
        return ComOutput::Nada;
    } else if cmd == "ucinewgame" {
        return ComOutput::NewGame
    } else if cmd == "quit" {
        return ComOutput::Quit

    } else if cmd.starts_with("position ") {
        match parse_position(cmd) {
            Ok(position) => return ComOutput::Position(position),
            Err(error) => {
                log_and_send(&mut logger, &mut sender, &format!("info string Invalid position command: {error}"),);
                return ComOutput::Nada;
            }
        }
    } else if cmd.starts_with("go") {
        let go_params = parse_go(cmd);
        return ComOutput::Go(go_params)
    }
    ComOutput::Nada
}

pub fn send_move<W: Write>(mv: Option<BitMove>, mut logger: &mut W, mut sender: &mut io::Stdout){
    if let Some(mv) = mv {
        log_and_send(&mut logger, &mut sender, &format!("bestmove {}", mv));

    } else {
        log_and_send(&mut logger, &mut sender, &format!("bestmove 0000"));
        }
    }


pub enum ComOutput{
    Nada,
    NewGame,
    Quit,
    Position(PositionParams),
    Go(GoParams)
}


#[derive(Debug)]
pub struct PositionParams {
    pub fen: Option<String>,
    pub moves: MoveList,
}

#[derive(Debug, Clone, Default)]
pub struct GoParams {
    pub wtime: Option<u64>,
    pub btime: Option<u64>,
    pub winc: Option<u64>,
    pub binc: Option<u64>,
    pub movestogo: Option<u32>,

    pub movetime: Option<u64>,
    pub depth: Option<u32>,
    pub nodes: Option<u64>,
    pub mate: Option<u32>,

    pub infinite: bool,
    pub ponder: bool,
}



fn parse_movelist(move_vect: Vec<String>, mut start_pos: Position)-> MoveList{
    let mut move_list = MoveList::new_empty();

    for mv in move_vect{
        let bit_move = MoveGen::stringmove_to_bitmove(&mut start_pos, &mv).expect(&format!("Move historyrecieved was incompatable with the start pos and its procedings. Couldnt make this a bitmove: {}", mv));
        move_list.add(bit_move);
        start_pos.make_move(&bit_move);
    };
    move_list
}

// Assumes go is the first command and the rest is structured like "go wtime 20000 btime 20978 ...""
fn parse_go(cmd: &str) -> GoParams {
    let mut p = GoParams::default();
    let mut it = cmd.split_whitespace().peekable();

    // first token is "go"
    let _ = it.next();

    while let Some(tok) = it.next() {
        match tok {
            "wtime" => p.wtime = it.next().and_then(|v| v.parse().ok()),
            "btime" => p.btime = it.next().and_then(|v| v.parse().ok()),
            "winc" => p.winc = it.next().and_then(|v| v.parse().ok()),
            "binc" => p.binc = it.next().and_then(|v| v.parse().ok()),
            "movestogo" => p.movestogo = it.next().and_then(|v| v.parse().ok()),

            "movetime" => p.movetime = it.next().and_then(|v| v.parse().ok()),
            "depth" => p.depth = it.next().and_then(|v| v.parse().ok()),
            "nodes" => p.nodes = it.next().and_then(|v| v.parse().ok()),
            "mate" => p.mate = it.next().and_then(|v| v.parse().ok()),

            "infinite" => p.infinite = true,
            "ponder" => p.ponder = true,

            _ => {}
        }
    }

    p
}



fn parse_position(cmd: &str)-> Result<PositionParams, String>{
    let tokens: Vec<&str> = cmd.split_whitespace().collect();

    let mut idx = 1;

    let fen  = match tokens.get(idx).copied() {
        Some("startpos") => {
            idx += 1;
            None
        },
        Some("fen") => {
            if tokens.len() < idx + 7{
                return Err("Incomplete fen... you gay or something".to_string());
            }
            let fen_string = tokens[idx + 1..idx + 7].join(" ");
            idx += 7;
            Some(fen_string)
        },  
        Some(other) =>{
            return Err(format!("The values after 'position' in the uci command was: {other}, i don't understand that)"))
        }
        None => return Err("Positioncommand was empty".to_string())    
    };

    let string_moves = if tokens.len() == idx{
        Vec::new()
    } else{
        if tokens[idx] != "moves" {
            return Err(format!("expected moves, received {}", tokens[idx]));
        };
        tokens[idx + 1 ..].iter().map(|m| m.to_string()).collect()
    };

    let start_pos = Position::new(fen.clone());
    let moves = parse_movelist(string_moves, start_pos);

    Result::Ok(PositionParams{fen, moves})
}





// position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4