use serde::{Serialize, Deserialize};
// Match the file name exactly: axiomatics
use crate::axiomatics::{AxiomaticEngine, LogicPartition};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ActionStatus {
    Permitted,
    Prohibited,
    Obligatory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Norm {
    pub name: String,
    pub status: ActionStatus,
}

impl Norm {
    pub fn new(name: &str, status: ActionStatus) -> Self {
        Self {
            name: name.to_string(),
            status,
        }
    }
}

pub struct DeonticEngine {
    pub norms: Vec<Norm>,
}

impl DeonticEngine {
    pub fn new() -> Self {
        Self { norms: Vec::new() }
    }

    pub fn add_norm(&mut self, norm: Norm) {
        self.norms.push(norm);
    }

    pub fn check_compliance(&self, action: &str) -> ActionStatus {
        for norm in &self.norms {
            if norm.name == action {
                return norm.status.clone();
            }
        }
        ActionStatus::Permitted
    }
}
