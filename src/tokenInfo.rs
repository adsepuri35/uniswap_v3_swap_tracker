use alloy::core::primitives::Address;


#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
    pub currency: String,
    pub value: Option<String>,
    pub last_updated: String,
}

impl TokenInfo {
    pub fn new(address: Address, symbol: String, decimals: u8, value: String) -> Self {
        Self { 
            address, 
            symbol, 
            decimals,
            currency: String::new(),
            value: None,
            last_updated: String::new(),
        }
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }
    
    pub fn get_decimals(&self) -> u8 {
        self.decimals
    }
}