use alloy::core::primitives::Address;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::backend::AlchemyNetwork;

#[derive(Serialize, Debug)]
struct TokenPriceRequest {
    addresses: Vec<TokenAddress>,
}

#[derive(Serialize, Debug)]
struct TokenAddress {
    network: AlchemyNetwork,
    address: String,
}

#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    data: Vec<TokenPriceData>,
}

#[derive(Deserialize, Debug)]
struct TokenPriceError {
    _message: String,
}

#[derive(Deserialize, Debug)]
struct TokenPriceData {
    network: String,
    address: String,
    prices: Vec<TokenPrice>,
    error: Option<TokenPriceError>,
}

#[derive(Deserialize, Debug)]
struct TokenPrice {
    currency: String,
    value: String,
    lastUpdatedAt: String,
}

// pass in network and token address
pub async fn get_token_price(
    network: AlchemyNetwork,
    token_address: Address,
    api_key: &str,
) -> Result<Option<String>> {
    let price_url = format!(
        "https://api.g.alchemy.com/prices/v1/{}/tokens/by-address",
        api_key
    );

    let request_body = TokenPriceRequest {
        addresses: vec![TokenAddress {
            network,
            address: format!("{:?}", token_address),
        }],
    };

    let client = Client::new();

    let response = client.post(&price_url).json(&request_body).send().await?;

    if !response.status().is_success() {
        println!("Failed to fetch token price: {}", response.status());
        return Ok(None);
    }

    let body_text = response.text().await?;

    let price_response: TokenPriceResponse = serde_json::from_str(&body_text)?;

    // get first price value
    if let Some(price_data) = price_response.data.first() {
        if let Some(_error_obj) = &price_data.error {
            // println!("Error fetching price for token {}: {}", token_address, error_obj.message);
            return Ok(None);
        }

        // Check if the prices array is empty
        if price_data.prices.is_empty() {
            // println!("No price data available for token {}", token_address);
            return Ok(None);
        }

        // extract the first price value
        if let Some(price) = price_data.prices.first() {
            return Ok(Some(price.value.clone()));
        }
    }

    Ok(None)
}
