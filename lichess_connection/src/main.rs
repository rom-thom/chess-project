use std::io::{self, BufRead};

pub mod game;
pub mod communication;

use chess_core::{log};
use engine::{serch::serch_structs::SearchLimits, time_spending::TimeUsage};
use game::Game;


use std::fs; // For printing the prosess to a fil
use std::io::BufWriter;

use crate::communication::{ComOutput, com_loop, install_panic_log, send_move};




fn main() {
    let tt_size = 524288; // 2**19
    let opening_book_enabled = false;

    let mut game = Game::new(None, tt_size, opening_book_enabled);

    let mut time_usage = TimeUsage::new(None, None, None, None, None, None);


    let stdin = io::stdin();
    let mut sender = io::stdout();
    let mut logger = BufWriter::new(fs::File::create("chess_log/lichess_log.txt").expect("make this a path where you want to debug the lichess conversation"));
    install_panic_log("rust_bot_panic.log"); // Make it print the panic messages to a tmp file

    for line in stdin.lock().lines() { // When it doesnt send something this kinda just whates fro the next line to be sendt
        
        let Ok(line) = line else { break };

        let com_output = com_loop(line, &mut logger, &mut sender);
        match com_output {
            ComOutput::Nada => {},
            ComOutput::NewGame => {game = Game::new(None, tt_size, opening_book_enabled)},
            ComOutput::Quit => {break;},

            ComOutput::Position(pos_params) => {game.set_position(pos_params.fen, &pos_params.moves).expect("Coundn't sync the move history with the game position for some reason");}


            ComOutput::Go(go_params) => {
                let max_time_ms = if let Some(mt) = go_params.movetime {Some(mt)} // If i get this then i know i dont get all the time info 
                    else if go_params.wtime.is_some() || go_params.btime.is_some() {
                        // normal clock mode
                        time_usage.update(go_params.wtime, go_params.btime, go_params.winc, go_params.binc, go_params.movestogo);
                        time_usage.time_to_use_ms(game.pos.current.side_to_move).map(|ms| ms)
                    }else{None};

                let max_depth = go_params.depth.map(|depth| depth as usize);

                let mut limits = SearchLimits::new(max_depth, max_time_ms, go_params.nodes, None);

                if let Some(time_to_spend) = &limits.max_time_ms{log!(format!("Time to use: {}", time_to_spend)).unwrap();}
                let thinking_result = game.think(&mut limits);
                let best_move = thinking_result.best_move;
                send_move(best_move, &mut logger, &mut sender);
            }
        };
    }
}





