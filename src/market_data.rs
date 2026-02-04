// src/market_data.rs

use std::time::{SystemTime, UNIX_EPOCH};

pub struct MarketOracle {
    pub last_price: f64,
}

impl MarketOracle {
    pub fn new() -> Self {
        Self { last_price: 2500.0 } // Initial base price (e.g., ETH)
    }

    /// Simulates fetching a live price with random volatility
    pub fn fetch_price(&mut self, ticker: &str) -> f64 {
        // Simple pseudo-random generator based on system time
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let volatility = (start % 100) as f64 / 1000.0; // 0.0 to 0.1 range
        
        let direction = if start % 2 == 0 { 1.0 } else { -1.0 };
        self.last_price += self.last_price * volatility * direction;
        
        println!("[ORACLE] Fetched {} price: ${:.2}", ticker, self.last_price);
        self.last_price
    }

    /// Returns market condition based on volatility
    pub fn is_volatile(&self) -> bool {
        // If price is ends with an even digit, simulate high volatility for testing
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        seed % 2 == 0
    }
}
