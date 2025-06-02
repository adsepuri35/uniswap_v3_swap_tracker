use alloy::core::primitives::Address;


#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8,
    pub value: Option<String>,
    pub prev_value: Option<String>,
    pub last_updated: String,
}

impl TokenInfo {
    pub fn new(address: Address, symbol: String, decimals: u8, value: Option<String>) -> Self {
        Self { 
            address, 
            symbol, 
            decimals,
            value,
            prev_value: None,
            last_updated: String::new(),
        }
    }

    pub fn update_value(&mut self, new_value: Option<String>) {
        self.prev_value = self.value.clone();
        self.value = new_value; 
    }

    pub fn get_price_change_color(&self) -> ratatui::style::Color {
        if let (Some(current), Some(previous)) = (&self.value, &self.prev_value) {
            if let (Ok(current), Ok(previous)) = (current.parse::<f64>(), previous.parse::<f64>()) {
                if current > previous {
                    return ratatui::style::Color::Green;
                } else if current < previous {
                    return ratatui::style::Color::Red;
                }
            }
        }
        ratatui::style::Color::White
    }
}