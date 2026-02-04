// src/errors.rs

#[derive(Debug)]
pub enum CrystallineError {
    AxiomaticParadox(String),
    PriorityOverflow(u32),
    ValidationFailed(String),
    OracleTimeout,
}

pub type Result<T> = std::result::Result<T, CrystallineError>;
