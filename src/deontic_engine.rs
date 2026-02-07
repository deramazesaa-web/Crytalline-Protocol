use crate::axiomatics::AxiomaticEngine;

// Added Debug so we can print the status in main.rs
#[derive(Debug)] 
pub enum ActionStatus {
    Allowed,
    Forbidden,
}

pub struct Norm {
    pub description: String,
}

impl Norm {
    pub fn new(desc: &str) -> Self {
        Self { description: desc.to_string() }
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
        // We use the engine to verify regularity (no loops)
        if self.axiomatics.verify_regularity(action) && !action.contains("illegal") {
            ActionStatus::Allowed
        } else {
            ActionStatus::Forbidden
        }
    }
}
