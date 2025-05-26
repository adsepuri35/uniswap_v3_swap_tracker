use std::ops::Add;

use alloy::core::primitives::Address;

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

    // price stats
    // current_price: f64,

    // // needed
    // tick_spacing: i32,
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
    ) -> Self {
        let fee_percent = fee as f64 / 10000.0;

        // generate pool name
        let token0_symbol = token0_info.get_symbol();
        let token1_symbol = token1_info.get_symbol();
        let pool_name = if token0_symbol.is_empty() || token1_symbol.is_empty() {
            format!("Pool-{:?}", pool_address)
        } else if token0_symbol < token1_symbol {
            format!("{}/{} ({}%)", token0_symbol, token1_symbol, fee_percent)
        } else {
            format!("{}/{} ({}%)", token1_symbol, token0_symbol, fee_percent)
        };

        PoolInfo {
            pool_address,
            pool_name,
            token0,
            token1,
            token0_info,
            token1_info,
            swaps_tracked,
            fee
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
}