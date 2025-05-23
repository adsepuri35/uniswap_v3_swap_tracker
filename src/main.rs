use std::io::{self, Write};
use alloy::{core::primitives::Address, network::Ethereum, providers::{self, Provider, ProviderBuilder}, rpc::types::{Filter, Topic}, sol_types::SolEvent};
use anyhow::Result;
use dotenv::dotenv;
use std::{env, collections::HashMap};
use amms::amms::uniswap_v3::{IUniswapV3Factory::IUniswapV3FactoryInstance, IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap};
// use reqwest::Client;


// other file imports
mod ierc20;
use ierc20::IERC20;
mod poollnfo;
use poollnfo::PoolInfo;


const BLOCKS_TO_TRACK: u64 = 0;
// Common Uniswap V3 pools to track
// const UNISWAP_POOLS: [&str; 5] = [
//     "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640", // USDC-ETH 0.3%
//     "0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8", // USDC-ETH 0.05%
//     "0x7858E59e0C01EA06Df3aF3D20aC7B0003275D4Bf", // USDC-USDT 0.3%
//     "0xCBCdF9626bC03E24f779434178A73a0B4bad62eD", // WBTC-ETH 0.3%
//     "0x4e68Ccd3E89f51C3074ca5072bbAC773960dFa36"  // USDT-ETH 0.3%
// ];

// fn sqrt_price_x96_to_price(sqrt_price_x96: uint160) -> f64 {

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
    if logs.len() > 0 {
        println!("{:#?}", logs[0])
    }

    let mut token_address_to_symbol: HashMap<Address, String> = HashMap::new();
    let mut pool_address_to_index: HashMap<Address, u16> = HashMap::new();
    let mut pool_storage: Vec<PoolInfo> = Vec::new();


    for log in logs {
        if let Ok(decode) =  log.log_decode::<Swap>() {

            let swap = decode.data();
            let pool_address = log.address();


            let contract = IUniswapV3PoolInstance::new(pool_address, provider.clone());

            // acquire token addresses + symbols
            let token0_address = contract.token0().call().await?;
            let token1_address = contract.token1().call().await?;
            if !token_address_to_symbol.contains_key(&token0_address) {
                let ierc20_token0 = IERC20::new(token0_address, provider.clone());
                let token0_symbol = ierc20_token0.symbol().call().await?;
                token_address_to_symbol.insert(token0_address, token0_symbol);
            }
            if !token_address_to_symbol.contains_key(&token1_address) {
                let ierc20_token1 = IERC20::new(token1_address, provider.clone());
                let token1_symbol = ierc20_token1.symbol().call().await?;
                token_address_to_symbol.insert(token1_address, token1_symbol);
            }



            // add structs to pool storage (new pool)
            if !pool_address_to_index.contains_key(&pool_address) {
                pool_address_to_index.insert(pool_address, pool_storage.len() as u16);

                let token0_symbol = token_address_to_symbol
                    .get(&token0_address)
                    .cloned()
                    .unwrap_or_else(|| String::from("Unknown"));
                    
                let token1_symbol = token_address_to_symbol
                    .get(&token1_address)
                    .cloned()
                    .unwrap_or_else(|| String::from("Unknown"));

                let new_pool = PoolInfo::new(pool_address, token0_address, token1_address, token0_symbol, token1_symbol, 1);
                pool_storage.push(new_pool);
            } else {
                // implement functionality for pool updates
                if let Some(&index) = pool_address_to_index.get(&pool_address) {
                    let pool = &mut pool_storage[index as usize];
                    
                    // Update the pool with the new swap information
                    pool.increment_swap_count();
                } else {
                    println!("Error: Pool found in HashMap but no index available");
                }

            }
            // println!("Sender: {:?}", swap.sender);
            // println!("Recipient: {:?}", swap.recipient);

        }
    }


    for pool in &pool_storage {
        println!("{:?} = {:?}" ,pool.get_pool_name(), pool.get_swap_count());
    }


    Ok(())

}