use crystalline_protocol::{Kernel, AxiomChecker, DeonticLogic};

/// SCENARIO: SECURE WHATSAPP TUNNELING
/// 
/// We are simulating a "Hostile Environment" (WhatsApp) where data must be 
/// mathematically sealed before it touches the application memory.
///
/// The Kernel applies 4 ZF-Axioms to ensure:
/// 1. Extensionality: The data identity is unique (No spoofing).
/// 2. Regularity: The data contains no recursive attack loops.
/// 3. Separation: The app can only access the "Public" subset, not the "Private" core.
/// 4. Choice: The system can deterministically select the valid decryption key.

fn main() {
    // 1. Initialize the Axiomatic Kernel
    let kernel = Kernel::new();
    println!("--- [SYSTEM BOOT] Crystalline Kernel v{} ---", kernel.version);
    println!("--- Mode: In-Situ Sovereignty (WhatsApp Wrapper) ---");
    println!("");

    // 2. Simulate Input Data
    // "CONFIDENTIAL" is the predicate required by the Axiom of Separation.
    // If this tag is missing, the Kernel mathematically cannot "see" the data.
    let raw_payload = "PAYLOAD_DATA_CONFIDENTIAL_WX99"; 
    
    // 3. User Identity Proof (The Key)
    // Must satisfy Deontic Logic (Not Forbidden, Not Revoked).
    let user_proof = "IDENTITY_VERIFIED_USER_01"; 

    // 4. The "Predicate" (Logic Filter)
    // We define a subset φ(x): "Allow access only if data is tagged CONFIDENTIAL"
    let separation_predicate = "CONFIDENTIAL";

    println!("[KERNEL] Intercepting Input Stream...");
    println!("[KERNEL] Applying ZF-Axioms (Extensionality, Regularity, Separation, Choice)...");

    // 5. The Core Verification Process
    let is_secure = kernel.verify_sovereignty(raw_payload, user_proof, separation_predicate);

    println!("");
    if is_secure {
        println!(">>> [SUCCESS] State Validated.");
        println!(">>> Axiom of Separation: ALLOWED (Subset condition met).");
        println!(">>> Axiom of Regularity: PASSED (No cycles detected).");
        println!(">>> Deontic Status: OBLIGATION_FULFILLED.");
        println!(">>> Action: Decrypting message locally via 'Blind Carrier' protocol.");
    } else {
        println!(">>> [FAILURE] Axiomatic Violation Detected.");
        println!(">>> The data structure contradicts the defined logical physics.");
        println!(">>> Action: MEMORY_WIPE initiated.");
    }
}
