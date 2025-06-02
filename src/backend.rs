use alloy::{
    network::Ethereum,
    primitives::map::HashMap,
    providers::{
        Identity, Provider, ProviderBuilder, RootProvider, WsConnect,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
    rpc::types::Filter,
    sol_types::SolEvent,
};
use amms::amms::uniswap_v3::IUniswapV3PoolEvents::Swap;
use anyhow::Result;
use dotenv::dotenv;
use futures::stream::{self, StreamExt};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{self, Write};
use std::{env, sync::Arc};
use tokio::sync::mpsc;

// other file imports
use crate::backend_update::BackendUpdate;
use crate::swap_processor::process_swap_event;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    _price: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AlchemyNetwork {
    BaseMainnet,
    ArbMainnet,
    EthMainnet,
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
        let eth_url = format!("https://{:?}.g.alchemy.com/v2/{}", network, api_key);
        let eth_ws = format!("wss://{:?}.g.alchemy.com/v2/{}", network, api_key);
        let eth_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_http(eth_url.parse()?);

        let connect = WsConnect::new(eth_ws);
        let ws_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_ws(connect)
            .await
            .expect("should create client");

        providers.push((network, eth_provider, ws_provider));
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
            return Err(anyhow::anyhow!(
                "Failed to load ALCHEMY_API_KEY from .env file"
            ));
        }
    };

    let events = vec![Swap::SIGNATURE_HASH];
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
    for (network, http, ws) in providers {
        let subscription = ws.subscribe_logs(&swap_filter).await?;
        provider_map.insert(network.clone(), http);
        let stream = subscription
            .into_stream()
            .map(move |log| (network.clone(), log))
            .boxed();
        streams.push(stream);
    }

    let mut merged_stream = stream::select_all(streams);

    io::stdout().flush()?;

    let mut token_info_map = HashMap::new();
    let mut pool_info_map = HashMap::new();
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
                network.clone(),
                &mut token_info_map,
                &mut pool_info_map,
                &api_key,
            )
            .await
            {
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
                        }
                        Err(file_err) => {
                            // If we can't open the file, write to stderr directly
                            eprintln!(
                                "Error processing swap: {} (couldn't open log: {})",
                                e, file_err
                            );
                        }
                    }
                }
            }

            tx.send(BackendUpdate::ChainStats {
                eth_swaps,
                base_swaps,
                arb_swaps,
            })
            .await?;
        }
    }

    Ok(())
}
