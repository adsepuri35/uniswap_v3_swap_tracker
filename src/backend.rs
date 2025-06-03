use std::{fmt::Debug, io::{self, Write} };
use alloy::{core::primitives::Address, network::Ethereum, providers::{
        Identity, Provider, ProviderBuilder, RootProvider, WsConnect,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    }, rpc::types::Filter, sol_types::SolEvent};
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

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    _price: Option<String>,
}


#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AlchemyNetwork {
    BaseMainnet,
    ArbMainnet,
    EthMainnet,
}


impl Debug for AlchemyNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        std::fmt::Formatter::write_str(f, 
                match self {
            AlchemyNetwork::ArbMainnet => "arb-mainnet",
            AlchemyNetwork::BaseMainnet => "base-mainnet",
            AlchemyNetwork::EthMainnet => "eth-mainnet",
        })

    }
}


type AlchemyProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

async fn build_clients(
    networks: Vec<AlchemyNetwork>,
    api_key: &str,
) -> anyhow::Result<Vec<(AlchemyNetwork, AlchemyProvider, AlchemyProvider)>> {
    let mut providers = vec![];

    for network in networks {
        let http_url = format!("https://{:?}.g.alchemy.com/v2/{}", network, api_key);
        let http_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_http(http_url.parse()?);

        let ws_url = format!("wss://{:?}.g.alchemy.com/v2/{}", network, api_key);
        let ws_connect = WsConnect::new(ws_url);
        let ws_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_ws(ws_connect)
            .await
            .expect("should create client");

        providers.push((network, http_provider, ws_provider));
    }

    Ok(providers)
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

    let chain_prefixes: Vec<AlchemyNetwork> = vec![
        AlchemyNetwork::EthMainnet,
        AlchemyNetwork::BaseMainnet,
        AlchemyNetwork::ArbMainnet,
    ];

    let providers = build_clients(chain_prefixes, &api_key)
        .await
        .expect("Should build clients");

    let mut streams = vec![];
    let mut provider_map = HashMap::new();
    for (network, http, ws) in &providers {
        provider_map.insert(network.clone(), http);
        let subscription = ws.subscribe_logs(&swap_filter).await?;
        let stream = subscription
            .into_stream()
            .map(move |log| (network.clone(), log))
            .boxed();
        streams.push(stream);
    }

    
    // eth websocket stream
    // let eth_ws_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    // let eth_ws_connect = WsConnect::new(&eth_ws_url);
    // let eth_ws_provider = ProviderBuilder::new()
    //     .network::<Ethereum>()
    //     .connect_ws(eth_ws_connect)
    //     .await?;

    // let eth_ws_subcription = eth_ws_provider.subscribe_logs(&swap_filter).await?;
    // let eth_ws_stream = eth_ws_subcription.into_stream().map(|log| (AlchemyNetwork::EthMainnet, log)).boxed();

    // // base websocket stream
    // let base_ws_url = format!("wss://base-mainnet.g.alchemy.com/v2/{}", api_key);
    // let base_ws_connect = WsConnect::new(&base_ws_url);
    // let base_ws_provider = ProviderBuilder::new()
    //     .network::<Ethereum>()
    //     .connect_ws(base_ws_connect)
    //     .await?;
    // let base_ws_subscription = base_ws_provider.subscribe_logs(&swap_filter).await?;
    // let base_ws_stream = base_ws_subscription.into_stream().map(|log| (AlchemyNetwork::BaseMainnet, log)).boxed();

    // // arb websocket stream
    // let arb_ws_url = format!("wss://arb-mainnet.g.alchemy.com/v2/{}", api_key);
    // let arb_ws_connect = WsConnect::new(&arb_ws_url);
    // let arb_ws_provider = ProviderBuilder::new()
    //     .network::<Ethereum>()
    //     .connect_ws(arb_ws_connect)
    //     .await?;
    // let arb_ws_subscription = arb_ws_provider.subscribe_logs(&swap_filter).await?;
    // let arb_ws_stream = arb_ws_subscription.into_stream().map(|log: alloy::rpc::types::Log| (AlchemyNetwork::ArbMainnet, log)).boxed();

    // merged websocket stream
    let mut merged_stream = stream::select_all(streams);

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
                AlchemyNetwork::EthMainnet => {
                    eth_swaps += 1;
                    provider_map
                        .get(&AlchemyNetwork::EthMainnet)
                        .expect("Shold get provider")
                }
                AlchemyNetwork::BaseMainnet => {
                    base_swaps += 1;
                    provider_map
                        .get(&AlchemyNetwork::BaseMainnet)
                        .expect("Shold get provider")
                }
                AlchemyNetwork::ArbMainnet => {
                    arb_swaps += 1;
                    provider_map
                        .get(&AlchemyNetwork::ArbMainnet)
                        .expect("Shold get provider")
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