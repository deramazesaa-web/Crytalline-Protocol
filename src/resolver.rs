#[derive(Debug, Clone)]
pub enum ResolutionStrategy {
    StandardWeighted, // Mathematical comparison of weights (W_obl >= W_pro)
    StrictSafety,     // Any prohibition blocks the action (Veto power)
}

pub struct ConflictResolver {
    strategy: ResolutionStrategy,
}

impl ConflictResolver {
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self { strategy }
    }

    /// Arbitrates between the necessity of action (Obligation) and the restriction (Prohibition).
    pub fn resolve(&self, obligation_weight: u32, prohibition_weight: u32) -> ResolutionResult {
        match self.strategy {
            ResolutionStrategy::StandardWeighted => {
                // Classic Logic: Higher priority wins
                if obligation_weight >= prohibition_weight {
                    ResolutionResult::Allowed("Obligation outweighs Prohibition".to_string())
                } else {
                    ResolutionResult::Denied("Prohibition weight too high".to_string())
                }
            },
            ResolutionStrategy::StrictSafety => {
                // Paranoid Logic: If forbidden at all, do not proceed
                if prohibition_weight > 0 {
                    ResolutionResult::Denied("Strict Safety Mode: Zero-tolerance for prohibitions".to_string())
                } else {
                    ResolutionResult::Allowed("No prohibitions detected".to_string())
                }
            }
        }
    }
}

pub enum ResolutionResult {
    Allowed(String),
    Denied(String),
}
