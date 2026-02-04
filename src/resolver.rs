// src/resolver.rs

use crate::deontic_engine::{DeonticModality, Rule};
use crate::errors::{CrystallineError, CrystallineResult};

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    LogicalContradiction,
    ResourceDeadlock,
    AxiomaticViolation,
}

pub struct ResolutionResult {
    pub winning_rule: Rule,
    pub conflict_type: ConflictType,
    pub resolved_by: String,
}

pub struct ConflictResolver {}

impl ConflictResolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resolve(&self, rule_a: &Rule, rule_b: &Rule) -> CrystallineResult<ResolutionResult> {
        // 1. Validate rules against priority bounds
        self.validate_rule(rule_a)?;
        self.validate_rule(rule_b)?;

        // 2. Direct contradiction check
        if self.is_direct_contradiction(rule_a, rule_b) {
            let winner = self.apply_axiom_of_choice(rule_a, rule_b);
            return Ok(ResolutionResult {
                winning_rule: winner,
                conflict_type: ConflictType::LogicalContradiction,
                resolved_by: "Axiom of Choice (Priority weights)".to_string(),
            });
        }

        // 3. Fallback: if no direct contradiction, take higher priority anyway
        Ok(ResolutionResult {
            winning_rule: self.apply_axiom_of_choice(rule_a, rule_b),
            conflict_type: ConflictType::ResourceDeadlock,
            resolved_by: "Standard Priority Selection".to_string(),
        })
    }

    fn validate_rule(&self, rule: &Rule) -> CrystallineResult<()> {
        if rule.priority > 1000 {
            return Err(CrystallineError::PriorityOverflow(rule.priority));
        }
        if rule.priority == 0 {
            return Err(CrystallineError::AxiomaticParadox("Zero priority is ontologically unstable".to_string()));
        }
        Ok(())
    }

    fn is_direct_contradiction(&self, a: &Rule, b: &Rule) -> bool {
        match (&a.modality, &b.modality) {
            (DeonticModality::Obligatory, DeonticModality::Prohibited) => true,
            (DeonticModality::Prohibited, DeonticModality::Obligatory) => true,
            _ => false,
        }
    }

    fn apply_axiom_of_choice(&self, a: &Rule, b: &Rule) -> Rule {
        if a.priority >= b.priority { a.clone() } else { b.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deontic_engine::DeonticModality;

    #[test]
    fn test_conflict_resolution_logic() {
        let resolver = ConflictResolver::new();
        
        let high_priority_rule = Rule {
            id: 1,
            description: "High Priority".to_string(),
            modality: DeonticModality::Obligatory,
            priority: 900,
        };

        let low_priority_rule = Rule {
            id: 2,
            description: "Low Priority".to_string(),
            modality: DeonticModality::Prohibited,
            priority: 100,
        };

        let result = resolver.resolve(&high_priority_rule, &low_priority_rule).unwrap();
        
        // Assert that the high priority rule always wins
        assert_eq!(result.winning_rule.id, 1);
    }

    #[test]
    fn test_priority_overflow_error() {
        let resolver = ConflictResolver::new();
        let invalid_rule = Rule {
            id: 99,
            description: "Invalid".to_string(),
            modality: DeonticModality::Obligatory,
            priority: 5000, // Above limit
        };
        let normal_rule = Rule {
            id: 1,
            description: "Normal".to_string(),
            modality: DeonticModality::Obligatory,
            priority: 500,
        };

        let result = resolver.resolve(&invalid_rule, &normal_rule);
        
        // Assert that it returns an Error, not a Result
        assert!(result.is_err());
    }
}
