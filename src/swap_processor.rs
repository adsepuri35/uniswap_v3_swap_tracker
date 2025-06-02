
use alloy::{core::primitives::{Address, U160, U256}, providers::{Provider}};
use anyhow::Result;
use std::{collections::HashMap};
use amms::amms::{uniswap_v3::{IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap}};
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use chrono::Local;

// other file imports
use crate::ierc20::IERC20;
use crate::pool_info::PoolInfo;
use crate::token_info::TokenInfo;
use crate::prices::get_token_price;

pub async fn process_swap_event<P: Provider + Clone> (
    log: &alloy::rpc::types::Log,
    provider: P,
    network: &str,
    token_info_map: &mut HashMap<Address, TokenInfo>,
    pool_info_map: &mut HashMap<Address, PoolInfo>,
    api_key: &str,
) -> Result<(Option<PoolInfo>, Option<TokenInfo>, Option<TokenInfo>)> {
    let data_bytes = &log.data().data;
    
    if data_bytes.len() < 128 {
        return Err(anyhow::anyhow!("Swap event data too short: {}", data_bytes.len()));
    }
    
    let pool_address = log.address();

    let contract = IUniswapV3PoolInstance::new(pool_address, provider.clone());

    let swap_event_log = match log.log_decode::<Swap>() {
        Ok(event) => {
            event
        },
        Err(error) => {

            panic!("{}", error);
        }
    };

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let amount0 = i128::try_from(swap_event_log.data().amount0).unwrap_or_default();
    let amount1 = i128::try_from(swap_event_log.data().amount1).unwrap_or_default();
    let sqrt_price_x96: U160 = swap_event_log.data().sqrtPriceX96.into();
    let liquidity = swap_event_log.data().liquidity;
    let tick = match swap_event_log.data().tick.try_into() {
        Ok(t) => t,
        Err(_) => {
            // log issue
            println!("Warning: Missing tick in swap event for pool {}", pool_address);
            0 //default tick -> 1.0001^0 = 1.0
        }
    };


    let mut updated_token0 = None;
    let mut updated_token1 = None;

    // token addresses + symbols
    let token0_address = contract.token0().call().await?;
    let token1_address = contract.token1().call().await?;
    if !token_info_map.contains_key(&token0_address) {
        let token_price = get_token_price(network.to_string(), token0_address, api_key).await?;
        let token_price_value = token_price.unwrap_or("Unknown".to_string());
        let ierc20_token0 = IERC20::new(token0_address, provider.clone());
        let token0_symbol = ierc20_token0.symbol().call().await?;
        let token0_decimal = ierc20_token0.decimals().call().await?;

        let new_token = TokenInfo {
            address: token0_address,
            symbol: token0_symbol,
            decimals: token0_decimal,
            value: Some(token_price_value),
            prev_value: None,
            last_updated: String::new(),
        };

        token_info_map.insert(token0_address, new_token.clone());
        updated_token0 = Some(new_token);
    } else {
        let curr_token_price = get_token_price(network.to_string(), token0_address, api_key).await?;
        let curr_token_price_value = curr_token_price.unwrap_or("Unknown".to_string());

        if let Some(token_info) = token_info_map.get_mut(&token0_address) {
            token_info.update_value(Some(curr_token_price_value)); // Use the update_value method
            updated_token0 = Some(token_info.clone());
        }
    }

    if !token_info_map.contains_key(&token1_address) {
        let token_price = get_token_price(network.to_string(), token0_address, api_key).await?;
        let token_price_value = token_price.unwrap_or("Unknown".to_string()); // Provide a default value
        let ierc20_token1 = IERC20::new(token1_address, provider.clone());
        let token1_symbol = ierc20_token1.symbol().call().await?;
        let token1_decimal = ierc20_token1.decimals().call().await?;

        let new_token = TokenInfo {
            address: token1_address,
            symbol: token1_symbol,
            decimals: token1_decimal,
            value: Some(token_price_value),
            prev_value: None,
            last_updated: String::new(),
        };
        token_info_map.insert(token1_address, new_token.clone());
        updated_token1 = Some(new_token);
    } else {
        let curr_token_price = get_token_price(network.to_string(), token1_address, api_key).await?;
        let curr_token_price_value = curr_token_price.unwrap_or("Unknown".to_string());

        if let Some(token_info) = token_info_map.get_mut(&token1_address) {
            token_info.update_value(Some(curr_token_price_value)); // Use the update_value method
            updated_token1 = Some(token_info.clone());
        }
    }

    //get updated pool
    let updated_pool = if !pool_info_map.contains_key(&pool_address) {
        Some(process_new_pool(network, pool_address, token0_address, token1_address, contract, token_info_map, pool_info_map, amount0, amount1, sqrt_price_x96, liquidity, tick, timestamp,).await?,)
    } else {
        Some(update_existing_pool(network, pool_address, pool_info_map, amount0, amount1, sqrt_price_x96, liquidity, timestamp,)?,)
    };

    Ok((updated_pool, updated_token0, updated_token1))
}

