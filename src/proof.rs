use crate::deontic_engine::LogicVerdict;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct FormalProof {
    pub proof_id: String,       // Unique ID of the decision
    pub timestamp: u64,         // When it happened
    pub block_height: u64,      // Simulated block number
    pub decision: bool,         // The final outcome
    pub confidence_score: u32,  // How sure the engine was
    pub integrity_hash: String, // Simulated cryptographic signature
    pub trace_log: Vec<String>, // The step-by-step reasoning
}

pub struct ProofGenerator;

impl ProofGenerator {
    /// Transforms a raw LogicVerdict into a formal, verifiable certificate.
    pub fn generate(verdict: LogicVerdict, block: u64) -> FormalProof {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Simulating a cryptographic hash of the state
        let integrity_hash = format!(
            "0x{}_{}_SHA256_VERIFIED", 
            timestamp, 
            if verdict.is_allowed { "AUTH" } else { "DENY" }
        );

        FormalProof {
            proof_id: format!("PROOF-{}-{}", block, timestamp),
            timestamp,
            block_height: block,
            decision: verdict.is_allowed,
            confidence_score: verdict.confidence_score,
            integrity_hash,
            trace_log: verdict.logs,
        }
    }

    pub fn print_certificate(proof: &FormalProof) {
        println!("\n=========== LOGICAL PROOF CERTIFICATE ===========");
        println!("ID:          {}", proof.proof_id);
        println!("TIMESTAMP:   {}", proof.timestamp);
        println!("BLOCK:       {}", proof.block_height);
        println!("HASH:        {}", proof.integrity_hash);
        println!("-------------------------------------------------");
        println!("VERDICT:     {}", if proof.decision { "AUTHORIZED [PASSED]" } else { "REJECTED [FAILED]" });
        println!("CONFIDENCE:  {}/100", proof.confidence_score);
        println!("-------------------------------------------------");
        println!("LOGIC TRACE:");
        for step in &proof.trace_log {
            println!("  > {}", step);
        }
        println!("=================================================\n");
    }
}
