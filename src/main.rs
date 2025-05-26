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
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};


// other file imports
mod ierc20;
use ierc20::IERC20;
mod poollnfo;
use poollnfo::PoolInfo;
mod ui;
use ui::TerminalUI;
mod swap_processor;
use swap_processor::process_swap_event;
mod backend;


// const BLOCKS_TO_TRACK: u64 = 1;

#[tokio::main]
async fn main() -> Result<()> {
    // UI implmentation
    // let mut terminal = ratatui::init();
    // let ui_result = TerminalUI::default().run(&mut terminal);
    // ratatui::restore();
    // ui_result



    // backend implementation
    let (tx, mut rx) = mpsc::channel::<Vec<PoolInfo>>(100);
    
    // Create a background task to print received data
    let receive_handle = tokio::spawn(async move {
        println!("Starting receiver...");
        // let mut count = 0;
        while let Some(pools) = rx.recv().await {
            // count += 1;
            // println!("RECEIVER: Got update #{}: {} pools", count, pools.len());
            // for pool in &pools {
            //     println!("RECEIVER:   - {}: {} swaps", pool.get_pool_name(), pool.get_swap_count());
            // }
            // Force flush stdout
            io::stdout().flush().unwrap();
        }
    });
    
    // Call the backend function with the sender
    let result = crate::backend::run_ws_backend(tx).await;
    
    // Cancel the receiver once backend is done
    receive_handle.abort();
    
    result

}