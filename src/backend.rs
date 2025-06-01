use std::{io::{self, Write}, ops::Add};
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
use crate::prices::get_token_price;
use crate::backendUpdate::BackendUpdate;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    price: Option<String>,
}


pub async fn run_ws_backend(tx: mpsc::Sender<(BackendUpdate)>) -> Result<()> {
    // load env variables
    dotenv().ok();
    
    let api_key = match env::var("ALCHEMY_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("Error loading API key: {}", e);
            return Err(anyhow::anyhow!("Failed to load ALCHEMY_API_KEY from .env file"));
        }
    };

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
    let eth_ws_stream = eth_ws_subcription.into_stream().map(|log| ("eth-mainnet", log)).boxed();

    // base websocket stream
    let base_ws_url = format!("wss://base-mainnet.g.alchemy.com/v2/{}", api_key);
    let base_ws_connect = WsConnect::new(&base_ws_url);
    let base_ws_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_ws(base_ws_connect)
        .await?;
    let base_ws_subscription = base_ws_provider.subscribe_logs(&swap_filter).await?;
    let base_ws_stream = base_ws_subscription.into_stream().map(|log| ("base-mainnet", log)).boxed();

    // arb websocket stream
    let arb_ws_url = format!("wss://arb-mainnet.g.alchemy.com/v2/{}", api_key);
    let arb_ws_connect = WsConnect::new(&arb_ws_url);
    let arb_ws_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_ws(arb_ws_connect)
        .await?;
    let arb_ws_subscription = arb_ws_provider.subscribe_logs(&swap_filter).await?;
    let arb_ws_stream = arb_ws_subscription.into_stream().map(|log: alloy::rpc::types::Log| ("arb-mainnet", log)).boxed();


    let mut merged_stream = stream::select_all(vec![
        eth_ws_stream,
        base_ws_stream,
        arb_ws_stream,
    ]);



    io::stdout().flush()?;



    let mut token_info_map: HashMap<Address, TokenInfo> = HashMap::new();
    // let mut pool_address_to_index: HashMap<(String, Address), u16> = HashMap::new();
    // let mut pool_storage: Vec<PoolInfo> = Vec::new();

    let mut pool_info_map: HashMap<(String, Address), PoolInfo> = HashMap::new();

    let mut eth_swaps = 0;
    let mut base_swaps = 0;
    let mut arb_swaps = 0;

    //stream until interrupted

    while let Some((network, log)) = merged_stream.next().await {
        if let Ok(decode) = log.log_decode::<Swap>() {


            let provider = match network {
                "eth-mainnet" => {
                    eth_swaps += 1;
                    eth_provider.clone()
                }
                "base-mainnet" => {
                    base_swaps += 1;
                    base_provider.clone()
                }
                "arb-mainnet" => {
                    arb_swaps += 1;
                    arb_provider.clone()
                }
                _ => {
                    continue;
                }
            };


            let pool_address = log.address();

            match process_swap_event(
                &log,
                provider,
                network,  
                &mut token_info_map,
                &mut pool_info_map,
                &api_key
            ).await {
                Ok((Some(updated_pool), Some(updated_token))) => {
                    // Only send on success
                    tx.send(BackendUpdate::PoolUpdated(updated_pool)).await?;
                    tx.send(BackendUpdate::TokenUpdated(updated_token)).await?;
                },
                Ok((Some(updated_pool), None)) => {
                    tx.send(BackendUpdate::PoolUpdated(updated_pool)).await?;
                },
                Ok((None, Some(updated_token))) => {
                    tx.send(BackendUpdate::TokenUpdated(updated_token)).await?;
                },
                Ok((None, None)) => {
                    // do no
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

            tx.send(BackendUpdate::ChainStats {
                eth_swaps,
                base_swaps,
                arb_swaps,
            }).await?;
        }
    }

    Ok(())
}



