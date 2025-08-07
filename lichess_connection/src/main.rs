use dotenv::dotenv;
use std::env;
use reqwest::Client;
use tokio_stream::StreamExt;
use serde::Deserialize;
use serde_json;



#[derive(Debug, Deserialize)]
struct GameStart {
    id: String,
    // you can add more fields later if needed
}






#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token = env::var("LICHESS_BOT_TOKEN")
        .expect("LICHESS_BOT_TOKEN must be set");

    let client = Client::new();

    let response = client
        .get("https://lichess.org/api/stream/event")
        .bearer_auth(&token)
        .send()
        .await?;

    println!("Connected to Lichess event stream...");

    let mut lines = response.bytes_stream();

    while let Some(chunk) = lines.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines() {
            if !line.trim().is_empty() {
                


                match serde_json::from_str::<LichessEvent>(line) {
                    Ok(event) => {
                        match event {
                            LichessEvent::Challenge { challenge } => {
                                println!("New challenge: {:?}", challenge.id);

                                let accept_url = format!("https://lichess.org/api/challenge/{}/accept", challenge.id);

                                let res = client
                                    .post(&accept_url)
                                    .bearer_auth(&token)
                                    .send()
                                    .await?;

                                if res.status().is_success() {
                                    println!("Challenge accepted ✅");
                                } else {
                                    println!("Failed to accept challenge ❌: {}", res.status());
                                }
                            }






                            LichessEvent::GameStart { game } => {
                                println!("Game started: {:?}", game.id);

                                // Connect to game stream
                                let game_url = format!("https://lichess.org/api/bot/game/stream/{}", game.id);

                                let game_response = client
                                    .get(&game_url)
                                    .bearer_auth(&token)
                                    .send()
                                    .await?;

                                let mut game_lines = game_response.bytes_stream();

                                println!("Connected to game stream...");

                                while let Some(line) = game_lines.next().await {
                                    let line = line?;
                                    let text = String::from_utf8_lossy(&line);

                                    for game_line in text.lines() {
                                        if game_line.trim().is_empty() {
                                            continue;
                                        }

                                        println!("Game event: {}", game_line);

                                        // TODO: parse moves and decide when it's your turn
                                    }
                                }
                            }




                            

                        }
                    }
                    Err(err) => {
                        eprintln!("Could not parse event: {}", err);
                    }
                }





            }
        }
    }

    Ok(())
}







#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum LichessEvent {
    #[serde(rename = "challenge")]
    Challenge { challenge: Challenge },

    #[serde(rename = "gameStart")]
    GameStart { game: GameStart },
}


#[derive(Debug, Deserialize)]
struct Challenge {
    id: String,
    // You can add more fields later if needed
}
