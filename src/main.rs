use std::io::{self, Write};
use alloy::{core::primitives::Address, network::Ethereum, providers::{self, Provider, ProviderBuilder, WsConnect}, rpc::types::{Filter, Topic}, sol_types::SolEvent};
use anyhow::Result;
use dotenv::dotenv;
use std::{env, collections::HashMap, time::{Duration, Instant}};
use amms::amms::uniswap_v3::{IUniswapV3Factory::IUniswapV3FactoryInstance, IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap};
use futures::StreamExt;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
// use reqwest::Client;


// other file imports
mod ierc20;
use ierc20::IERC20;
mod poollnfo;
use poollnfo::PoolInfo;
mod ui;
use ui::TerminalUI;
mod swap_processor;
use swap_processor::process_swap_event;


const BLOCKS_TO_TRACK: u64 = 1;

#[tokio::main]
async fn main() -> Result<()> {
    // UI implmentation
    // let mut terminal = ratatui::init();
    // let app_result = TerminalUI::default().run(&mut terminal);
    // ratatui::restore();
    // app_result



    // backend implementation

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
        .network::<Ethereum>()
        .connect_http(url.parse()?);

    
    // websocket provider
    let ws_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    let ws_connect = WsConnect::new(&ws_url);
    let ws_provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect_ws(ws_connect)
        .await?;

    let events = vec![
        Swap::SIGNATURE_HASH
    ];

    let ws_filter = Filter::new().event_signature(events);

    let ws_subcription = ws_provider.subscribe_logs(&ws_filter).await?;

    let mut ws_stream = ws_subcription.into_stream();


    println!("Uniswap tracker starting...");
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

    let mut token_address_to_symbol: HashMap<Address, String> = HashMap::new();
    let mut pool_address_to_index: HashMap<Address, u16> = HashMap::new();
    let mut pool_storage: Vec<PoolInfo> = Vec::new();

    //stream until interrupted
    loop {
        match tokio::time::timeout(Duration::from_secs(3), ws_stream.next()).await {
            Ok(Some(log)) => {
                if let Ok(decode) = log.log_decode::<Swap>() {
                    let block_number = log.block_number;

                    println!("block number: {:?}", block_number);
                    println!("swap detected");

                    let swap = decode.data();

                    process_swap_event(
                        &log,
                        provider.clone(),
                        &mut token_address_to_symbol,
                        &mut pool_address_to_index,
                        &mut pool_storage
                    ).await?;
                }
            },
            Ok(None) => {
                println!("\nWebSocket stream ended unexpectedly");
                break;
            },
            Err(_) => {
                println!("=============================================");
                    for pool in &pool_storage {
                        println!("{:?} = {:?}" ,pool.get_pool_name(), pool.get_swap_count());
                    }
                println!("=============================================");
                io::stdout().flush();
            }
        }
    }

    // for pool in &pool_storage {
    //     println!("{:?} = {:?}" ,pool.get_pool_name(), pool.get_swap_count());
    // }


    Ok(())

}