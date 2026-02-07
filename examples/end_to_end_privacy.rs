use crystalline_protocol::{AxiomaticEngine, DeonticEngine, ActionStatus};

fn main() {
    println!("--- [MODULE: E2E PRIVACY] ---");
    let engine = AxiomaticEngine::new();
    let mut deontic = DeonticEngine::new();

    let payload = "confidential_e2e_message";
    
    // Extensionality ensures the message hasn't been tampered with
    if engine.verify_extensionality(payload) {
        println!("Axiom of Extensionality: Integrity confirmed.");
        
        let compliance = deontic.check_compliance(payload);
        match compliance {
            ActionStatus::Allowed => println!("Privacy Check: Message cleared for transmission."),
            ActionStatus::Forbidden => println!("Privacy Check: Potential data leak detected."),
        }
    }
}
