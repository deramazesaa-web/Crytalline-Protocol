mod identity;

use identity::{CrystallineIdentity, ProtocolMode};

fn main() {
    // Stage 1: Protocol Initialization
    println!("=== Crystalline Protocol v0.1.0 ===");
    
    // Stage 2: Identity Generation
    // We choose Secp256k1 for initial ecosystem compatibility
    let identity = CrystallineIdentity::generate_new(ProtocolMode::Secp256k1);
    
    // Stage 3: Output Metadata
    println!("Status: Active");
    println!("Security Mode: SECP256K1");
    println!("Node Identity (Secret): {}", identity.get_private_key_hex());
    println!("===================================");
    
    // Future Stage: Here we will add data encryption and transport
    println!("Ready for secure data transmission.");
}
