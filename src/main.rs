use std::io::{self, Write};
use tokio::time::{sleep, Duration};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use reqwest::Client;

const BLOCKS_TO_TRACK: u64 = 100;
// Common Uniswap V3 pools to track
const UNISWAP_POOLS: [&str; 5] = [
    "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640", // USDC-ETH 0.3%
    "0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8", // USDC-ETH 0.05%
    "0x7858E59e0C01EA06Df3aF3D20aC7B0003275D4Bf", // USDC-USDT 0.3%
    "0xCBCdF9626bC03E24f779434178A73a0B4bad62eD", // WBTC-ETH 0.3%
    "0x4e68Ccd3E89f51C3074ca5072bbAC773960dFa36"  // USDT-ETH 0.3%
];

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

async fn get_logs_for_blocks(client: &Client, api_key: &str, from_block: u64, to_block: u64) -> Result<serde_json::Value> {
    let url = format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getLogs",
            "params": [{
                "fromBlock": format!("0x{:x}", from_block),
                "toBlock": format!("0x{:x}", to_block),
                "address": UNISWAP_POOLS,
                "topics": [
                    // Swap event signature
                    "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"
                ]
            }]
        }))
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    Ok(result)
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
    let latest_block = get_latest_block(&client, &api_key).await?;
    println!("Latest block: {}", latest_block);
    
    // Calculate the block number from 100 blocks ago
    let from_block = if latest_block > BLOCKS_TO_TRACK {
        latest_block - BLOCKS_TO_TRACK
    } else {
        0
    };
    
    println!("Fetching logs from block {} to {}", from_block, latest_block);
    
    // Get logs for the block range
    match get_logs_for_blocks(&client, &api_key, from_block, latest_block).await {
        Ok(logs) => {
            let logs_array = logs["result"].as_array().unwrap();
            println!("Found {} Uniswap V3 swap events", logs_array.len());
            
            // Print details of each swap
            for log in logs_array {
                let block_number = u64::from_str_radix(&log["blockNumber"].as_str().unwrap()[2..], 16)?;
                let pool_address = &log["address"].as_str().unwrap();
                println!("Block {}: Swap in pool {}", block_number, pool_address);
            }
        },
        Err(e) => {
            println!("Error getting logs: {}", e);
            return Err(e);
        }
    }

    // Keep the program running
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
