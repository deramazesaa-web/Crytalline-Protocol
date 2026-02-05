use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::Secp256k1;
use bech32::{self, ToBase32, Variant};
use rand_core::{OsRng, RngCore};

#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    mnemonic: String,
    address: String,
    pubkey: String,
}

fn main() {
    // 1. Generate 256-bit entropy
    let mut entropy = [0u8; 32];
    OsRng.fill_bytes(&mut entropy);

    // 2. Mock mnemonic (replace with real BIP-39 logic if needed)
    let mnemonic_phrase = "example word phrase generated for demonstration purpose only".to_string();

    // 3. Generate Keys
    let secret_key = k256::SecretKey::from_be32(&entropy).expect("Invalid length");
    let public_key = secret_key.public_key();
    let pubkey_hex = hex::encode(public_key.to_encoded_point(true).as_bytes());

    // 4. Generate Bech32 Address (CosmWasm style)
    let address_bytes = &entropy[0..20]; // Simple derivation for example
    let address = bech32::encode("wasm", address_bytes.to_base32(), Variant::Bech32)
        .expect("Encoding failed");

    // 5. Create Wallet Object
    let wallet = Wallet {
        mnemonic: mnemonic_phrase,
        address: address,
        pubkey: pubkey_hex,
    };

    // 6. Save to JSON
    let json_data = serde_json::to_string_pretty(&wallet).expect("JSON serialization failed");
    let mut file = File::create("wallet.json").expect("File creation failed");
    file.write_all(json_data.as_bytes()).expect("Write failed");

    println!("Success: Wallet saved to wallet.json");
}
