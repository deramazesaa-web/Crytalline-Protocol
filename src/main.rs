pub mod deontic_engine;
pub mod ethical_rules;
pub mod market_data; // Register the new module

use deontic_engine::DeonticEngine;
use ethical_rules::{bootstrap_crystalline_logic, calculate_risk_index};
use market_data::SimulatedOracle;

fn main() {
    println!("--- CRYSTALLINE PROTOCOL v0.3.0: ORACLE INTEGRATION ---");
    
    // 1. Initialize Components
    let mut engine = DeonticEngine::new();
    bootstrap_crystalline_logic(&mut engine);
    
    let oracle = SimulatedOracle::new("Starknet_Pragma_Feed");

    // 2. Simulate Scenario: MARKET CRASH
    println!("\n>>> SCENARIO: DETECTING MARKET ANOMALY");
    let current_state = oracle.fetch_live_state("CRASH");

    println!("[MARKET_DATA]");
    println!("  Volatility: {:.2}", current_state.market_volatility);
    println!("  Collateral: {:.2}", current_state.collateral_ratio);

    // 3. Define Action
    let target_profit = 1500.0;
    let risk_score = calculate_risk_index(target_profit, current_state.network_slippage);
    
    println!("[RISK_ASSESSMENT]");
    println!("  Calculated Risk Score: {:.4}", risk_score);

    // 4. Execute Logic
    let verdict = engine.check(target_profit, &current_state);

    println!("\n[LOGIC_KERNEL_OUTPUT]");
    for log in &verdict.logs {
        println!("  {}", log);
    }

    println!("\n[FINAL_CONSENSUS]");
    if verdict.is_allowed {
        println!("  ✅ TRANSACTION APPROVED (Priority Override)");
    } else {
        println!("  ❌ TRANSACTION BLOCKED (Safety Violation)");
    }
    
    println!("--------------------------------------------------");
}
