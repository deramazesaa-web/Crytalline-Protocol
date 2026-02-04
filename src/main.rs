// src/main.rs

// Module declarations
mod axiomatics;
mod deontic_engine;
mod ethical_rules;
mod proof;
mod resolver;
mod market_data;
mod errors;
use crate::errors::{CrystallineError, CrystallineResult};

// Imports
use crate::axiomatics::AxiomaticState;
use crate::deontic_engine::{DeonticModality, Rule};
use crate::resolver::{ConflictResolver, ConflictType};
use crate::market_data::MarketOracle;

fn main() {
    println!("--------------------------------------------------");
    println!("   CRYSTALLINE PROTOCOL v0.1.0 (Genesis Node)     ");
    println!("   System Boot: Initiating ZF-Axiomatic Logic     ");
    println!("--------------------------------------------------");

    // 1. Initialize System State
    let axiomatic_system = AxiomaticState::new();
    println!("[OK] Axiomatic Base Loaded. Depth: Recursive.");

    // 2. Simulate Market Data Ingestion
    let oracle = MarketOracle::new();
    let current_price = oracle.fetch_price("ETH");
    println!("[ORACLE] External feed active. Asset Price: ${}", current_price);

    // 3. Define Competing Governance Rules (Simulation)
    // Rule A: Market conditions suggest we MUST execute transaction
    let rule_executive = Rule {
        id: 101,
        description: "Execute Liquidity Provision".to_string(),
        modality: DeonticModality::Obligatory,
        priority: 80, // High priority based on market opportunity
    };

    // Rule B: Security protocol SAYS NO due to volatility
    let rule_security = Rule {
        id: 102,
        description: "Halt Trading (Volatility Protection)".to_string(),
        modality: DeonticModality::Prohibited,
        priority: 95, // Higher priority based on safety axiom
    };

    println!("\n[EVENT] Conflict Detected in Governance Queue:");
    println!("   -> Rule A: {} (Priority: {})", rule_executive.description, rule_executive.priority);
    println!("   -> Rule B: {} (Priority: {})", rule_security.description, rule_security.priority);

    // 4. Resolve Conflict via Axiom of Choice
    let resolver = ConflictResolver::new();
    
    // Handling the resolution with the new Error System
    match resolver.resolve(&rule_executive, &rule_security) {
        Ok(resolution) => {
            println!("\n[RESOLVER] Logic Processed: {}", resolution.resolved_by);
            println!("   -> WINNER: Rule ID #{}", resolution.winning_rule.id);
            
            // 4a. Commit to state
            axiomatic_system.commit_rule(resolution.winning_rule.clone()); 

            // 4b. Generate Proof (Step 1.2 & 2)
            let proof = crate::proof::ProofGenerator::generate_proof(
                &resolution.winning_rule, 
                &axiomatic_system
            );
            println!("[SYSTEM] Proof generated: {}", proof.witness_hash);

            // 4c. Immediate Verification check
            if crate::proof::ProofGenerator::verify_proof(&proof, &axiomatic_system) {
                println!("[SUCCESS] Decision verified by axiomatic verifier.");
            } else {
                println!("[FAILURE] Proof verification failed! State mismatch.");
            }
        },
        Err(e) => {
            // This catches violations like PriorityOverflow or AxiomaticParadox
            println!("\n[CRITICAL ERROR] Axiomatic violation detected: {:?}", e);
            println!("[SYSTEM] Transaction rejected to maintain logical consistency.");
        }
    }

    // 5. Verify Final System Status
    println!("\n[FINALIZING EPOCH]");
    axiomatic_system.get_status();
