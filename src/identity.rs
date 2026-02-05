use k256::SecretKey;
use rand_core::{OsRng, RngCore};

pub enum ProtocolMode {
    Secp256k1,
    Ed25519,
}

pub struct CrystallineIdentity {
    pub secret: [u8; 32],
    pub mode: ProtocolMode,
}

impl CrystallineIdentity {
    pub fn generate_new(mode: ProtocolMode) -> Self {
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        
        Self {
            secret: seed,
            mode,
        }
    }

    pub fn get_private_key_hex(&self) -> String {
        hex::encode(self.secret)
    }
}
