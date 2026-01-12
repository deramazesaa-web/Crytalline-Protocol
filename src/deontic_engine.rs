#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeonticModality {
    Obligation,
    Prohibition,
    Permission,
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
}

impl DeonticEngine {
    pub fn new() -> Self {
        Self { norms: Vec::new() }
    }

    pub fn add_norm(&mut self, id: &'static str, modality: DeonticModality, priority: u32) {
        self.norms.push(Norm { id, modality, priority });
    }

    pub fn check(&self, profit: f64, state: &WorldState) -> LogicVerdict {
        let mut logs = Vec::new();
        let mut forbidden_weight = 0;
        let mut mandatory_weight = 0;

        // Anti-Greed Analysis
        if state.network_slippage > 0.01 && profit > 0.0 {
            if let Some(n) = self.norms.iter().find(|n| n.id == "ETHICAL_MEV_GUARD") {
                forbidden_weight += n.priority;
                logs.push(format!("MODALITY_CRITICAL: {} triggered (Weight: {})", n.id, n.priority));
            }
        }

        // Solvency Analysis
        if state.collateral_ratio < 1.3 {
            if let Some(n) = self.norms.iter().find(|n| n.id == "SURVIVAL_AXIOM") {
                mandatory_weight += n.priority;
                logs.push(format!("MODALITY_CRITICAL: {} triggered (Weight: {})", n.id, n.priority));
            }
        }

        let is_allowed = mandatory_weight >= forbidden_weight;
        let score = if is_allowed { mandatory_weight } else { forbidden_weight };

        LogicVerdict {
            is_allowed,
            confidence_score: score,
            logs,
        }
    }
}
