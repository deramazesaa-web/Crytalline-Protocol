mod axiomatics;
mod deontic_engine;

use axiomatics::LogicPartition;
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    println!("--- CRYSTALLINE KERNEL: ACTIVE ---");

    let mut engine = DeonticEngine::new();
    
    // Create a secure partition
    let partition = LogicPartition::new("ZONE_ALPHA");
    println!("Partition [{}] is initialized.", partition.label);

    // Create and add a norm using the new() method
    let secure_norm = Norm::new("Process_Verified_Batch");
    engine.add_norm(secure_norm);
    
    let action_payload = "data_packet_01_verify";
    let status = engine.check_compliance(action_payload);
    
    // Now ActionStatus can be printed with {:?} because we added #[derive(Debug)]
    println!("Deontic Status for payload: {:?}", status);
    
    match status {
        ActionStatus::Allowed => {
            println!("Result: PROCEED. Constraints satisfied.");
        },
        ActionStatus::Forbidden => {
            println!("Result: HALT. Logic violation detected.");
        },
    }
}
