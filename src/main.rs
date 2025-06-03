use anyhow::Result;
use backend_update::BackendUpdate;
use tokio::sync::mpsc;

// other file imports
mod ierc20;
mod pool_info;
use pool_info::PoolInfo;
mod ui;
use ui::TerminalUI;
mod swap_processor;
mod backend;
mod token_info;
mod prices;
use token_info::TokenInfo;
mod backend_update;

#[tokio::main]
async fn main() -> Result<()> {

    // max 100 swaps in channel
    let (tx, rx) = mpsc::channel::<BackendUpdate>(100);
    
    // start backend with separate task

    let backend_handle = tokio::spawn(async move {
        match crate::backend::run_ws_backend(tx).await {
            Ok(_) => println!("Backend finished successfully"),
            Err(e) => eprintln!("Backend error: {}", e),
        }
    });

    // initialize UI with receiver
    let mut terminal = ratatui::init();
    let ui_result = TerminalUI::with_receiver(rx).run(&mut terminal);
    ratatui::restore();
    
    // stop backend upon UI exit
    backend_handle.abort();
    
    ui_result

}