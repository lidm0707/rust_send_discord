use anyhow::{anyhow, Result};
use chrono::{Local, FixedOffset};
use dotenvy::dotenv;
use reqwest::Client;
use std::env;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn, Level};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let webhook_url = env::var("URL")?;

    let client = Client::new();
    info!("Application started");

    let bangkok_offset = FixedOffset::east_opt(7 * 3600)
        .ok_or_else(|| anyhow!("Invalid timezone offset"))?;

    loop {
        info!("loop");
        let time = Local::now().with_timezone(&bangkok_offset).format("%Y-%m-%d %H:%M:%S");

        let payload = serde_json::json!({
            "content": format!("Hello from home at {}", time),
        });
        let response = client.post(&webhook_url).json(&payload).send().await?;

        if response.status().is_success() {
            info!("Message sent successfully!");
        } else if response.status().is_server_error() {
            warn!("Server error: {}", response.status());
        } else {
            error!("Failed to send message: {}", response.status());
        }
        // Sleep for 8 hours
        sleep(Duration::from_secs(8 * 60 * 60)).await;
    }
}
