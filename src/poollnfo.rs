use std::ops::{Add, DerefMut};

use alloy::core::primitives::{Address, U160};

use crate::tokenInfo::TokenInfo;


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
    liquidity: u128,
    tick_range: (i32, i32),

    // // needed
    // last_price: Option<f64>,     // derived from sqrtPriceX96
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
        liquidity: u128,
        tick_range: (i32, i32),
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
            liquidity,
            tick_range,
        }
    }

    pub fn increment_swap_count(&mut self) {
        self.swaps_tracked += 1;
    }

    pub fn get_swap_count(&self) -> &usize {
        &self.swaps_tracked
    }

    pub fn get_pool_name(&self) -> &str {
        &self.pool_name
    }

    pub fn get_current_price(&self) -> f64 {
        self.current_price
    }

    pub fn update_current_price(&mut self, new_price: f64) {
        self.current_price = new_price;
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

    pub fn get_fee_percent(&self) -> f64 {
        self.fee as f64 / 10000.0
    }
}