/*
 * CRYSTALLINE PROTOCOL: MAIN EXECUTION NODE
 * -----------------------------------------
 * This is the entry point that synthesizes:
 * Layer 0 (Axiomatics) + Layer 1 (Deontic/Ethics)
 */

mod axiomatics;
mod deontic_engine;
mod ethical_rules;

use crate::axiomatics::{AxiomaticEngine, LogicPartition};
use crate::deontic_engine::{DeonticEngine, DeonticStatus};
use crate::ethical_rules::EthicalRules;

fn main() {
    println!("--- Crystalline Protocol: Axiomatic Engine Starting ---");

    // 1. Setup our environment
    let engine = DeonticEngine::new();
    
    // Simulated Transaction Data
    let sender = "user_alpha";
    let receiver = "user_beta";
    let action = "transfer_funds";
    let impact = 100; // Positive ethical impact

    println!("Processing Action: '{}' from {} to {}...", action, sender, receiver);

    // 2. LAYER 0: Fundamental Invariant Check (Regularity)
    if !AxiomaticEngine::verify_regularity(sender, receiver) {
        println!("CRITICAL FAILURE: Axiom of Regularity Violated (Self-reference).");
        return;
    }
    println!("Layer 0: Regularity Verified [x ∉ x].");

    // 3. LAYER 1: Ethical Predicate P(x) (Specification)
    let is_ethically_sound = EthicalRules::predicate_p(action, impact);
    if !is_ethically_sound {
        println!("LOGICAL REJECTION: Action fails Predicate P(x). Partition: Set N (Forbidden).");
        return;
    }
    println!("Layer 1: Ethical Predicate P(x) Satisfied.");

    // 4. LAYER 1: Deontic Evaluation
    let partition = engine.evaluate_transition(sender, receiver);
    
    match partition {
        LogicPartition::Allowed => {
            println!("RESULT: Transaction mapped to Set M (Allowed). Executing...");
            println!("Final State Density: {}", EthicalRules::calculate_logical_density(impact));
        },
        LogicPartition::Conditional => {
            println!("RESULT: Transaction mapped to Set R (Conditional). Awaiting Witness π.");
        },
        LogicPartition::Forbidden => {
            println!("RESULT: Transaction mapped to Set N (Forbidden). Ontological Block applied.");
        }
    }

    println!("--- Execution Complete ---");
}
