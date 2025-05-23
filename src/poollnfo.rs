use std::ops::Add;

use alloy::core::primitives::Address;


#[derive(Debug)]
pub struct PoolInfo {
    pool_address: Address,
    pool_name: String,
    token0: Address,
    token1: Address,
    token0_symbol: String,
    token1_symbol: String,
    swaps_tracked: usize,
    fee: u32,

    // // needed
    // fee: u32,
    // tick_spacing: i32,
    // swaps_tracked: usize,        // number of swaps seen
    // last_price: Option<f64>,     // derived from sqrtPriceX96
}

impl PoolInfo {
    pub fn new(
        pool_address: Address,
        token0: Address,
        token1: Address,
        token0_symbol: String,
        token1_symbol: String,
        swaps_tracked: usize,
        fee: u32,    
    ) -> Self {
        let fee_percent = fee as f64 / 10000.0;

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
            token0_symbol,
            token1_symbol,
            fee,
            swaps_tracked,
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