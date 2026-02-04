// src/resolver.rs

use crate::deontic_engine::{DeonticModality, Rule};
// Assuming AxiomaticState is available or we model it here for resolution context
use crate::axiomatics::AxiomaticState;

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    LogicalContradiction, // Obligatory AND Prohibited simultaneously
    ResourceDeadlock,     // Two rules deadlock a single resource
    AxiomaticViolation,   // Rule violates a fundamental axiom
}

#[derive(Debug)]
pub struct ResolutionResult {
    pub winning_rule: Option<Rule>,
    pub conflict_type: ConflictType,
    pub resolved_by: String,
}

pub struct ConflictResolver {
    // Reference to global axiomatic state parameters
    axiomatic_depth: u32,
}

impl ConflictResolver {
    pub fn new() -> Self {
        Self { axiomatic_depth: 10 } // Recursion check depth
    }

    /// Main method for resolving disputes between two rules
    pub fn resolve(&self, rule_a: &Rule, rule_b: &Rule) -> ResolutionResult {
        // 1. Check for fundamental axiom violation (Layer 0)
        if !self.check_regularity(rule_a) {
            return ResolutionResult {
                winning_rule: Some(rule_b.clone()),
                conflict_type: ConflictType::AxiomaticViolation,
                resolved_by: "Axiom of Regularity (Rule A dropped)".to_string(),
            };
        }
        if !self.check_regularity(rule_b) {
            return ResolutionResult {
                winning_rule: Some(rule_a.clone()),
                conflict_type: ConflictType::AxiomaticViolation,
                resolved_by: "Axiom of Regularity (Rule B dropped)".to_string(),
            };
        }

        // 2. Determine conflict type
        if self.is_direct_contradiction(rule_a, rule_b) {
            // 3. Resolution via Axiom of Choice (weighted selection)
            let winner = self.apply_axiom_of_choice(rule_a, rule_b);
            return ResolutionResult {
                winning_rule: Some(winner),
                conflict_type: ConflictType::LogicalContradiction,
                resolved_by: "Axiom of Choice (Weight Priority)".to_string(),
            };
        }

        // If no conflict, return None (both rules valid, but we select one for order)
        ResolutionResult {
            winning_rule: None,
            conflict_type: ConflictType::LogicalContradiction, // Placeholder
            resolved_by: "No Conflict Detected".to_string(),
        }
    }

    /// Checks if the rule creates an infinite loop (violation of Axiom of Regularity)
    fn check_regularity(&self, rule: &Rule) -> bool {
        // In a real system, this would be a dependency graph check.
        // For MVP, we check that rule priority is within valid ontological bounds.
        rule.priority > 0 && rule.priority < 1000
    }

    /// Checks for direct logical collision (Obligatory vs Prohibited)
    fn is_direct_contradiction(&self, a: &Rule, b: &Rule) -> bool {
        match (a.modality.clone(), b.modality.clone()) {
            (DeonticModality::Obligatory, DeonticModality::Prohibited) => true,
            (DeonticModality::Prohibited, DeonticModality::Obligatory) => true,
            _ => false,
        }
    }

    /// Uses rule "Weight" (priority) as a deterministic selector via Axiom of Choice
    fn apply_axiom_of_choice(&self, a: &Rule, b: &Rule) -> Rule {
        if a.priority >= b.priority {
            a.clone()
        } else {
            b.clone()
        }
    }
}

// Logic verification tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::deontic_engine::DeonticModality;

    #[test]
    fn test_resolve_paradox() {
        let resolver = ConflictResolver::new();
        
        let rule_obligatory = Rule {
            id: 1,
            description: "Must pay tax".to_string(),
            modality: DeonticModality::Obligatory,
            priority: 50,
        };

        let rule_prohibited = Rule {
            id: 2,
            description: "Do not pay tax".to_string(),
            modality: DeonticModality::Prohibited,
            priority: 10, // Lower priority
        };

        let result = resolver.resolve(&rule_obligatory, &rule_prohibited);
        
        // Expect the rule with higher priority (Obligation) to win
        assert_eq!(result.winning_rule.unwrap().id, 1);
        println!("Conflict resolved: {:?}", result.resolved_by);
    }
}
