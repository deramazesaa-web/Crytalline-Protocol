/*
 * CRYSTALLINE PROTOCOL: AXIOMATIC IMPLEMENTATION
 * ---------------------------------------------------------
 * Security through Zermelo-Fraenkel (ZF) Set Theory.
 */

pub mod layer_0 {
    //! Layer 0: Fundamental Axioms
    
    #[derive(Debug, PartialEq)]
    pub struct StateNode {
        pub id: String,
        pub parent_id: Option<String>, 
    }

    pub struct AxiomaticEngine;

    impl AxiomaticEngine {
        /// Axiom of Regularity (Anti-Recursion)
        /// Verifies that a state node does not reference itself or create a simple loop.
        /// In a full impl, this would perform a Deep Directed Acyclic Graph (DAG) check.
        pub fn verify_regularity(current_id: &str, parent_id: &Option<String>) -> bool {
            match parent_id {
                Some(p_id) => current_id != p_id, // Basic check: node cannot be its own parent
                None => true, // Root node is always regular
            }
        }

        /// Axiom of Choice: Deterministic Choice Function
        /// Selects the transaction with the highest "Logical Density" (Proof weight).
        pub fn dcf_select(tx_a: Transaction, tx_b: Transaction) -> Transaction {
            if tx_a.logical_density >= tx_b.logical_density { tx_a } else { tx_b }
        }
    }

    #[derive(Clone)]
    pub struct Transaction {
        pub id: String,
        pub logical_density: u64,
    }
}

pub mod layer_1 {
    use crate::layer_0::Transaction;

    pub enum StatePartition {
        Allowed,    // Set M
        Forbidden,  // Set N
        Conditional, // Set R
    }

    pub struct Filter;

    impl Filter {
        pub fn classify(tx: &Transaction) -> StatePartition {
            if tx.logical_density == 0 {
                StatePartition::Forbidden
            } else {
                StatePartition::Allowed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::layer_0::*;

    #[test]
    fn test_regularity_violation() {
        let node_id = "node_1".to_string();
        let invalid_parent = Some("node_1".to_string());

        // This should return FALSE because it violates the Axiom of Regularity (x ∉ x)
        let is_valid = AxiomaticEngine::verify_regularity(&node_id, &invalid_parent);
        
        assert!(!is_valid, "Axiom of Regularity failed: Node allowed to be its own parent!");
        println!("Regularity Check Passed: Circular reference blocked.");
    }

    #[test]
    fn test_choice_function() {
        let tx1 = Transaction { id: "tx_1".to_string(), logical_density: 100 };
        let tx2 = Transaction { id: "tx_2".to_string(), logical_density: 200 };

        let winner = AxiomaticEngine::dcf_select(tx1, tx2);
        assert_eq!(winner.id, "tx_2");
        println!("Choice Function Passed: Deterministic winner selected.");
    }
}
