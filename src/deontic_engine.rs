use crate::resolver::{ConflictResolver, ResolutionStrategy, ResolutionResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeonticModality {
    Obligation,   // Mandatory actions (O)
    Prohibition,  // Forbidden actions (P)
    Permission,   // Allowed actions (A)
}

pub struct Norm {
    pub id: &'static str,
    pub modality: DeonticModality,
    pub priority: u32, 
}

pub struct WorldState {
    pub collateral_ratio: f64,
    pub network_slippage: f64,
    pub market_volatility: f64,
}

pub struct LogicVerdict {
    pub is_allowed: bool,
    pub confidence_score: u32,
    pub logs: Vec<String>,
}

pub struct DeonticEngine {
    norms: Vec<Norm>,
    resolver: ConflictResolver,
}

impl DeonticEngine {
    /// Initializes a new engine with a specific arbitration strategy
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self { 
            norms: Vec::new(),
            resolver: ConflictResolver::new(strategy),
        }
    }

    /// Adds a formal norm to the engine's knowledge base
    pub fn add_norm(&mut self, id: &'static str, modality: DeonticModality, priority: u32) {
        self.norms.push(Norm { id, modality, priority });
    }

    /// Evaluates a proposed transaction against the current world state
    pub fn check(&self, profit: f64, state: &WorldState) -> LogicVerdict {
        let mut logs = Vec::new();
        let mut forbidden_weight = 0;
        let mut mandatory_weight = 0;

        logs.push("INITIALIZING_DEONTIC_EVALUATION".to_string());

        // 1. Scan for Prohibitions (e.g., Anti-Greed/Ethical filters)
        if state.network_slippage > 0.01 && profit > 0.0 {
            if let Some(n) = self.norms.iter().find(|n| n.id == "ETHICAL_MEV_GUARD") {
                forbidden_weight += n.priority;
                logs.push(format!("CONFLICT_DETECTED: {} (Weight: {})", n.id, n.priority));
            }
        }

        // 2. Scan for Obligations (e.g., Protocol Survival/Solvency)
        if state.collateral_ratio < 1.3 {
            if let Some(n) = self.norms.iter().find(|n| n.id == "SURVIVAL_AXIOM") {
                mandatory_weight += n.priority;
                logs.push(format!("URGENCY_DETECTED: {} (Weight: {})", n.id, n.priority));
            }
        }

        // 3. Delegation to the Conflict Resolver (Arbitration Phase)
        let resolution = self.resolver.resolve(mandatory_weight, forbidden_weight);
        
        let (is_allowed, reason) = match resolution {
            ResolutionResult::Allowed(msg) => (true, msg),
            ResolutionResult::Denied(msg) => (false, msg),
        };

        logs.push(format!("ARBITRATION_RESULT: {}", reason));

        LogicVerdict {
            is_allowed,
            confidence_score: if is_allowed { mandatory_weight } else { forbidden_weight },
            logs,
        }
    }
}
