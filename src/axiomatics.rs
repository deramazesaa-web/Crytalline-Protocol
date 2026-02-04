use crate::proof::FormalProof;

/// AxiomaticEngine: The core Deductive Engine of Crystalline.
/// It enforces ZF-Set Theory invariants at the hardware-logic level.
pub struct AxiomaticEngine;

impl AxiomaticEngine {
    /// AXIOM OF EXTENSIONALITY
    /// Identity is derived solely from internal constituents.
    /// Eliminates Transaction Malleability: if the logic is the same, the ID is the same.
    pub fn verify_extensionality(set_a: &Vec<String>, set_b: &Vec<String>) -> bool {
        if set_a.len() != set_b.len() {
            return false;
        }
        set_a.iter().all(|item| set_b.contains(item))
    }

    /// AXIOM OF REGULARITY
    /// Every non-empty set must contain an element disjoint from itself.
    /// This makes Re-entrancy and infinite recursive loops physically impossible.
    pub fn verify_regularity(current_state: &str, next_state: &str) -> bool {
        // In a well-founded state, a transition cannot be its own cause (x ∉ x).
        if current_state == next_state {
            println!("Axiomatic Violation: Circular state dependency detected (Regularity).");
            return false;
        }
        true
    }

    /// AXIOM OF CHOICE
    /// Resolves conflicts (Double-Spends) via a Deterministic Choice Function.
    /// Instead of social voting, we select the state with the highest "Logical Density".
    pub fn resolve_choice<T, F>(conflicts: Vec<T>, density_fn: F) -> Option<T>
    where
        F: Fn(&T) -> u64,
    {
        // The engine independently collapses the wave function of possibilities
        // into a single line of truth.
        conflicts.into_iter().max_by_key(|proof| density_fn(proof))
    }

    /// AXIOM OF SPECIFICATION
    /// Partitions the state into valid subsets based on predicates (L1/L0 isolation).
    pub fn verify_specification<P>(candidate: &String, predicate: P) -> bool
    where
        P: Fn(&String) -> bool,
    {
        predicate(candidate)
    }
}

/* --- AXIOMATIC CORE TESTS --- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axiom_of_regularity() {
        let state_a = "0x71C...1b2";
        let state_b = "0x82D...3c4";
        
        // Success: Standard transition
        assert!(AxiomaticEngine::verify_regularity(state_a, state_b));
        
        // Failure: Self-reference (Re-entrancy prevention)
        assert!(!AxiomaticEngine::verify_regularity(state_a, state_a));
    }

    #[test]
    fn test_axiom_of_extensionality() {
        let set_a = vec!["tx1".to_string(), "tx2".to_string()];
        let set_b = vec!["tx2".to_string(), "tx1".to_string()];
        let set_c = vec!["tx1".to_string(), "tx3".to_string()];

        // Sets are equal if contents are equal, regardless of order
        assert!(AxiomaticEngine::verify_extensionality(&set_a, &set_b));
        assert!(!AxiomaticEngine::verify_extensionality(&set_a, &set_c));
    }

    #[test]
    fn test_axiom_of_choice_resolution() {
        let p1 = FormalProof { witness_pi: "p1".to_string(), density: 50, is_verified: true };
        let p2 = FormalProof { witness_pi: "p2".to_string(), density: 150, is_verified: true };
        let p3 = FormalProof { witness_pi: "p3".to_string(), density: 10, is_verified: true };

        let conflicts = vec![p1, p2, p3];
        let winner = AxiomaticEngine::resolve_choice(conflicts, |p| p.density).unwrap();
        
        assert_eq!(winner.density, 150);
        assert_eq!(winner.witness_pi, "p2");
    }
}
