use alloy::core::primitives::Address;


#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
    pub value: Option<String>,
    pub last_updated: String,
}

impl TokenInfo {
    pub fn new(address: Address, symbol: String, decimals: u8, value: Option<String>) -> Self {
        Self { 
            address, 
            symbol, 
            decimals,
            value,
            last_updated: String::new(),
        }
    }
}