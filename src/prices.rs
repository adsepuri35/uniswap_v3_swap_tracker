use alloy::{core::primitives::Address, providers::{Provider, ProviderBuilder}};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Debug)]
struct TokenPriceRequest {
    addresses: Vec<TokenAddress>,
}

#[derive(Serialize, Debug)]
struct TokenAddress {
    network: String,
    address: String,
}

#[derive(Deserialize, Debug)]
struct TokenPriceResponse {
    data: Vec<TokenPriceData>, // Match the "data" array in the JSON
}

#[derive(Deserialize, Debug)]
struct TokenPriceError {
    message: String,
}


#[derive(Deserialize, Debug)]
struct TokenPriceData {
    network: String,
    address: String,
    prices: Vec<TokenPrice>, // Match the "prices" array in the JSON
    error: Option<TokenPriceError>,   // Match the "error" field in the JSON
}

#[derive(Deserialize, Debug)]
struct TokenPrice {
    currency: String,
    value: String,           // Match the "value" field as a String
    lastUpdatedAt: String,   // Match the "lastUpdatedAt" field
}


// pass in network and token address
pub async fn get_token_price(
    network: String,
    token_address: Address,
    api_key: &str,
) -> Result<Option<String>> {
    let price_url = format!("https://api.g.alchemy.com/prices/v1/{}/tokens/by-address", api_key);

    let request_body = TokenPriceRequest {
        addresses: vec![TokenAddress {
            network,
            address: format!("{:?}", token_address),
        }],
    };

    let client = Client::new();

    let response = client
        .post(&price_url)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Failed to fetch token price: {}", response.status());
        return Ok(None);
    }

    // Log the response body for debugging
    let body_text = response.text().await?;
    // let file_result = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("response_log.txt");

    // if let Ok(mut file) = file_result {
    //     writeln!(file, "Response body: {}", body_text).ok();
    // }

    // Deserialize the response
    let price_response: TokenPriceResponse = serde_json::from_str(&body_text)?;

    // Extract the first price value
    if let Some(price_data) = price_response.data.first() {
        // Check if there is an error
        if let Some(error_obj) = &price_data.error {
            // println!("Error fetching price for token {}: {}", token_address, error_obj.message);
            return Ok(None);
        }

        // Check if the prices array is empty
        if price_data.prices.is_empty() {
            // println!("No price data available for token {}", token_address);
            return Ok(None);
        }

        // Extract the first price value
        if let Some(price) = price_data.prices.first() {
            return Ok(Some(price.value.clone())); // Return the value as a String
        }
    }

    Ok(None)
}