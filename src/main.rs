pub mod deontic_engine;
pub mod ethical_rules;
pub mod market_data;
pub mod axiomatics;
pub mod resolver; // Register module

use deontic_engine::DeonticEngine;
use ethical_rules::{bootstrap_crystalline_logic, calculate_risk_index};
use market_data::SimulatedOracle;
use axiomatics::validate_state_integrity;
use resolver::ResolutionStrategy;

fn main() {
    println!("--- CRYSTALLINE PROTOCOL v0.5.0: CONFLICT RESOLVER ---");
    
    // SETUP: We choose 'StandardWeighted' strategy (Survival > Ethics)
    // Try changing this to 'StrictSafety' to see the difference!
    let active_strategy = ResolutionStrategy::StandardWeighted;
    
    println!(">>> SYSTEM MODE: {:?}", active_strategy);

    let mut engine = DeonticEngine::new(active_strategy);
    bootstrap_crystalline_logic(&mut engine);
    let oracle = SimulatedOracle::new("Starknet_Pragma_Feed");

    // PHASE 1: AXIOMS
    let raw_state = oracle.fetch_live_state("CRASH");
    let integrity = validate_state_integrity(&raw_state);
    if !integrity.is_valid {
        println!("❌ SYSTEM HALT: Axiom violation.");
        return;
    }

    // PHASE 2: LOGIC
    let target_profit = 1500.0;
    println!(">>> Analyzing transaction in CRASH state...");
    
    let verdict = engine.check(target_profit, &raw_state);

    for log in &verdict.logs {
        println!("  {}", log);
    }

    println!("\n[FINAL DECISION]");
    if verdict.is_allowed {
        println!("  ✅ EXECUTE (Approved by Resolver)");
    } else {
        println!("  ❌ REJECT (Blocked by Resolver)");
    }
}
