use crate::deontic_engine::WorldState;

pub struct SimulatedOracle {
    pub provider_name: String,
    pub is_connection_secure: bool,
}

impl SimulatedOracle {
    pub fn new(provider: &str) -> Self {
        Self {
            provider_name: provider.to_string(),
            is_connection_secure: true,
        }
    }

    /// Simulates fetching real-time data from a blockchain network
    pub fn fetch_live_state(&self, scenario_type: &str) -> WorldState {
        println!("[ORACLE] Fetching data from {}...", self.provider_name);
        
        match scenario_type {
            "CRASH" => WorldState {
                collateral_ratio: 1.15, // Dangerous
                network_slippage: 0.05, // High slippage
                market_volatility: 0.9, // Extreme volatility
            },
            "STABLE" => WorldState {
                collateral_ratio: 2.5,  // Safe
                network_slippage: 0.001, // Low slippage
                market_volatility: 0.1, // Low volatility
            },
            _ => WorldState {
                collateral_ratio: 1.5,
                network_slippage: 0.01,
                market_volatility: 0.5,
            }
        }
    }
}
