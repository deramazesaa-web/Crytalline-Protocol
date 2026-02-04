/*
 * CRYSTALLINE PROTOCOL: ETHICAL RULES (LAYER 1 PREDICATES)
 * --------------------------------------------------------
 * These rules define the Predicate P(x) for the Axiom of Specification.
 * They transform raw transitions into ethically validated state changes.
 */

pub struct EthicalRules;

impl EthicalRules {
    /// The Core Predicate P(x) for the Axiom of Specification.
    /// In Crystalline, ethics are formal logical constraints.
    pub fn predicate_p(action: &str, impact_score: i32) -> bool {
        // Rule 1: Non-Harm (Basic filter)
        let is_non_harmful = impact_score >= 0;

        // Rule 2: Protocol Integrity
        let is_valid_action = action != "exploit" && action != "overflow";

        // Axiom of Specification: x belongs to the Allowed set M ONLY IF P(x) is true.
        is_non_harmful && is_valid_action
    }

    /// Example of a "Conditional" (Branch R) predicate.
    /// Requires a witness or additional proof (pi) to become true.
    pub fn requires_witness(action: &str) -> bool {
        // High-value actions or sensitive state changes move to the Conditional branch.
        action == "governance_vote" || action == "large_transfer"
    }

    /// Quantifies "Logical Density" for the Axiom of Choice.
    /// Higher ethics/utility = higher priority in conflict resolution.
    pub fn calculate_logical_density(impact_score: i32) -> u64 {
        if impact_score < 0 {
            0 // Will be rejected by the Axiom of Choice as it's the "weakest" proof.
        } else {
            impact_score as u64 + 100 // Baseline density
        }
    }
}
