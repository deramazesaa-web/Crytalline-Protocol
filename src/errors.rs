#[derive(Debug)]
pub enum CrystallineError {
    /// Violation of ZF Axioms (e.g., Extensionality or Regularity failure)
    AxiomaticViolation(String),
    
    /// Conflict with Deontic Norms (e.g., doing something Prohibited)
    DeonticConflict(String),
    
    /// Failure in Specification (Axiom of Specification failed to filter data)
    LogicalInconsistency(String),
}

pub type Result<T> = std::result::Result<T, CrystallineError>;
