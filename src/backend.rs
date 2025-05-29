use std::io::{self, Write};
use alloy::{core::primitives::Address, network::Ethereum, providers::{self, Provider, ProviderBuilder, WsConnect}, rpc::types::{Filter, Topic}, sol_types::SolEvent};
use anyhow::Result;
use dotenv::dotenv;
use std::{env, collections::HashMap, time::{Duration, Instant}};
use amms::amms::uniswap_v3::{IUniswapV3Factory::IUniswapV3FactoryInstance, IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap};
// use reqwest::Client;
use tokio::sync::mpsc;
use std::fs::OpenOptions;
use std::io::BufWriter;
use futures::stream::{self, StreamExt, SelectAll};


// other file imports
use crate::ierc20::IERC20;
use crate::poollnfo::PoolInfo;
use crate::swap_processor::process_swap_event;
use crate::tokenInfo::TokenInfo;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    price: Option<f64>,
}



pub async fn run_ws_backend(tx: mpsc::Sender<(Vec<PoolInfo>, usize, usize, usize)>) -> Result<()> {
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

    let events = vec![
        Swap::SIGNATURE_HASH
    ];
    let swap_filter = Filter::new().event_signature(events);

    // eth http provider
    let eth_url = format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    let eth_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_http(eth_url.parse()?);

    // base http provider
    let base_url = format!("https://base-mainnet.g.alchemy.com/v2/{}", api_key);
    let base_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_http(base_url.parse()?);

    // arb http provider
    let arb_url = format!("https://arb-mainnet.g.alchemy.com/v2/{}", api_key);
    let arb_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_http(arb_url.parse()?);


    
    // eth websocket stream
    let eth_ws_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    let eth_ws_connect = WsConnect::new(&eth_ws_url);
    let eth_ws_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_ws(eth_ws_connect)
        .await?;

    let eth_ws_subcription = eth_ws_provider.subscribe_logs(&swap_filter).await?;
    let mut eth_ws_stream = eth_ws_subcription.into_stream().map(|log| ("eth", log)).boxed();

    // base websocket stream
    let base_ws_url = format!("wss://base-mainnet.g.alchemy.com/v2/{}", api_key);
    let base_ws_connect = WsConnect::new(&base_ws_url);
    let base_ws_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_ws(base_ws_connect)
        .await?;
    let base_ws_subscription = base_ws_provider.subscribe_logs(&swap_filter).await?;
    let mut base_ws_stream = base_ws_subscription.into_stream().map(|log| ("base", log)).boxed();

    // arb websocket stream
    let arb_ws_url = format!("wss://arb-mainnet.g.alchemy.com/v2/{}", api_key);
    let arb_ws_connect = WsConnect::new(&arb_ws_url);
    let arb_ws_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_ws(arb_ws_connect)
        .await?;
    let arb_ws_subscription = arb_ws_provider.subscribe_logs(&swap_filter).await?;
    let mut arb_ws_stream = arb_ws_subscription.into_stream().map(|log| ("arb", log)).boxed();


    let mut merged_stream = stream::select_all(vec![
        eth_ws_stream,
        base_ws_stream,
        arb_ws_stream,
    ]);




    // println!("Uniswap tracker starting...");
    io::stdout().flush()?;

    // // get latest block
    // let latest_block = provider.get_block_number().await?;
    // println!("Latest block: {}", latest_block);
    
    // // get curr block - x
    // let from_block = if latest_block > BLOCKS_TO_TRACK {
    //     latest_block - BLOCKS_TO_TRACK
    // } else {
    //     0
    // };
    
    // println!("Fetching logs from block {} to {}", from_block, latest_block);

    // let range_filter: Filter = Filter::new().
    //     from_block(from_block)
    //     .to_block(latest_block)
    //     .event_signature(events.clone());

    // let logs = provider.get_logs(&range_filter).await?;

    

    // println!("{:?}", logs);
    // println!("Num Uniswap swap events: {}", logs.len());
    // if logs.len() > 0 {
    //     println!("{:#?}", logs[0])
    // }

    let mut token_info_map: HashMap<Address, TokenInfo> = HashMap::new();
    let mut pool_address_to_index: HashMap<(String, Address), u16> = HashMap::new();
    let mut pool_storage: Vec<PoolInfo> = Vec::new();

    let mut eth_swaps = 0;
    let mut base_swaps = 0;
    let mut arb_swaps = 0;

    //stream until interrupted
    loop {
        match tokio::time::timeout(Duration::from_secs(3), merged_stream.next()).await {
            Ok(Some((network, log))) => {
                if let Ok(decode) = log.log_decode::<Swap>() {
                    // let block_number = log.block_number;
                    // println!("block number: {:?}", block_number);
                    // println!("swap detected");

                    let swap = decode.data();


                    let provider = match network {
                        "eth" => {
                            eth_swaps += 1;
                            eth_provider.clone()
                        }
                        "base" => {
                            base_swaps += 1;
                            base_provider.clone()
                        }
                        "arb" => {
                            arb_swaps += 1;
                            arb_provider.clone()
                        }
                        _ => {
                            continue;
                        }
                    };
                    match process_swap_event(
                        &log,
                        provider,
                        network,  
                        &mut token_info_map,
                        &mut pool_address_to_index,
                        &mut pool_storage
                    ).await {
                        Ok(_) => {
                            // Only send on success
                            tx.send((pool_storage.clone(), eth_swaps, base_swaps, arb_swaps)).await?;
                        },
                        Err(e) => {
                            let file_result = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("swap_errors.log");
                            
                            match file_result {
                                Ok(file) => {
                                    // Write to the log file
                                    let mut writer = BufWriter::new(file);
                                    writeln!(writer, "Error processing swap: {}", e).ok();
                                },
                                Err(file_err) => {
                                    // If we can't open the file, write to stderr directly
                                    eprintln!("Error processing swap: {} (couldn't open log: {})", e, file_err);
                                }
                            }
                        }
                    }

                    // send updated pools
                    tx.send((pool_storage.clone(), eth_swaps, base_swaps, arb_swaps)).await?;
                }
            },
            Ok(None) => {
                println!("\nWebSocket stream ended unexpectedly");
                break;
            },
            Err(_) => {
                // println!("=============================================");
                //     for pool in &pool_storage {
                //         println!("{:?} = {:?}" ,pool.get_pool_name(), pool.get_swap_count());
                //     }
                // println!("=============================================");
                io::stdout().flush();

                if !pool_storage.is_empty() {
                    // println!("Attempting to send {} pools through channel", pool_storage.len());
                    match tx.send((pool_storage.clone(), eth_swaps, base_swaps, arb_swaps)).await {
                        Ok(_) => {},
                        Err(e) => println!("Failed to send pools: {}", e),
                    }
                }
            }
        }
    }

    // for pool in &pool_storage {
    //     println!("{:?} = {:?}" ,pool.get_pool_name(), pool.get_swap_count());
    // }

    Ok(())
}



