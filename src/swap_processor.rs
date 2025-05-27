
use alloy::{core::primitives::{Address, U160, U256}, providers::{Provider}};
use anyhow::Result;
use serde_json::error;
use std::{collections::HashMap};
use amms::amms::uniswap_v3::{IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap};
// use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;


// other file imports
use crate::ierc20::IERC20;
use crate::poollnfo::PoolInfo;
use crate::tokenInfo::TokenInfo;

pub async fn process_swap_event<P: Provider + Clone> (
    log: &alloy::rpc::types::Log,
    provider: P,
    token_info_map: &mut HashMap<Address, TokenInfo>,
    pool_address_to_index: &mut HashMap<Address, u16>,
    pool_storage: &mut Vec<PoolInfo>,
) -> Result<()> {
    let data_bytes = &log.data().data;  // Access the bytes field directly
    
    if data_bytes.len() < 128 {  // Check length on bytes
        return Err(anyhow::anyhow!("Swap event data too short: {}", data_bytes.len()));
    }
    
    // Wrap critical sections in better error handling
    let pool_address = log.address();

    let contract = IUniswapV3PoolInstance::new(pool_address, provider.clone());

    let swap_event = match log.log_decode::<Swap>() {
        Ok(event) => {
            event
        },
        Err(error) => {

            panic!("{}", error);
        }
    };

    // let swap_event= log.log_decode::<Swap>()?; // possible error


    let amount0 = i128::try_from(swap_event.data().amount0).unwrap_or_default();
    let amount1 = i128::try_from(swap_event.data().amount1).unwrap_or_default();
    let sqrt_price_x96: U160 = swap_event.data().sqrtPriceX96.into();
    let liquidity = swap_event.data().liquidity;
    let tick = match swap_event.data().tick.try_into() {
        Ok(t) => t,
        Err(_) => {
            // Log this issue
            println!("Warning: Missing tick in swap event for pool {}", pool_address);
            0 // Default tick, which will give a price of 1.0001^0 = 1.0
        }
    };

    // acquire token addresses + symbols
    let token0_address = contract.token0().call().await?;
    let token1_address = contract.token1().call().await?;
    if !token_info_map.contains_key(&token0_address) {
        let ierc20_token0 = IERC20::new(token0_address, provider.clone());
        let token0_symbol = ierc20_token0.symbol().call().await?;
        let token0_decimal = ierc20_token0.decimals().call().await?;
        token_info_map.insert(token0_address, TokenInfo::new(token0_address, token0_symbol, token0_decimal));
    }
    if !token_info_map.contains_key(&token1_address) {
        let ierc20_token1 = IERC20::new(token1_address, provider.clone());
        let token1_symbol = ierc20_token1.symbol().call().await?;
        let token1_decimal = ierc20_token1.decimals().call().await?;
        token_info_map.insert(token1_address, TokenInfo::new(token1_address, token1_symbol, token1_decimal));
    }



    // add structs to pool storage (new pool)
    if !pool_address_to_index.contains_key(&pool_address) {
        process_new_pool(pool_address, token0_address, token1_address, provider, contract, token_info_map, pool_address_to_index, pool_storage,amount0, amount1, sqrt_price_x96, liquidity, tick).await?;
    } else {
        update_existing_pool(pool_address, pool_address_to_index, pool_storage, amount0, amount1, sqrt_price_x96, liquidity, tick)?;
    }

    Ok(())
}

async fn process_new_pool<P: Provider + Clone>(
    pool_address: Address,
    token0_address: Address,
    token1_address: Address,
    provider: P,
    contract: IUniswapV3PoolInstance<P>,
    token_info_map: &mut HashMap<Address, TokenInfo>,
    pool_address_to_index: &mut HashMap<Address, u16>,
    pool_storage: &mut Vec<PoolInfo>,
    amount0: i128,
    amount1: i128,
    sqrt_price_x96: U160,
    liquidity: u128,
    tick: i32,
) -> Result<()> {
    pool_address_to_index.insert(pool_address, pool_storage.len() as u16);
    
    let fee_uint = contract.fee().call().await?;
    let fee = fee_uint.to::<u32>();

    // calculate current price for pool
    let token0_decimals = token_info_map.get(&token0_address).unwrap().get_decimals();
    let token1_decimals = token_info_map.get(&token1_address).unwrap().get_decimals();
    let current_price = calculate_price_from_sqrt_price_x96(sqrt_price_x96, token0_decimals, token1_decimals);

    let tick_spacing = contract.tickSpacing().call().await?.as_i32();
    let tick_range = calculate_tick_range(tick, tick_spacing);

    let apr = calculate_apr(fee, 1, liquidity);
    let amount0_scaled = amount0.abs() as f64 / 10f64.powi(token0_decimals as i32);
    let amount1_scaled = amount1.abs() as f64 / 10f64.powi(token1_decimals as i32);
    let volume = amount0_scaled + amount1_scaled;

    let new_pool = PoolInfo::new(pool_address, token0_address, token1_address, token_info_map.get(&token0_address).unwrap().clone(), token_info_map.get(&token1_address).unwrap().clone(), 1, fee, current_price, 0.0, liquidity, tick_range, apr, volume);
    pool_storage.push(new_pool);

    Ok(())
}

