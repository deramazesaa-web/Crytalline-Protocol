// src/main.rs

mod axiomatics;
mod deontic_engine;
mod ethical_rules;
mod proof;
mod resolver;
mod market_data;
mod errors;

use crate::axiomatics::AxiomaticState;
use crate::deontic_engine::{DeonticModality, Rule};
use crate::resolver::ConflictResolver;
use crate::market_data::MarketOracle;
use crate::proof::ProofGenerator;

fn main() {
    println!("--------------------------------------------------");
    println!("   CRYSTALLINE PROTOCOL v0.1.0 (Genesis Node)     ");
    println!("   Status: Operational | Mode: Multi-Epoch        ");
    println!("--------------------------------------------------");

    // Initialize core components
    let mut axiomatic_system = AxiomaticState::new();
    let mut oracle = MarketOracle::new();
    let resolver = ConflictResolver::new();

    // Simulate 3 business cycles (Epochs) to see the system in motion
    for epoch in 1..=3 {
        println!("\n>>> STARTING EPOCH #{} <<<", epoch);

        // 1. Fetch live market data from the Oracle
        let price = oracle.fetch_price("ETH");
        let market_is_risky = oracle.is_volatile();

        if market_is_risky {
            println!("[WARNING] High volatility detected in Epoch #{}", epoch);
        }

        // 2. Define competing rules based on current market sentiment
        let rule_executive = Rule {
            id: 100 + epoch,
            description: format!("Execute Trade at ${:.2}", price),
            modality: DeonticModality::Obligatory,
            priority: if market_is_risky { 40 } else { 90 }, // Trade priority drops during volatility
        };

        let rule_security = Rule {
            id: 200 + epoch,
            description: "Safety Buffer Activation".to_string(),
            modality: DeonticModality::Prohibited,
            priority: 75, // Security has a constant high threshold
        };

        // 3. Resolve conflict using the Axiom of Choice
        match resolver.resolve(&rule_executive, &rule_security) {
            Ok(resolution) => {
                println!("[RESOLVER] Decision: {} (via {})", 
                    resolution.winning_rule.description, resolution.resolved_by);
                
                // 4. Commit the winning rule to the global axiomatic state
                axiomatic_system.commit_rule(resolution.winning_rule.clone());

                // 5. Generate and verify a cryptographic proof
                let proof = ProofGenerator::generate_proof(
                    &resolution.winning_rule, 
                    &axiomatic_system
                );
                
                if ProofGenerator::verify_proof(&proof, &axiomatic_system) {
                    println!("[SUCCESS] Proof verified: {}", proof.witness_hash);
                } else {
                    println!("[CRITICAL] Proof verification failed for witness {}", proof.witness_hash);
                }
            },
            Err(e) => {
                println!("[ERROR] Logic failure in Epoch #{}: {:?}", epoch, e);
            }
        }
    }

    // Final summary of the protocol's state
    println!("\n--------------------------------------------------");
    println!("   FINAL SYSTEM STATE SUMMARY                    ");
    axiomatic_system.get_status();
    println!("--------------------------------------------------");
}
