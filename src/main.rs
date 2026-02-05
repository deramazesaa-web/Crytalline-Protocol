mod deontic_engine;
mod axiomatic_layer;

use axiomatic_layer::AxiomaticLayer;
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    // 1. Axiomatic Level (Fundamental Rules)
    let axiom_system = AxiomaticLayer::new();
    
    // Check if the world-state is logically possible
    if !axiom_system.validate_foundations() {
        panic!("Axiomatic violation: The logical foundation of the protocol is unstable.");
    }

    // 2. Deontic Level (System Behavior)
    let mut engine = DeonticEngine::new();

    // Define a rule: Data must be verified
    let integrity_norm = Norm::new(
        "Data_Integrity_Verification",
        ActionStatus::Obligatory,
    );

    engine.add_norm(integrity_norm);

    // 3. Evaluation
    let current_action = "Process_New_Block";
    let result = engine.check_compliance(current_action);

    println!("Axiomatic status: VALID");
    println!("Action '{}' evaluation: {:?}", current_action, result);
}
