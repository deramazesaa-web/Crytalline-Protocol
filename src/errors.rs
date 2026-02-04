// src/errors.rs

#[derive(Debug)]
pub enum CrystallineError {
    /// Found a logical loop or paradox (violation of Axiom of Regularity)
    AxiomaticParadox(String),
    /// Rule priority is out of allowed bounds (0-1000)
    PriorityOverflow(u32),
    /// Conflict that couldn't be resolved even by Axiom of Choice
    IrresolvableConflict(u32, u32),
    /// External data source is unreachable
    OracleFailure(String),
}

// Define a custom Result type for the entire protocol
pub type CrystallineResult<T> = std::result::Result<T, CrystallineError>;
