use alloy::core::primitives::Address;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInfo {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
}

impl TokenInfo {
    pub fn new(address: Address, symbol: String, decimals: u8) -> Self {
        Self { address, symbol, decimals }
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }
    
    pub fn get_decimals(&self) -> u8 {
        self.decimals
    }
}