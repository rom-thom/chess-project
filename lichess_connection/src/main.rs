use chess_core::movegen::MoveGen;
use chess_core::position::Position;
use dotenv::dotenv;
use engine::engine::Engine;
use engine::serch;
use engine::serch::serch_structs::SearchLimits;
use reqwest::Client;
use serde::Deserialize;
use tokio::time;
use tokio_stream::StreamExt;
use std::env;
pub mod game;


// Enjoy the wonderful work of vibe coding:

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct LichessEvent {
    #[serde(rename = "type")]
    kind: String,
    #[serde(flatten)]
    inner: EventInner,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EventInner {
    Challenge { challenge: Challenge },
    GameStart { game: GameStart },
}

#[derive(Debug, Deserialize)]
struct Challenge {
    id: String,
}

#[derive(Debug, Deserialize)]
struct GameStart {
    id: String,
    color: String,    // "white" or "black"
}

#[derive(Debug, Deserialize)]
struct GameState {
    moves: String,    // space-separated UCI strings
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum GameEvent {
    #[serde(rename = "gameFull")]
    GameFull { state: GameState },

    #[serde(rename = "gameState")]
    GameState { moves: String },
    // you can add other variants if you care (e.g. chatLine)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("LICHESS_BOT_TOKEN")
        .expect("LICHESS_BOT_TOKEN must be set");
    let client = reqwest::Client::builder()
    .tcp_keepalive(Some(Duration::from_secs(30)))
    .pool_idle_timeout(Some(Duration::from_secs(15)))
    .pool_max_idle_per_host(0) // avoids stale pooled conns
    .build()?;


    // 1) Connect to the global event stream
    let mut ev_stream = client
        .get("https://lichess.org/api/stream/event")
        .header("Accept", "application/x-ndjson")
        .bearer_auth(&token)
        .send()
        .await?
        .bytes_stream();

    println!("Connected to Lichess event stream...");

    // For counting games
    let active_games = Arc::new(AtomicUsize::new(0));

    // 2) Handle challenges and game starts
    while let Some(chunk) = ev_stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let evt: LichessEvent = serde_json::from_str(line.trim())?;
            match evt.inner {
                EventInner::Challenge { challenge } => {
                    println!("New challenge: {}", challenge.id);
                    client
                        .post(&format!(
                            "https://lichess.org/api/challenge/{}/accept",
                            challenge.id
                        ))
                        .bearer_auth(&token)
                        .send()
                        .await?
                        .error_for_status()?;
                    println!("Challenge accepted ✅");
                }
                EventInner::GameStart { game } => {

                    // Spawn a new task for each game:
                    let client   = client.clone();
                    let token    = token.clone();
                    let id       = game.id.clone();
                    let color    = game.color.clone();
                    let counter  = active_games.clone();

                    // Increment and print:
                    let now = counter.fetch_add(1, Ordering::SeqCst) + 1;
                    println!(
                        "➤ [{}] Game started (playing as {}).  🚀 {} game(s) running",
                        id, color, now
                    );

                    

                    tokio::spawn(async move {
                        // Any println! inside play_game we’ll prefix with [id], see below
                        if let Err(err) = play_game(&client, &token, id.clone(), color.clone()).await {
                            eprintln!("[{}] ⚠️ Game error: {}", id, err);
                        }
                        // ------------- After finishing: decrement & log -------------
                        let remaining = counter.fetch_sub(1, Ordering::SeqCst) - 1;
                        println!("➤ [{}] Game finished. 🛑 {} game(s) remaining", id, remaining);
                        // ------------------------------------------------------------
                    });
                }
            }
        }
    }

    Ok(())
}

