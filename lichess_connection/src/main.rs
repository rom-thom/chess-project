use std::io::{self, BufRead};

pub mod game;
pub mod communication;


use engine::{debug_file::log_dbg, serch::serch_structs::SearchLimits, time_spending::TimeUsage};
use game::Game;


use std::fs; // For printing the prosess to a fil
use std::io::BufWriter;

use crate::communication::{ComOutput, com_loop, install_panic_log, send_move};




fn main() {
    let tt_size = 524288; // 2**19
    let mut game = Game::new(None, tt_size);
    let mut limits = SearchLimits::new(None, Some(18_000), None);

    let mut time_usage = TimeUsage::new(None, None, None, None, None, None);


    let stdin = io::stdin();
    let mut sender = io::stdout();
    let mut logger = BufWriter::new(fs::File::create("/tmp/lichess_log.txt").expect("make this a path where you want to debug the lichess conversation"));
    install_panic_log("/tmp/rust_bot_panic.log"); // Make it print the panic messages to a tmp file

    for line in stdin.lock().lines() {

        let Ok(line) = line else { break };

        let com_output = com_loop(line, &mut logger, &mut sender);
        match com_output {
            ComOutput::Nada => {},
            ComOutput::NewGame => {game = Game::new(None, tt_size)},
            ComOutput::Quit => {break;},
            ComOutput::PosHist(hist) => {game.sync_moves(&hist).expect("Coundn't sync the move history with the game position for some reason");},
            ComOutput::Go(go_params) => {
                if let Some(mt) = go_params.movetime { // If i get this then i know i dont get all the time info 
                    limits.max_time_ms = Some(mt);
                } else {
                    // normal clock mode
                    time_usage.update(go_params.wtime, go_params.btime, go_params.winc, go_params.binc, go_params.movestogo);
                    limits.max_time_ms = time_usage.time_to_use_ms(game.pos.current.side_to_move).map(|ms| ms);
                }

                log_dbg("/tmp/debug_file.log", "Time to use: ", &limits.max_time_ms.unwrap_or(67), file!(), line!()).unwrap();
                let thinking_result = game.think(&mut limits);
                let best_move = thinking_result.best_move;
                send_move(best_move, &mut logger, &mut sender);
            }
        };

    }
}





