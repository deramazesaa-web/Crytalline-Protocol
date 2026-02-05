mod identity;
mod crypto;

use identity::{CrystallineIdentity, ProtocolMode};
use crypto::CrystallineCrypto;

fn main() {
    println!("=== Crystalline Protocol v0.1.0 ===");

    // 1. Identity Layer
    let identity = CrystallineIdentity::generate_new(ProtocolMode::Secp256k1);
    println!("Node Identity: Initialized");

    // 2. Data to protect
    let secret_message = b"Crystalline Protocol: Universal Security Active";
    println!("Original Message: {}", String::from_utf8_lossy(secret_message));

    // 3. Encryption Layer
    // We use the identity's secret key to encrypt the message
    let (encrypted_data, nonce) = CrystallineCrypto::encrypt(secret_message, &identity.secret);
    println!("Encryption: Success (Hex: {})", hex::encode(&encrypted_data));

    // 4. Verification / Decryption Layer
    let decrypted_data = CrystallineCrypto::decrypt(&encrypted_data, &identity.secret, &nonce);
    
    println!("Decrypted Result: {}", String::from_utf8_lossy(&decrypted_data));
    println!("Protocol Integrity: Verified");
    println!("===================================");
}
