use dotenv::dotenv;
use std::env;
use reqwest::blocking::Client; // blocking client for simplicity
use chess_core;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token = env::var("LICHESS_BOT_TOKEN")
        .expect("LICHESS_BOT_TOKEN must be set");

    let client = Client::new();

    let res = client
        .get("https://lichess.org/api/account")
        .bearer_auth(&token)
        .send()?;

    if res.status().is_success() {
        let body = res.text()?;
        println!("Successfully authenticated. Bot profile info:");
        println!("{}", body);
    } else {
        eprintln!("Authentication failed. Status: {}", res.status());
    }

    Ok(())
}
