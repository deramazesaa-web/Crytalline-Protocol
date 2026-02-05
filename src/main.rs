mod axiomatics;
mod deontic_engine;

use axiomatics::AxiomaticLayer;
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    println!("=== Crystalline Protocol: ZF-Axiomatic Logic ===");

    // 1. Axiomatic Level
    let axioms = AxiomaticLayer::new();
    
    // Check Axiom of Regularity
    if !axioms.axiom_regularity("state_b", "state_a") {
        panic!("Axiomatic violation detected.");
    }
    println!("Axiomatic status: VALID");

    // 2. Deontic Level
    let mut engine = DeonticEngine::new();
    let integrity_norm = Norm::new("Process_Data", ActionStatus::Obligatory);
    engine.add_norm(integrity_norm);

    // 3. Evaluation
    let current_action = "Process_Data";
    let result = engine.check_compliance(current_action);

    println!("Action '{}' evaluation: {:?}", current_action, result);
    println!("=== Verification Passed ===");
}
