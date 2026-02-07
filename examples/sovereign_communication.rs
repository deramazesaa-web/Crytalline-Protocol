use crystalline_protocol::{AxiomaticEngine, DeonticEngine, LogicPartition, Norm, ActionStatus};

fn main() {
    println!("--- [MODULE: SOVEREIGN COMMUNICATIONS] ---");

    // Initialize core components
    let engine = AxiomaticEngine::new();
    let mut deontic = DeonticEngine::new();

    // 1. Create a logic partition (Axiom of Separation)
    let chat_partition = LogicPartition::new("SECURE_CHAT_v1");
    println!("Partition [{}] established.", chat_partition.label);

    // 2. Define the Sovereignty Norm
    let comm_norm = Norm::new("AXIOMATIC_ENCRYPTION_MANDATORY");
    deontic.add_norm(comm_norm);

    // 3. Simulate a message payload
    let raw_payload = "user_secret_message_001";
    println!("Intercepting payload: '{}'", raw_payload);

    // 4. Verify via Axiomatic Engine (Axiom of Regularity)
    if engine.verify_regularity(raw_payload) {
        println!("Check 1: Regularity verified.");

        // 5. Check Deontic Compliance
        let result = deontic.check_compliance(raw_payload);
        match result {
            ActionStatus::Allowed => {
                println!("Check 2: Policy matched.");
                println!("Final Status: TRANSMISSION GRANTED.");
            },
            ActionStatus::Forbidden => {
                println!("Check 2: Policy violation.");
                println!("Final Status: TRANSMISSION BLOCKED.");
            }
        }
    } else {
        println!("CRITICAL: Axiomatic integrity failure!");
    }
}
