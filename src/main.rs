pub mod deontic_engine;
pub mod ethical_rules;

use deontic_engine::{DeonticEngine, WorldState};
use ethical_rules::{bootstrap_crystalline_logic, calculate_risk_index};

fn main() {
    println!("--- CRYSTALLINE PROTOCOL: ADVANCED LOGIC KERNEL ---");
    println!("--- STATUS: BOOTSTRAPPING FORMAL SYSTEMS ---\n");

    let mut engine = DeonticEngine::new();
    bootstrap_crystalline_logic(&mut engine);

    // CASE: High-profit attempt during liquidation risk
    let current_state = WorldState {
        collateral_ratio: 1.25,  // Critical level
        network_slippage: 0.015, // High slippage
        market_volatility: 0.8,  // Extreme volatility
    };

    let target_profit = 1200.0;
    let risk = calculate_risk_index(target_profit, current_state.network_slippage);

    println!("[TRANSACTION_SCAN]");
    println!("Target Profit: ${:.2} | Calculated Risk Index: {:.2}", target_profit, risk);

    let verdict = engine.check(target_profit, &current_state);

    println!("\n[DECISION_ENGINE_TRACE]");
    if verdict.logs.is_empty() {
        println!("  No deontic conflicts detected.");
    } else {
        for log in &verdict.logs {
            println!("  {}", log);
        }
    }

    println!("\n[FINAL_VERDICT]");
    if verdict.is_allowed {
        println!("  ✅ AUTHORIZED: Survival obligation overrides ethical constraints.");
    } else {
        println!("  ❌ REJECTED: Ethical guardrails block toxic extraction.");
    }
    
    println!("  Confidence Score: {}", verdict.confidence_score);
    println!("\n--- EXECUTION_COMPLETE ---");
}
