mod axiomatics;
mod deontic_engine;

use axiomatics::LogicPartition;
use deontic_engine::{DeonticEngine, Norm, ActionStatus};

fn main() {
    println!("--- CRYSTALLINE KERNEL: ACTIVE ---");
    println!("System Mode: Formal Axiomatic Verification");

    // Initialize the Deontic Engine (Integrates Axiomatic Engine internally)
    let mut engine = DeonticEngine::new();
    
    // 1. Axiom of Separation: Create a secure logical subset
    let partition = LogicPartition::new("ZONE_ALPHA_SECURE");
    println!("Status: Logic Partition [{}] initialized.", partition.label);

    // 2. Define a Deontic Norm (Rule)
    let security_norm = Norm { 
        description: "Zero-Trust Axiomatic Constraint".to_string() 
    };
    engine.add_norm(security_norm);
    
    // 3. Evaluate an Action (e.g., a WhatsApp message being processed)
    let action_payload = "data_packet_01_verify";
    println!("Processing payload: '{}'", action_payload);
    
    let status = engine.check_compliance(action_payload);
    
    // 4. Pattern matching for the final decision
    match status {
        ActionStatus::Allowed => {
            println!("Result: ACCESS_GRANTED. All axiomatic constraints satisfied.");
        },
        ActionStatus::Forbidden => {
            println!("Result: ACCESS_DENIED. Logical contradiction or policy violation.");
        },
    }
}
