use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use tokio_stream::StreamExt;
use std::env;

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
    #[serde(rename = "gameState")]
    GameState { moves: String },
    // you can add other variants if you care (e.g. chatLine)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("LICHESS_BOT_TOKEN")
        .expect("LICHESS_BOT_TOKEN must be set");
    let client = Client::new();

    // 1) Connect to the global event stream
    let mut ev_stream = client
        .get("https://lichess.org/api/stream/event")
        .bearer_auth(&token)
        .send()
        .await?
        .bytes_stream();

    println!("Connected to Lichess event stream...");

    // 2) Handle challenges and game starts
    while let Some(chunk) = ev_stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let evt: LichessEvent = serde_json::from_str(line)?;
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
                    println!("Game started ({}): playing as {}", game.id, game.color);
                    play_game(&client, &token, game.id.clone(), game.color.clone()).await?;
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

    println!("Connected to game stream…");

    while let Some(chunk) = gs.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            if let Ok(GameEvent::GameState { moves }) = serde_json::from_str::<GameEvent>(line) {
                // split into ["e2e4", "e7e5", …]
                let history: Vec<&str> = moves.split_whitespace().collect();

                // determine whose turn it is:
                let my_turn = if color == "white" {
                    history.len() % 2 == 0
                } else {
                    history.len() % 2 == 1
                };

                if my_turn {
                    // 👉 generate your move here:
                    let uci = generate_move(&history);
                    println!("Playing move: {}", uci);

                    // send it to Lichess
                    let res = client
                        .post(&format!(
                            "https://lichess.org/api/bot/game/{}/move/{}",
                            game_id, uci
                        ))
                        .bearer_auth(token)
                        .send()
                        .await?;

                    if res.status().is_success() {
                        println!("Move {} sent ✅", uci);
                    } else {
                        eprintln!("Failed to send move: {}", res.status());
                    }
                }
            }
        }
    }

    Ok(())
}

/// Stub: replace with your own move-generation logic
/// `history` is the list of all past UCI moves in the game so far.
fn generate_move(_history: &[&str]) -> String {
    // For now, always play "e2e4"
    "e7e5".into()
}