async fn play_game(
    client: &Client,
    token: &str,
    game_id: String,
    color: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // 3) Open the per-game stream
    let mut gs = client
        .get(&format!("https://lichess.org/api/bot/game/stream/{}", game_id))
        .bearer_auth(token)
        .send()
        .await?
        .bytes_stream();

    println!("[{}] Connected to game stream…", game_id);

    while let Some(chunk) = gs.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines().filter(|l| !l.trim().is_empty()) {

            // parse either `gameFull` or `gameState`
            if let Ok(evt) = serde_json::from_str::<GameEvent>(line.trim()) {
                // extract the move list in either case
                let moves_str = match evt {
                    GameEvent::GameFull { state }  => state.moves,
                    GameEvent::GameState { moves } => moves,
                };

                // now split and decide if it's your turn
                let history: Vec<&str> = moves_str.split_whitespace().collect();
                let my_turn = if color == "white" {
                    history.len() % 2 == 0
                } else {
                    history.len() % 2 == 1
                };

                if my_turn {
                    match alpha_beta_generator(move_setup(&history)) {
                        Some(uci) => {
                            println!("[{}] Playing move: {}", game_id, uci);

                        let res = client
                            .post(format!("https://lichess.org/api/bot/game/{}/move/{}", game_id, uci))
                            .bearer_auth(token)
                            .timeout(Duration::from_secs(10))  // safe here; not applied to the stream
                            .send()
                            .await?;


                        if res.status().is_success() {
                            println!("[{}] Move {} sent ✅", game_id, uci);
                        } else {
                            eprintln!("[{}] Failed to send move: {}", game_id, res.status());
                        }
                    }
                    None => {
                        println!("[{}] No legal moves; game over.", game_id);
                        return Ok(());
                    }
                    }
                }
            }
        }
    }

    Ok(())
}




fn move_setup(history:  &[&str]) -> Position{
    let mut pos = Position::new(None); // We are starting from the starting_ position
    for mov in history{
        let str_move = MoveGen::stringmove_to_bitmove(&mut pos,mov).expect("Couldn't convert move recieved from liches to bitmove");
        pos.make_move(&str_move);
    };
    pos
}

use chess_core::{piece::Piece, *};
use rand::Rng;

use crate::game::Game;

/// `history` is the list of all past UCI moves in the game so far.
fn random_move(history: &[&str]) -> Option<String> { // random for now
    let mut pos = Position::new(None); // We are starting from the starting_ position
    for mov in history{
        let str_move = MoveGen::stringmove_to_bitmove(&mut pos,mov).expect("Couldn't convert move recieved from liches to bitmove");
        pos.make_move(&str_move);
    };
    let mut legal_moves = moves::MoveList::new_empty();
    MoveGen::fill_legal(&mut pos, &mut legal_moves);


    // if no legal moves, game is over
    if legal_moves.size() == 0 {
        return None;
    }
    let mut good_moves = moves::MoveList::new_empty();
    for i in legal_moves.iter(){
        if i.is_capture(){
            good_moves.add(*i);
            continue;
        }
        pos.make_move(i);
        let opp = MoveGen::legal_moves(&mut pos);
        if opp.size() == 0{
            pos.current.side_to_move = !pos.current.side_to_move;
            let my_2 = MoveGen::legal_moves(&mut pos);
            for my_move in my_2.iter(){
                if let Some(piece_index) = pos.current.bitboards.piece_on_square(my_move.get_end_square()){
                    if piece_index.to_piece() == Piece::King{
                        return Some(i.to_string());
                    }
                }
                
                continue;
            }
            pos.current.side_to_move = !pos.current.side_to_move;
        }
        pos.undo_move().expect("couldn't undo move i just did");
    }
    if good_moves.size() == 0{
        good_moves = legal_moves;
    }


    std::thread::sleep(Duration::from_millis(5000));
    // find random move
    let mut rng = rand::rng();
    let move_index = rng.random_range(0..good_moves.size());

    let move_to_play = good_moves.get(move_index).expect("Here should be a move, otherwise the rand thing has found an elegal mmove index");
    Some(move_to_play.to_string())
}



fn alpha_beta_generator(mut pos: Position) -> Option<String>{

    let mut legal_moves = moves::MoveList::new_empty();
    MoveGen::fill_legal(&mut pos, &mut legal_moves);

    let mut engine = Engine::new(524288); // 2**19
    let serch_result = engine.negamax(&mut pos, 7);
    let best_move = serch_result.best_move;

    if let Some(bm) = best_move{
        return Some(bm.to_string());
    }
    else{
        return None;
    }
}




fn engine_setup(fen_string:Option<&str>) -> Game{
    Game::new(fen_string, 8) //TT = 2**3
}

fn while_other_thinks(pos: &mut Position, game: &mut Game){
    // TODO: Make it so that there is an instant way of aborting a search, for example using multi threding
    game.think(pos, SearchLimits{max_depth: Some(5), max_time_ms: None});
}

fn find_best_move(){
    
}