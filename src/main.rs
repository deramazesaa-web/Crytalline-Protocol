// src/main.rs

// Module declarations
mod axiomatics;
mod deontic_engine;
mod ethical_rules;
mod proof;
mod resolver;
mod market_data;

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
    let resolution = resolver.resolve(&rule_executive, &rule_security);

    println!("\n[RESOLVER] Processing via Logic Gate...");
    
    match resolution.winning_rule {
        Some(rule) => {
            println!("   -> WINNER: Rule ID #{}", rule.id);
            println!("   -> ACTION: {}", rule.description);
            println!("   -> REASON: {}", resolution.resolved_by);
        },
        None => println!("   -> No conflict or tie."),
    }

    if let ConflictType::LogicalContradiction = resolution.conflict_type {
        println!("[ALERT] Logical Contradiction handled safely without system crash.");
    }

    // 5. Finalize
    println!("\n--------------------------------------------------");
    println!("   Block Validated. Waiting for next epoch...     ");
    println!("--------------------------------------------------");
}
