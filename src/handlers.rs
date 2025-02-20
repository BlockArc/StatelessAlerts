use serde_json::json;
use reqwest::{Client, StatusCode};

/// Sends a message to Slack using a webhook URL.
pub async fn send_slack_message(webhook_url: &str, message: &str) -> Result<(), String> {
    let client = Client::new();
    let payload = json!({ "text": message });

    let res = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if res.status() == StatusCode::OK {
        println!("Slack message sent successfully.");
        Ok(())
    } else {
        Err(format!(
            "Failed to send Slack message. Status: {:?}, Response: {:?}",
            res.status(),
            res.text().await.unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

/// Sends a message to a Telegram bot.
pub async fn send_telegram_message(bot_token: &str, chat_id: &str, message: &str) -> Result<(), String> {
    let client = Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        bot_token
    );

    let payload = json!({
        "chat_id": chat_id,
        "text": message
    });

    let res = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if res.status() == StatusCode::OK {
        println!("Telegram message sent successfully.");
        Ok(())
    } else {
        Err(format!(
            "Failed to send Telegram message. Status: {:?}, Response: {:?}",
            res.status(),
            res.text().await.unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}
