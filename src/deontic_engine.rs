use crate::axiomatics::AxiomaticEngine;

#[derive(Debug, PartialEq)] // Added Debug for println! and PartialEq for comparisons
pub enum ActionStatus {
    Allowed,
    Forbidden,
}

pub struct Norm {
    pub description: String,
}

impl Norm {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
        }
    }
}

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

    pub fn check_compliance(&self, action: &str) -> ActionStatus {
        let is_axiomatic = self.axiomatics.verify_regularity(action);
        let is_forbidden = action.contains("FORBIDDEN") || action.contains("unauthorized");

        if is_axiomatic && !is_forbidden {
            ActionStatus::Allowed
        } else {
            ActionStatus::Forbidden
        }
    }
}
