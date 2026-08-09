use std::{io::{self, BufRead}, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub mod game;
pub mod communication;

use chess_core::{log};
use engine::{serch::serch_structs::SearchLimits, time_spending::TimeUsage};
use game::Game;


use crate::communication::{ComOutput, SearchCommand, com_loop, install_panic_log, send_move};




fn main() {
    let tt_size = 524288; // 2**19
    let opening_book_enabled = false;


    let stop_flag = Arc::new(AtomicBool::new(false));
    let search_stop_flag = stop_flag.clone();

    let (game_thred_tx, game_thred_rx) = channel();

    let game_thread = thread::spawn(move||{
        let mut game = Game::new(None, tt_size, opening_book_enabled);
        let mut time_usage = TimeUsage::new(None, None, None, None, None, None);
        
        while let Ok(command) = game_thred_rx.recv() {
            match command {
                SearchCommand::NewGame => {
                    game = Game::new(None, tt_size, opening_book_enabled)
                }
                SearchCommand::Position(pos_params) =>{
                    game.set_position(pos_params.fen, &pos_params.moves).expect("Coundn't sync the move history with the game position for some reason");
                }
                SearchCommand::Go(go_params) =>{
                    let max_time_ms = if let Some(mt) = go_params.movetime {Some(mt)} // If i get this then i know i dont get all the time info 
                        else if go_params.wtime.is_some() || go_params.btime.is_some() {
                            // normal clock mode
                            time_usage.update(go_params.wtime, go_params.btime, go_params.winc, go_params.binc, go_params.movestogo);
                            time_usage.time_to_use_ms(game.pos.current.side_to_move).map(|ms| ms)
                        }else{None};

                    let max_depth = go_params.depth.map(|depth| depth as usize);

                    let mut limits = SearchLimits::new(max_depth, max_time_ms, go_params.nodes, None, search_stop_flag.clone());

                    if let Some(time_to_spend) = &limits.max_time_ms{log!(format!("Time to use: {}", time_to_spend)).unwrap();}
                    let thinking_result = game.think(&mut limits); 
                    let best_move = thinking_result.best_move;
                    println!( "info depth {} score cp {} nodes {} time {}", thinking_result.depth, thinking_result.score, thinking_result.nodes, limits.time_elapsed_ms());
                    send_move(best_move);
                },
                SearchCommand::Quit => { break; },
            }
        }
    });


    let stdin = io::stdin();
    install_panic_log("rust_bot_panic.log"); // Make it print the panic messages to a tmp file

    for line in stdin.lock().lines() { // When it doesn't send something this kinda just whates fro the next line to be sendt
        
        let Ok(line) = line else { break };

        let com_output = com_loop(line);
        match com_output {
            ComOutput::Nada => {},
            ComOutput::NewGame => {game_thred_tx.send(SearchCommand::NewGame).expect("The game thred should always be running")},
            ComOutput::Quit => {
                stop_flag.store(true, Ordering::Relaxed);
                game_thred_tx.send(SearchCommand::Quit).expect("game thred has alredy bean escaped");
                let _ = game_thread.join(); // Whaiting for engine to stop before quiting everything
                break;
            },
            ComOutput::Stop => {stop_flag.store(true, Ordering::Relaxed);},
            ComOutput::Position(pos_params) => {game_thred_tx.send(SearchCommand::Position(pos_params)).expect("Game thred, unexpectedly stoped")}
            ComOutput::Go(go_params) => {
                stop_flag.store(false, Ordering::Relaxed);
                game_thred_tx.send(SearchCommand::Go(go_params)).expect("You have failed me gamethread"); 
            }
        };
    }
}





