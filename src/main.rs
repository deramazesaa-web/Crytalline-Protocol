pub mod deontic_engine;
pub mod ethical_rules;
pub mod market_data;
pub mod axiomatics;
pub mod resolver;
pub mod proof; // Register the final module

use deontic_engine::DeonticEngine;
use ethical_rules::{bootstrap_crystalline_logic, calculate_risk_index};
use market_data::SimulatedOracle;
use axiomatics::validate_state_integrity;
use resolver::ResolutionStrategy;
use proof::ProofGenerator;

fn main() {
    println!("--- CRYSTALLINE PROTOCOL v1.0: GENESIS RELEASE ---");
    println!("--- ARCHITECTURE: 5-LAYER LOGIC STACK ---\n");

    // 1. INITIALIZATION LAYER
    // We choose 'StandardWeighted' strategy for this block
    let mut engine = DeonticEngine::new(ResolutionStrategy::StandardWeighted);
    bootstrap_crystalline_logic(&mut engine);
    let oracle = SimulatedOracle::new("Starknet_Pragma_Feed");
    let current_block = 845221; // Simulated block height

    // 2. DATA LAYER (Oracle)
    println!(">>> [LAYER 1] ORACLE DATA FETCH");
    let raw_state = oracle.fetch_live_state("CRASH"); // Simulating a market crash
    println!("    State Captured: Volatility {:.2} | Collateral {:.2}", 
             raw_state.market_volatility, raw_state.collateral_ratio);

    // 3. AXIOMATIC LAYER (Set Theory)
    println!(">>> [LAYER 2] AXIOMATIC VALIDATION");
    let integrity = validate_state_integrity(&raw_state);
    if !integrity.is_valid {
        println!("    ❌ FATAL: State Axiom Violated. Proof generation aborted.");
        return;
    }
    println!("    ✅ Axioms Hold. State is within valid set.");

    // 4. LOGIC & RESOLUTION LAYER (Deontic Engine + Resolver)
    println!(">>> [LAYER 3 & 4] DEONTIC ENGINE EXECUTION");
    let target_profit = 1500.0;
    let risk = calculate_risk_index(target_profit, raw_state.network_slippage);
    println!("    Analyzing Action: Profit ${} | Risk Index {:.2}", target_profit, risk);

    let verdict = engine.check(target_profit, &raw_state);

    // 5. PROOF LAYER (Audit)
    println!(">>> [LAYER 5] PROOF GENERATION");
    let proof = ProofGenerator::generate(verdict, current_block);

    // Output the final certificate
    ProofGenerator::print_certificate(&proof);
}
