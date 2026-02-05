use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use rand_core::{OsRng, RngCore};

pub struct CrystallineCrypto;

impl CrystallineCrypto {
    /// Encrypts data using AES-256-GCM (Industrial Grade Encryption)
    pub fn encrypt(data: &[u8], key: &[u8; 32]) -> (Vec<u8>, [u8; 12]) {
        let key_init = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
        
        // Generate a unique 12-byte nonce (initialization vector)
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = key_init
            .encrypt(nonce, data)
            .expect("Encryption failure");

        (ciphertext, nonce_bytes)
    }

    /// Decrypts data using the same key and nonce
    pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce_bytes: &[u8; 12]) -> Vec<u8> {
        let key_init = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
        let nonce = Nonce::from_slice(nonce_bytes);

        key_init
            .decrypt(nonce, ciphertext)
            .expect("Decryption failure - possibly wrong key or corrupted data")
    }
}
