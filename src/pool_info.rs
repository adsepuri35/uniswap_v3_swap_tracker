use alloy::core::primitives::{Address};

use crate::{backend::AlchemyNetwork, token_info::TokenInfo};


#[derive(Debug, Clone)]
pub struct PoolInfo {
    pub networks: Vec<AlchemyNetwork>,
    pub pool_address: Address,
    pub pool_name: String,
    pub token0: Address,
    pub token1: Address,
    pub token0_info: TokenInfo,
    pub token1_info: TokenInfo,
    pub swaps_tracked: usize,
    pub fee: u32,
    pub current_price: f64,
    pub prev_price: f64,
    pub liquidity: u128,
    pub tick_range: (i32, i32),
    pub current_apr: f64,
    pub volume: f64,
    pub swap_store: Vec<(f64, f64, String)> // amount0, amount1, time
}

impl PoolInfo {
    pub fn new(
        network: AlchemyNetwork,
        pool_address: Address,
        token0: Address,
        token1: Address,
        token0_info: TokenInfo,
        token1_info: TokenInfo,
        swaps_tracked: usize,
        fee: u32,    
        current_price: f64,
        prev_price: f64,
        liquidity: u128,
        tick_range: (i32, i32),
        current_apr: f64,
        volume: f64,
        swap_store: Vec<(f64, f64, String)>,
    ) -> Self {

        // generate pool name
        let token0_symbol = &token0_info.symbol;
        let token1_symbol = &token1_info.symbol;
        let pool_name = if token0_symbol.is_empty() || token1_symbol.is_empty() {
            format!("Pool-{:?}", pool_address)
        } else if token0_symbol < token1_symbol {
            format!("{}/{}", token0_symbol, token1_symbol)
        } else {
            format!("{}/{}", token1_symbol, token0_symbol)
        };

        PoolInfo {
            networks: vec![network],
            pool_address,
            pool_name,
            token0,
            token1,
            token0_info,
            token1_info,
            swaps_tracked,
            fee,
            current_price,
            prev_price,
            liquidity,
            tick_range,
            current_apr,
            volume,
            swap_store
        }
    }

    pub fn increment_swap_count(&mut self) {
        self.swaps_tracked += 1;
    }

    pub fn update_current_price(&mut self, new_price: f64) {
        self.prev_price = self.current_price;
        self.current_price = new_price;
    }

    pub fn get_price_change_color(&self) -> ratatui::style::Color {
        if self.current_price > self.prev_price && self.prev_price != 0.0 {
            ratatui::style::Color::Green
        } else if self.current_price < self.prev_price {
            ratatui::style::Color::Red
        } else {
            ratatui::style::Color::White
        }
    }

    pub fn get_token0_decimals(&self) -> u8 {
        self.token0_info.decimals
    }
    
    pub fn get_token1_decimals(&self) -> u8 {
        self.token1_info.decimals
    }

    pub fn update_liquidity(&mut self, new_liquidity: u128) {
        self.liquidity = new_liquidity;
    }

    pub fn get_fee_percent(&self) -> f64 {
        self.fee as f64 / 10000.0
    }

    pub fn update_current_apr(&mut self, new_apr: f64) {
        self.current_apr = new_apr
    }


    pub fn add_volume(&mut self, this_swap_volume: f64) {
        self.volume += this_swap_volume
    }

    pub fn add_swap_store(&mut self, amount0: f64, amount1: f64, timestamp: String) {
        const MAX_SWAP_HISTORY: usize = 80;

        self.swap_store.push((amount0, amount1, timestamp));

        if self.swap_store.len() > MAX_SWAP_HISTORY {
            self.swap_store = self.swap_store.split_off(self.swap_store.len() - MAX_SWAP_HISTORY);
        }
    }

    pub fn get_token0_symbol(&self) -> &str {
        &self.token0_info.symbol
    }

    pub fn get_token1_symbol(&self) -> &str {
        &self.token1_info.symbol
    }
}

