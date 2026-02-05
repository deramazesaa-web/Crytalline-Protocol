mod deontic_engine;
mod axiomatic_layer;

use deontic_engine::{DeonticEngine, Norm, ActionStatus};
use axiomatic_layer::AxiomaticLayer;

fn main() {
    println!("--- Crystalline Protocol: Logic is Physics Mode ---");

    // 1. Initialize the Axiomatic Foundation
    let axiom_foundation = AxiomaticLayer::new();
    if !axiom_foundation.verify_integrity() {
        panic!("Protocol Collapse: Axiomatic integrity violation.");
    }

    // 2. Initialize the Deontic Engine
    let mut engine = DeonticEngine::new();

    // 3. Define a normative constraint (Example: Transaction validation)
    let transaction_norm = Norm::new(
        "SecureTransfer",
        ActionStatus::Obligatory,
    );
    
    engine.add_norm(transaction_norm);

    // 4. Process a logical state through the engine
    let test_action = "Inbound_Data_Packet";
    let compliance = engine.check_compliance(test_action);

    println!("Action: {}", test_action);
    println!("Compliance Status: {:?}", compliance);
    
    // 5. Finalize the state
    println!("--- Crystalline Execution Successful ---");
}
