use std::io::{self, Write};
use alloy::{network::Ethereum, providers::{Provider, ProviderBuilder}, rpc::types::{Filter, Topic}, sol_types::SolEvent};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use amms::amms::uniswap_v3::{IUniswapV3Factory::IUniswapV3FactoryInstance, IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap};
// use reqwest::Client;

mod ierc20;
use ierc20::IERC20;


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
    // load env variables
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

    // get latest block
    let latest_block = provider.get_block_number().await?;
    println!("Latest block: {}", latest_block);
    
    // get curr block - x
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
    // if logs.len() > 0 {
    //     println!("{:#?}", logs[0])
    // }

    for log in logs {
        if let Ok(decode) =  log.log_decode::<Swap>() {

            let swap = decode.data();
            println!("Pool address: {:?}", log.address());

            let contract = IUniswapV3PoolInstance::new(log.address(), provider.clone());
            let token0_address = contract.token0().call().await?;
            let token1_address = contract.token1().call().await?;
            // println!("Token address: {:?}", token0_address);

            let ierc20_token0 = IERC20::new(token0_address, provider.clone());
            let ierc20_token1 = IERC20::new(token1_address, provider.clone());
            println!("Token 0: {:?}", ierc20_token0.symbol().call().await?);
            println!("Token 1: {:?}", ierc20_token1.symbol().call().await?);

    
            println!("Sender: {:?}", swap.sender);
            println!("Recipient: {:?}", swap.recipient);

        }
    }

    Ok(())

}