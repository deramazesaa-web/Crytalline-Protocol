mod axiomatics;
mod deontic_engine;

// Fixed: Import AxiomaticEngine instead of AxiomaticLayer
use axiomatics::{AxiomaticEngine, LogicPartition};
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    println!("--- CRYSTALLINE KERNEL: ACTIVE ---");

    let mut engine = DeonticEngine::new();
    
    // Axiom of Separation check
    let partition = LogicPartition::new("ZONE_ALPHA_SECURE");
    println!("Partition [{}] initialized.", partition.label);

    let secure_norm = Norm::new("Axiomatic_Zero_Trust_Policy");
    engine.add_norm(secure_norm);
    
    let action_payload = "verify_data_integrity";
    let status = engine.check_compliance(action_payload);
    
    // Pattern match the result
    match status {
        ActionStatus::Allowed => {
            println!("Result: PROCEED. Logic validated.");
        },
        ActionStatus::Forbidden => {
            println!("Result: HALT. Contradiction detected.");
        },
    }
}
