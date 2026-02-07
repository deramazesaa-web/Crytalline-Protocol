// src/deontic_engine.rs
use crate::axiomatics::{AxiomaticEngine, LogicPartition};

/// ActionStatus defines the result of a deontic evaluation.
pub enum ActionStatus {
    Allowed,
    Forbidden,
}

/// A Norm represents a rule: Obligation, Permission, or Prohibition.
pub struct Norm {
    pub description: String,
}

/// The DeonticEngine enforces normative rules on top of axiomatic truths.
pub struct DeonticEngine {
    pub axiomatics: AxiomaticEngine,
    pub norms: Vec<Norm>,
}

impl DeonticEngine {
    pub fn new() -> Self {
        Self {
            axiomatics: AxiomaticEngine::new(),
            norms: Vec::new(),
        }
    }

    pub fn add_norm(&mut self, norm: Norm) {
        self.norms.push(norm);
    }

    /// Verifies if an action is compliant with both axioms and norms.
    pub fn check_compliance(&self, action: &str) -> ActionStatus {
        // 1. First, check the Axiom of Regularity via the AxiomaticEngine
        let is_axiomatic = self.axiomatics.verify_regularity(action);
        
        // 2. Then check if the action is explicitly forbidden
        let is_forbidden = action.contains("FORBIDDEN") || action.contains("illegal");

        if is_axiomatic && !is_forbidden {
            ActionStatus::Allowed
        } else {
            ActionStatus::Forbidden
        }
    }
}
