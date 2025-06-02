use std::{io::{self, Write}, };
use alloy::{core::primitives::Address, network::Ethereum, providers::{Provider, ProviderBuilder, WsConnect}, rpc::types::Filter, sol_types::SolEvent};
use anyhow::Result;
use dotenv::dotenv;
use std::{env, collections::HashMap,};
use amms::amms::uniswap_v3::IUniswapV3PoolEvents::Swap;
use tokio::sync::mpsc;
use std::fs::OpenOptions;
use std::io::BufWriter;
use futures::stream::{self, StreamExt};

// other file imports
use crate::pool_info::PoolInfo;
use crate::swap_processor::process_swap_event;
use crate::token_info::TokenInfo;
use crate::backend_update::BackendUpdate;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    _price: Option<String>,
}


pub async fn run_ws_backend(tx: mpsc::Sender<BackendUpdate>) -> Result<()> {
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

    // merged websocket stream
    let mut merged_stream = stream::select_all(vec![
        eth_ws_stream,
        base_ws_stream,
        arb_ws_stream,
    ]);

    io::stdout().flush()?;

    let mut token_info_map: HashMap<Address, TokenInfo> = HashMap::new();
    let mut pool_info_map: HashMap<Address, PoolInfo> = HashMap::new();
    let mut eth_swaps = 0;
    let mut base_swaps = 0;
    let mut arb_swaps = 0;

    //stream until interrupted

    while let Some((network, log)) = merged_stream.next().await {
        if let Ok(_decode) = log.log_decode::<Swap>() {

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

            match process_swap_event(
                &log,
                provider,
                network,  
                &mut token_info_map,
                &mut pool_info_map,
                &api_key
            ).await {
                Ok((updated_pool, updated_token0, updated_token1)) => {
                    // Send updates for pool and tokens if they exist
                    if let Some(pool) = updated_pool {
                        tx.send(BackendUpdate::PoolUpdated(pool)).await?;
                    }
                    if let Some(token0) = updated_token0 {
                        tx.send(BackendUpdate::TokenUpdated(token0)).await?;
                    }
                    if let Some(token1) = updated_token1 {
                        tx.send(BackendUpdate::TokenUpdated(token1)).await?;
                    }
                }
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