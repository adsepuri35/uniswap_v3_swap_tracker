
use alloy::{core::primitives::Address, providers::{Provider}};
use anyhow::Result;
use std::{collections::HashMap};
use amms::amms::uniswap_v3::{IUniswapV3Pool::IUniswapV3PoolInstance};
// use reqwest::Client;


// other file imports
use crate::ierc20::IERC20;
use crate::poollnfo::PoolInfo;

pub async fn process_swap_event<P: Provider + Clone> (
    log: &alloy::rpc::types::Log,
    provider: P,
    token_address_to_symbol: &mut HashMap<Address, String>,
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

    // acquire token addresses + symbols
    let token0_address = contract.token0().call().await?;
    let token1_address = contract.token1().call().await?;
    if !token_address_to_symbol.contains_key(&token0_address) {
        let ierc20_token0 = IERC20::new(token0_address, provider.clone());
        let token0_symbol = ierc20_token0.symbol().call().await?;
        token_address_to_symbol.insert(token0_address, token0_symbol);
    }
    if !token_address_to_symbol.contains_key(&token1_address) {
        let ierc20_token1 = IERC20::new(token1_address, provider.clone());
        let token1_symbol = ierc20_token1.symbol().call().await?;
        token_address_to_symbol.insert(token1_address, token1_symbol);
    }



    // add structs to pool storage (new pool)
    if !pool_address_to_index.contains_key(&pool_address) {
        process_new_pool(pool_address, token0_address, token1_address, provider, contract, token_address_to_symbol, pool_address_to_index, pool_storage).await?;
    } else {
        update_existing_pool(pool_address, pool_address_to_index, pool_storage)?;
    }

    Ok(())
}

async fn process_new_pool<P: Provider + Clone>(
    pool_address: Address,
    token0_address: Address,
    token1_address: Address,
    provider: P,
    contract: IUniswapV3PoolInstance<P>,
    token_address_to_symbol: &HashMap<Address, String>,
    pool_address_to_index: &mut HashMap<Address, u16>,
    pool_storage: &mut Vec<PoolInfo>,
) -> Result<()> {
    pool_address_to_index.insert(pool_address, pool_storage.len() as u16);

    let token0_symbol = token_address_to_symbol
        .get(&token0_address)
        .cloned()
        .unwrap_or_else(|| String::from("Unknown"));
        
    let token1_symbol = token_address_to_symbol
        .get(&token1_address)
        .cloned()
        .unwrap_or_else(|| String::from("Unknown"));

    
    let fee_uint = contract.fee().call().await?;
    let fee = fee_uint.to::<u32>();

    let new_pool = PoolInfo::new(pool_address, token0_address, token1_address, token0_symbol, token1_symbol, 1, fee);
    pool_storage.push(new_pool);

    Ok(())
}

fn update_existing_pool(
    pool_address: Address,
    pool_address_to_index: &HashMap<Address, u16>,
    pool_storage: &mut Vec<PoolInfo>,
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