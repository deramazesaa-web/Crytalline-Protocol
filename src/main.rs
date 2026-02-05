mod deontic_engine;
mod axiomatic_layer;

use axiomatic_layer::AxiomaticLayer;
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    println!("=== Crystalline Protocol: Hierarchy of Logic ===");

    // PHASE 1: Axiomatic Layer (Fundamental Physics)
    // The system cannot exist if axioms are violated.
    let axioms = AxiomaticLayer::new();
    
    if !axioms.is_consistent() {
        panic!("CRITICAL FAILURE: Axiomatic inconsistency detected. System collapse.");
    }
    println!("Status: Axiomatic Layer Verified (Logic is Physics).");

    // PHASE 2: Deontic Layer (Legal/Normative Logic)
    // Once physics is stable, we apply normative rules.
    let mut deontic_engine = DeonticEngine::new();

    // Example: Defining a security norm based on the axiomatic state
    let security_norm = Norm::new(
        "Encrypted_Transmission",
        ActionStatus::Obligatory,
    );

    deontic_engine.add_norm(security_norm);

    // PHASE 3: Execution and Compliance
    let action = "Inbound_Packet_Stream";
    let compliance_result = deontic_engine.check_compliance(action);

    println!("Processing Action: '{}'", action);
    println!("Deontic Compliance: {:?}", compliance_result);
    
    println!("=== Protocol Lifecycle: Execution Completed ===");
}
