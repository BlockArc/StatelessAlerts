mod handlers;
mod models;
mod state;

use std::env;
use dotenv::dotenv;
use axum::Router;
use handlers::{send_slack_message, send_telegram_message};
use state::AppState;
use std::net::SocketAddr;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file

    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set in .env");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set in .env");
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL not set in .env");

    let app_state = AppState {};
    let app = Router::new().with_state(app_state);

    // Spawn async tasks for Telegram and Slack messages
    task::spawn(async move {
        if let Err(err) = send_telegram_message(&bot_token, &chat_id, "Hello from Rust!").await {
            eprintln!("Failed to send Telegram message: {:?}", err);
        }
        if let Err(err) = send_slack_message(&slack_webhook_url, "Hello from Rust!").await {
            eprintln!("Failed to send Slack message: {:?}", err);
        }
    });

    // Start Axum server
    let addr: SocketAddr = "127.0.0.1:3000".parse()?;
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            eprintln!("Server error: {:?}", e);
            e.into()
        })
}
