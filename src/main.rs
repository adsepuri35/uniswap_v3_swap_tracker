use std::io::{self, Write};
use alloy::{network::Ethereum, providers::{Provider, ProviderBuilder}, rpc::types::{Filter, Topic}, sol_types::SolEvent};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use amms::amms::uniswap_v3::{IUniswapV3PoolEvents::Swap};
// use reqwest::Client;


const BLOCKS_TO_TRACK: u64 = 0;
// Common Uniswap V3 pools to track
// const UNISWAP_POOLS: [&str; 5] = [
//     "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640", // USDC-ETH 0.3%
//     "0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8", // USDC-ETH 0.05%
//     "0x7858E59e0C01EA06Df3aF3D20aC7B0003275D4Bf", // USDC-USDT 0.3%
//     "0xCBCdF9626bC03E24f779434178A73a0B4bad62eD", // WBTC-ETH 0.3%
//     "0x4e68Ccd3E89f51C3074ca5072bbAC773960dFa36"  // USDT-ETH 0.3%
// ];

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
    
    // let client = Client::new();

    let url = format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    let provider = ProviderBuilder::new()
        .network::<Ethereum>().
        connect_http(url.parse()?);

    println!("Uniswap tracker starting...");
    io::stdout().flush()?;

    // Get latest block
    let latest_block = provider.get_block_number().await?;
    println!("Latest block: {}", latest_block);
    
    // Calculate the block number from 100 blocks ago
    let from_block = if latest_block > BLOCKS_TO_TRACK {
        latest_block - BLOCKS_TO_TRACK
    } else {
        0
    };
    
    println!("Fetching logs from block {} to {}", from_block, latest_block);


    let events = vec![
        Swap::SIGNATURE_HASH
    ];

    let filter: Filter = Filter::new().
        from_block(from_block)
        .to_block(latest_block)
        .event_signature(events);

    let logs = provider.get_logs(&filter).await?;


    // println!("{:?}", logs);
    println!("Num Uniswap swap events: {}", logs.len());
    if logs.len() > 0 {
        println!("{:#?}", logs[0])
    }

    Ok(())

}