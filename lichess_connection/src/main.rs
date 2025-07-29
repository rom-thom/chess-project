use dotenv::dotenv;
use std::env;
pub mod lichess_move_repr;
use chess_core;

fn main() {
    match dotenv() {
        Ok(path) => eprintln!("Loaded .env from {:?}", path),
        Err(err) => eprintln!(".env not found or couldn’t be read: {}", err),
    }

    let token = env::var("LICHESS_BOT_TOKEN")
        .expect("LICHESS_BOT_TOKEN must be set");
    dbg!(token);
}
