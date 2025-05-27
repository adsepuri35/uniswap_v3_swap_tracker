use alloy::core::primitives::{Address};

use crate::tokenInfo::TokenInfo;
use amms::amms::uniswap_v3::{IUniswapV3PoolEvents::Swap};


#[derive(Debug, Clone)]
pub struct PoolInfo {
    pool_address: Address,
    pool_name: String,
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
    swap_store: Vec<(Swap, u64)>
    // stats to add: last swap (time), 
}

impl PoolInfo {
    pub fn new(
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
        swap_store: Vec<(Swap, u64)>,
    ) -> Self {

        // generate pool name
        let token0_symbol = token0_info.get_symbol();
        let token1_symbol = token1_info.get_symbol();
        let pool_name = if token0_symbol.is_empty() || token1_symbol.is_empty() {
            format!("Pool-{:?}", pool_address)
        } else if token0_symbol < token1_symbol {
            format!("{}/{}", token0_symbol, token1_symbol)
        } else {
            format!("{}/{}", token1_symbol, token0_symbol)
        };

        PoolInfo {
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

    pub fn get_swap_count(&self) -> usize {
        self.swaps_tracked
    }

    pub fn get_pool_name(&self) -> &str {
        &self.pool_name
    }

    pub fn get_current_price(&self) -> f64 {
        self.current_price
    }

    pub fn update_current_price(&mut self, new_price: f64) {
        self.prev_price = self.current_price;
        self.current_price = new_price;
    }

    pub fn get_price_change_color(&self) -> ratatui::style::Color {
        if self.current_price > self.prev_price && self.prev_price != 0.0 {
            ratatui::style::Color::Green // Price increased
        } else if self.current_price < self.prev_price {
            ratatui::style::Color::Red // Price decreased
        } else {
            ratatui::style::Color::White // No change
        }
    }

    pub fn get_token0_decimals(&self) -> u8 {
        self.token0_info.get_decimals()
    }
    
    pub fn get_token1_decimals(&self) -> u8 {
        self.token1_info.get_decimals()
    }

    pub fn get_liquidity(&self) -> u128 {
        self.liquidity
    }

    pub fn update_liquidity(&mut self, new_liquidity: u128) {
        self.liquidity = new_liquidity;
    }

    pub fn get_tick_range(&self) -> (i32, i32) {
        self.tick_range
    }

    pub fn get_fee(&self) -> u32 {
        self.fee
    }

    pub fn get_fee_percent(&self) -> f64 {
        self.fee as f64 / 10000.0
    }

    pub fn get_current_apr(&self) -> f64 {
        self.current_apr
    }

    pub fn update_current_apr(&mut self, new_apr: f64) {
        self.current_apr = new_apr
    }

    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    pub fn add_volume(&mut self, this_swap_volume: f64) {
        self.volume += this_swap_volume
    }

    pub fn add_swap_store(&mut self, new_swap: Swap, timestamp: u64) {
        self.swap_store.push((new_swap, timestamp));
    }

    pub fn get_swap_store(&self) -> &Vec<(Swap, u64)> {
        &self.swap_store
    }
}