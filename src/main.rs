pub mod deontic_engine;
pub mod ethical_rules;
pub mod market_data;
pub mod axiomatics; // Register new module

use deontic_engine::DeonticEngine;
use ethical_rules::{bootstrap_crystalline_logic, calculate_risk_index};
use market_data::SimulatedOracle;
use axiomatics::validate_state_integrity;

fn main() {
    println!("--- CRYSTALLINE PROTOCOL v0.4.0: AXIOMATIC CORE ---");
    
    // 1. Initialize Components
    let mut engine = DeonticEngine::new();
    bootstrap_crystalline_logic(&mut engine);
    let oracle = SimulatedOracle::new("Starknet_Pragma_Feed");

    // 2. Simulate Scenario: CORRUPTED DATA (Example of Axiom Failure)
    // Let's pretend the oracle sent garbage data (e.g., negative slippage)
    println!("\n>>> PHASE 1: INTEGRITY CHECK");
    
    // We modify the simulated state to trigger an axiom violation for demonstration
    // In a real run, this comes directly from 'oracle.fetch_live_state'
    let mut raw_state = oracle.fetch_live_state("STABLE");
    raw_state.network_slippage = -0.5; // IMPOSSIBLE VALUE

    let integrity = validate_state_integrity(&raw_state);

    if !integrity.is_valid {
        println!("❌ CRITICAL FAILURE: STATE INTEGRITY COMPROMISED");
        println!("   Violated Law: {}", integrity.violated_axiom.unwrap());
        println!("   Action: SYSTEM HALT. No logic evaluation performed.");
        return; // Stop the program here
    }
    
    // If we get here, the state is mathematically valid. Now we check Ethics.
    println!("✅ AXIOMS HOLD. Proceeding to Deontic Logic Engine...");

    // ... (Остальной код выполняется только если аксиомы верны)
    // Для демонстрации вернем нормальное состояние и продолжим
    let valid_state = oracle.fetch_live_state("CRASH");
    
    let target_profit = 1500.0;
    let verdict = engine.check(target_profit, &valid_state);

    println!("\n>>> PHASE 2: LOGIC EVALUATION");
    for log in &verdict.logs {
        println!("  {}", log);
    }

    if verdict.is_allowed {
        println!("  ✅ TRANSACTION APPROVED");
    } else {
        println!("  ❌ TRANSACTION BLOCKED");
    }
}