fn update_existing_pool(
    pool_address: Address,
    pool_address_to_index: &HashMap<Address, u16>,
    pool_storage: &mut Vec<PoolInfo>,
    amount0: i128,
    amount1: i128,
    sqrt_price_x96: U160,
    liquidity: u128,
    tick: i32,
) -> Result<()> {
    if let Some(&index) = pool_address_to_index.get(&pool_address) {
        let pool = &mut pool_storage[index as usize];

        pool.increment_swap_count();

        // update price
        let token0_decimals = pool.get_token0_decimals();
        let token1_decimals = pool.get_token1_decimals();
        let new_price = calculate_price_from_sqrt_price_x96(sqrt_price_x96, token0_decimals, token1_decimals);
        let new_apr = calculate_apr(pool.get_fee(), pool.get_swap_count(), pool.get_liquidity());
        pool.update_current_price(new_price);
        pool.update_liquidity(liquidity);
        pool.update_current_apr(new_apr);

        // volume calc
        let amount0_scaled = amount0.abs() as f64 / 10f64.powi(token0_decimals as i32);
        let amount1_scaled = amount1.abs() as f64 / 10f64.powi(token1_decimals as i32);
        let volume = amount0_scaled + amount1_scaled;
        pool.add_volume(volume);

    } else {
        println!("Error: Pool found in HashMap but no index available");
    }

    Ok(())
}



pub fn calculate_price_from_sqrt_price_x96(
    sqrt_price_x96: U160,
    decimals_token0: u8,
    decimals_token1: u8,
) -> f64 {
    if sqrt_price_x96 == U160::ZERO {
        return 0.0;
    }

    // Convert U160 to U256 to preserve full 160-bit precision
    let mut limbs = [0u64; 4]; // Create an array of 4 u64 elements
    let u160_limbs = sqrt_price_x96.into_limbs();
    limbs[0] = u160_limbs[0];
    limbs[1] = u160_limbs[1];
    limbs[2] = u160_limbs[2];
    // limbs[3] is already 0

    let sqrt_price_u256 = U256::from_limbs(limbs);

    // Convert U256 to f64 via string to avoid truncation
    let sqrt_price_f64 = f64::from_str(&sqrt_price_u256.to_string()).unwrap_or(0.0);

    // Compute raw price = (sqrtPriceX96 / 2^96)^2
    let sqrt_scale = 2f64.powi(96);
    let price = (sqrt_price_f64 / sqrt_scale).powi(2);

    // Adjust for token decimals
    let decimal_adjustment = 10f64.powi((decimals_token0 as i32) - (decimals_token1 as i32));
    let adjusted_price = price * decimal_adjustment;

    // Sanity check and clamp
    if !adjusted_price.is_finite() || adjusted_price > 1e10 || adjusted_price < 1e-10 {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("price_calculation.log")
        {
            let _ = writeln!(
                file,
                "Extreme price: {} (sqrt: {}, raw: {}, adjustment: {})",
                adjusted_price,
                sqrt_price_u256,
                price,
                decimal_adjustment
            );
        }

        if adjusted_price > 1e10 {
            return 1e6;
        } else if adjusted_price < 1e-10 && adjusted_price > 0.0 {
            return 1e-6;
        } else {
            return 1.0;
        }
    }

    adjusted_price
}

pub fn calculate_tick_range(tick: i32, tick_spacing: i32) -> (i32, i32) {
    let lower_tick = tick - (tick % tick_spacing);
    let upper_tick = lower_tick + tick_spacing;
    (lower_tick, upper_tick)
}

pub fn calculate_daily_fees(fee: u32, swaps_tracked: usize) -> f64 {
    let fee_tier = fee as f64 / 1_000_000.0;
    let total_fees = swaps_tracked as f64 * fee_tier; 
    total_fees
}

pub fn calculate_apr(fee: u32, swaps_tracked: usize, liquidity: u128) -> f64 {
    let daily_fees = calculate_daily_fees(fee, swaps_tracked);
    if liquidity == 0 {
        return 0.0;
    }
    let scaled_liquidity = liquidity as f64 / 1e18;
    let apr = (daily_fees / scaled_liquidity as f64) * 365.0 * 100.0;
    apr
}

