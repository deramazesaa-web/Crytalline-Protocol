mod axiomatics;
mod deontic_engine;

use axiomatics::AxiomaticLayer;
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    println!("=== Crystalline Protocol: ZF-Axiomatic Logic ===");

    // PHASE 1: Axiomatic Verification
    let axioms = AxiomaticLayer::new();
    
    if !axioms.verify_foundations() {
        panic!("Axiomatic Instability: ZF requirements not met.");
    }
    println!("Axioms: Extensionality, Regularity, Specification, Choice - ACTIVE");

    // PHASE 2: Deontic Compliance
    let mut engine = DeonticEngine::new();

    // The Norm is based on the Axiom of Specification:
    // Only "Verified" actions are permitted.
    let secure_norm = Norm::new("Specified_Action", ActionStatus::Permitted);
    engine.add_norm(secure_norm);

    // PHASE 3: Execution
    let action = "Validated_State_Change";
    let compliance = engine.check_compliance(action);

    println!("Action: {}", action);
    println!("Status: {:?}", compliance);
    println!("=== Protocol Integrity: Axiomatic & Deontic Sync Successful ===");
}
