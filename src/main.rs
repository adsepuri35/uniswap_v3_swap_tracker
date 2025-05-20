use std::io::{self, Write};
use tokio::time::{sleep, Duration};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use reqwest::Client;

async fn get_latest_block(client: &Client, api_key: &str) -> Result<u64> {
    let url = format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_blockNumber",
            "params": []
        }))
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    let block_number_hex = result["result"].as_str().unwrap();
    
    // Convert hex string to u64
    let block_number = u64::from_str_radix(&block_number_hex[2..], 16)?;
    Ok(block_number)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    let api_key = match env::var("ALCHEMY_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("Error loading API key: {}", e);
            return Err(anyhow::anyhow!("Failed to load ALCHEMY_API_KEY from .env file"));
        }
    };
    
    let client = Client::new();

    println!("Uniswap tracker starting...");
    io::stdout().flush()?;

    // Get latest block
    match get_latest_block(&client, &api_key).await {
        Ok(latest_block) => {
            println!("Latest block: {}", latest_block);
        },
        Err(e) => {
            println!("Error getting latest block: {}", e);
            return Err(e);
        }
    }
    io::stdout().flush()?;

    // Keep the program running
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