async fn process_new_pool<P: Provider + Clone>(
    network: &str,
    pool_address: Address,
    token0_address: Address,
    token1_address: Address,
    contract: IUniswapV3PoolInstance<P>,
    token_info_map: &mut HashMap<Address, TokenInfo>,
    pool_info_map: &mut HashMap<Address, PoolInfo>,
    amount0: i128,
    amount1: i128,
    sqrt_price_x96: U160,
    liquidity: u128,
    tick: i32,
    timestamp: String,
) -> Result<PoolInfo> {
    
    
    let fee_uint = contract.fee().call().await?;
    let fee = fee_uint.to::<u32>();

    // calculate current price for pool
    let token0_decimals = token_info_map.get(&token0_address).unwrap().decimals;
    let token1_decimals = token_info_map.get(&token1_address).unwrap().decimals;
    let current_price = calculate_price_from_sqrt_price_x96(sqrt_price_x96, token0_decimals, token1_decimals);

    let tick_spacing = contract.tickSpacing().call().await?.as_i32();
    let tick_range = calculate_tick_range(tick, tick_spacing);

    let apr = calculate_apr(fee, 1, liquidity);
    let amount0_scaled = amount0.abs() as f64 / 10f64.powi(token0_decimals as i32);
    let amount1_scaled = amount1.abs() as f64 / 10f64.powi(token1_decimals as i32);
    let volume = amount0_scaled + amount1_scaled;


    let readable_amount0 = make_amount_readable(amount0, token0_decimals);
    let readable_amount1 = make_amount_readable(amount1, token1_decimals);
    let swap_store = vec![(readable_amount0, readable_amount1, timestamp)];

    let simplified_network = simplify_network_name(network);
    let new_pool = PoolInfo::new(simplified_network.to_string(), pool_address, token0_address, token1_address, token_info_map.get(&token0_address).unwrap().clone(), token_info_map.get(&token1_address).unwrap().clone(), 1, fee, current_price, 0.0, liquidity, tick_range, apr, volume, swap_store);
    pool_info_map.insert(pool_address, new_pool.clone());

    Ok(new_pool)
}

fn update_existing_pool(
    network: &str,
    pool_address: Address,
    pool_info_map: &mut HashMap<Address, PoolInfo>,
    amount0: i128,
    amount1: i128,
    sqrt_price_x96: U160,
    liquidity: u128,
    timestamp: String,
) -> Result<PoolInfo> {
    if let Some(pool) = pool_info_map.get_mut(&pool_address) {

        pool.increment_swap_count();

        let simplified_network = simplify_network_name(network);
        if !pool.networks.contains(&simplified_network.to_string()) {
            pool.networks.push(simplified_network.to_string());
        }

        // update price
        let token0_decimals = pool.get_token0_decimals();
        let token1_decimals = pool.get_token1_decimals();
        let new_price = calculate_price_from_sqrt_price_x96(sqrt_price_x96, token0_decimals, token1_decimals);
        let new_apr = calculate_apr(pool.fee, pool.swaps_tracked, pool.liquidity);
        pool.update_current_price(new_price);
        pool.update_liquidity(liquidity);
        pool.update_current_apr(new_apr);

        // volume calc
        let amount0_scaled = amount0.abs() as f64 / 10f64.powi(token0_decimals as i32);
        let amount1_scaled = amount1.abs() as f64 / 10f64.powi(token1_decimals as i32);
        let volume = amount0_scaled + amount1_scaled;
        pool.add_volume(volume);


        let readable_amount0 = make_amount_readable(amount0, token0_decimals);
        let readable_amount1 = make_amount_readable(amount1, token1_decimals);
        pool.add_swap_store(readable_amount0, readable_amount1, timestamp);

        return Ok(pool.clone())

    } else {
        println!("Error: Pool found in HashMap but no index available");
    }

    Err(anyhow::anyhow!("Failed to update pool"))
}



pub fn calculate_price_from_sqrt_price_x96(
    sqrt_price_x96: U160,
    decimals_token0: u8,
    decimals_token1: u8,
) -> f64 {
    if sqrt_price_x96 == U160::ZERO {
        return 0.0;
    }

    let mut limbs = [0u64; 4];
    let u160_limbs = sqrt_price_x96.into_limbs();
    limbs[0] = u160_limbs[0];
    limbs[1] = u160_limbs[1];
    limbs[2] = u160_limbs[2];

    let sqrt_price_u256 = U256::from_limbs(limbs);

    let sqrt_price_f64 = f64::from_str(&sqrt_price_u256.to_string()).unwrap_or(0.0);

    let sqrt_scale = 2f64.powi(96);
    let price = (sqrt_price_f64 / sqrt_scale).powi(2);

    let decimal_adjustment = 10f64.powi((decimals_token0 as i32) - (decimals_token1 as i32));
    let adjusted_price = price * decimal_adjustment;

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


pub fn make_amount_readable(amount: i128, decimals: u8) -> f64 {
    let divisor = 10f64.powi(decimals as i32);
    amount as f64 / divisor
}

pub fn simplify_network_name(network: &str) -> &str {
    match network {
        "eth-mainnet" => "eth",
        "base-mainnet" => "base",
        "arb-mainnet" => "arb",
        _ => network,
    }
}