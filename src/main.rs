use anyhow::Result;
use backendUpdate::BackendUpdate;
// use reqwest::Client;
use tokio::sync::mpsc;
use std::collections::HashMap;
use alloy::core::primitives::Address;


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
mod tokenInfo;
mod prices;
use tokenInfo::TokenInfo;
mod backendUpdate;

#[tokio::main]
async fn main() -> Result<()> {

    let (tx, rx) = mpsc::channel::<(BackendUpdate)>(100);
    
    // Start the backend in a separate task
    let backend_handle = tokio::spawn(async move {
        match crate::backend::run_ws_backend(tx).await {
            Ok(_) => println!("Backend finished successfully"),
            Err(e) => eprintln!("Backend error: {}", e),
        }
    });
    
    // Initialize the UI with the receiver
    let mut terminal = ratatui::init();
    let ui_result = TerminalUI::with_receiver(rx).run(&mut terminal);
    ratatui::restore();
    
    // When UI exits, stop the backend
    backend_handle.abort();
    
    ui_result

}