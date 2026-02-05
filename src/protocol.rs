/// Universal trait for Crystalline security operations
pub trait CrystallineSecure {
    fn sign_data(&self, data: &[u8]) -> Vec<u8>;
    fn verify_data(&self, data: &[u8], signature: &[u8]) -> bool;
    fn encrypt_for_ecosystem(&self, data: &[u8], recipient_key: &[u8]) -> Vec<u8>;
}
