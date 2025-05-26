
use alloy::{core::primitives::Address, providers::{Provider}};
use anyhow::Result;
use std::{collections::HashMap};
use amms::amms::uniswap_v3::{IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap};
// use reqwest::Client;


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

    let swap_event= log.log_decode::<Swap>()?;

    let amount0 = i128::try_from(swap_event.data().amount0).unwrap_or_default();
    let amount1 = i128::try_from(swap_event.data().amount1).unwrap_or_default();
    let sqrt_price_x96 = u128::try_from(swap_event.data().sqrtPriceX96).unwrap_or_default();
    let liquidity = swap_event.data().liquidity;
    let tick = i32::try_from(swap_event.data().tick).unwrap_or_default();



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
        process_new_pool(pool_address, token0_address, token1_address, provider, contract, token_info_map, pool_address_to_index, pool_storage).await?;
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
) -> Result<()> {
    pool_address_to_index.insert(pool_address, pool_storage.len() as u16);

    let token0_symbol = token_info_map.get(&token0_address).unwrap().get_symbol();
    let token0_decimal = token_info_map.get(&token0_address).unwrap().get_decimals();
        
    let token1_symbol = token_info_map.get(&token1_address).unwrap().get_symbol();
    let token1_decimal = token_info_map.get(&token1_address).unwrap().get_decimals();

    
    let fee_uint = contract.fee().call().await?;
    let fee = fee_uint.to::<u32>();

    let new_pool = PoolInfo::new(pool_address, token0_address, token1_address, token_info_map.get(&token0_address).unwrap().clone(), token_info_map.get(&token1_address).unwrap().clone(), 1, fee);
    pool_storage.push(new_pool);

    Ok(())
}

fn update_existing_pool(
    pool_address: Address,
    pool_address_to_index: &HashMap<Address, u16>,
    pool_storage: &mut Vec<PoolInfo>,
    amount0: i128,
    amount1: i128,
    sqrt_price_x96: u128,
    liquidity: u128,
    tick: i32,
) -> Result<()> {
    if let Some(&index) = pool_address_to_index.get(&pool_address) {
        let pool = &mut pool_storage[index as usize];
        
        // Update the pool with the new swap information
        pool.increment_swap_count();
    } else {
        println!("Error: Pool found in HashMap but no index available");
    }

    Ok(())
}



pub fn calculate_price_from_sqrt_price_x96(sqrt_price_x96: u128, token0_decimals: u8, token1_decimals: u8) -> f64 {
    // More numerically stable calculation
    let sqrt_price = sqrt_price_x96 as f64;
    let base = 2_f64.powi(96);
    
    // Square the sqrt price and divide by 2^192
    let raw_price = (sqrt_price / base) * (sqrt_price / base);
    
    // Adjust for token decimals
    let decimal_adjustment = 10_f64.powi((token1_decimals as i32) - (token0_decimals as i32));
    
    raw_price * decimal_adjustment
}

async fn get_token_decimals<P: Provider + Clone>(
    token_address: Address,
    provider: P
) -> Result<u8> {
    // Use the IERC20 interface you already have
    let token = IERC20::new(token_address, provider);
    
    // Standard ERC20 decimals() function call
    let decimals = token.decimals().call().await?;
    
    Ok(decimals)
}