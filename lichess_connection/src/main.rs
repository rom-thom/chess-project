use std::io::{self, BufRead};

pub mod game;
pub mod communication;


use engine::{serch::serch_structs::SearchLimits};
use game::Game;


use std::fs; // For printing the prosess to a fil
use std::io::BufWriter;

use crate::communication::{ComOutput, com_loop, install_panic_log, send_move};




fn main() {
    let tt_size = 524288; // 2**19
    let mut game = Game::new(None, tt_size);
    let mut limits = SearchLimits::new(Some(6), None); // This might change dynamicaly in the future


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
                let thinking_result = game.think(&limits);
                let best_move = thinking_result.best_move;
                send_move(best_move, &mut logger, &mut sender);
            }
        };

    }
}





