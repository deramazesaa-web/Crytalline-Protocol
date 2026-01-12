pub mod deontic_engine;
/* * CRYSTALLINE PROTOCOL: Proof of Concept (v0.1.0)
 * -----------------------------------------------
 * This PoC demonstrates the "Logic Firewall" mechanism.
 * Unlike EVM, which executes imperative code blindly, CVM (Crystalline VM)
 * validates the topological consistency of the resulting state against
 * Zermelo-Fraenkel (ZF) axioms BEFORE execution.
 * * Focus: AXIOM OF REGULARITY (Foundation)
 * Goal: Prevent logical recursion/re-entrancy loops at the protocol level.
 */

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

// --- TYPES & ENTITIES ---

type EntityId = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RelationType {
    Controls,       // Entity A controls Entity B
    Funds,          // Entity A funds Entity B
    DependsOn,      // Entity A logic depends on Entity B
}

#[derive(Debug, Clone)]
struct Relationship {
    from: EntityId,
    to: EntityId,
    rel_type: RelationType,
}

// The "World State" modeled as a Directed Graph (Set of relationships)
#[derive(Default)]
struct WorldState {
    entities: HashSet<EntityId>,
    edges: Vec<Relationship>,
}

impl WorldState {
    fn new() -> Self {
        Self::default()
    }

    fn add_entity(&mut self, id: &str) {
        self.entities.insert(id.to_string());
    }

    // Temporarily apply a transition to check validity
    fn simulate_transition(&self, transition: &Transaction) -> WorldState {
        let mut new_state = WorldState {
            entities: self.entities.clone(),
            edges: self.edges.clone(),
        };

        // Apply new relationships from the transaction
        for edge in &transition.new_edges {
            new_state.entities.insert(edge.from.clone());
            new_state.entities.insert(edge.to.clone());
            new_state.edges.push(edge.clone());
        }
        new_state
    }
}

// --- LOGIC ENGINE (ZF AXIOM VERIFIER) ---

#[derive(Debug)]
enum AxiomViolation {
    Regularity(String), // Violation of Foundation Axiom (Cycles)
    Extensionality,     // Reserved for future: Identity conflicts
}

impl fmt::Display for AxiomViolation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AxiomViolation::Regularity(msg) => write!(f, "[AXIOM OF REGULARITY VIOLATION] {}", msg),
            AxiomViolation::Extensionality => write!(f, "[AXIOM OF EXTENSIONALITY VIOLATION]"),
        }
    }
}

struct LogicEngine;

impl LogicEngine {
    /// Validates a state against the Axiom of Regularity.
    /// In Graph Theory terms: Checks if the graph is a DAG (Directed Acyclic Graph).
    /// In Set Theory: Ensures no set contains itself (x ∈ x) or forms a loop (x ∈ y ∈ x).
    fn verify_regularity(state: &WorldState) -> Result<(), AxiomViolation> {
        let mut adjacency: HashMap<&EntityId, Vec<&EntityId>> = HashMap::new();
        
        // Build adjacency list
        for edge in &state.edges {
            adjacency.entry(&edge.from).or_default().push(&edge.to);
        }

        // Detect cycles using DFS
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for node in &state.entities {
            if Self::has_cycle(node, &adjacency, &mut visited, &mut recursion_stack) {
                return Err(AxiomViolation::Regularity(format!(
                    "Logical loop detected involving entity '{}'. \
                    This violates the Foundation Axiom (infinite regress).", 
                    node
                )));
            }
        }
        Ok(())
    }

    fn has_cycle<'a>(
        node: &'a EntityId,
        adj: &HashMap<&'a EntityId, Vec<&'a EntityId>>,
        visited: &mut HashSet<&'a EntityId>,
        stack: &mut HashSet<&'a EntityId>,
    ) -> bool {
        stack.insert(node);
        visited.insert(node);

        if let Some(neighbors) = adj.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if Self::has_cycle(neighbor, adj, visited, stack) {
                        return true;
                    }
                } else if stack.contains(neighbor) {
                    return true; // Cycle found
                }
            }
        }

        stack.remove(node);
        false
    }
}

// --- VIRTUAL MACHINE (CVM) ---

struct Transaction {
    id: String,
    description: String,
    new_edges: Vec<Relationship>,
}

struct CrystallineVM {
    state: WorldState,
}

impl CrystallineVM {
    fn new() -> Self {
        Self { state: WorldState::new() }
    }

    fn submit_transaction(&mut self, tx: Transaction) {
        println!("\n>>> Processing TX: [{}] {}", tx.id, tx.description);
        println!("... Generating State Transition Proof...");

        // 1. Simulate the future state
        let future_state = self.state.simulate_transition(&tx);

        // 2. Verify Axioms on the future state
        match LogicEngine::verify_regularity(&future_state) {
            Ok(_) => {
                println!("✅ [VALID] Axiomatic check passed. Committing state.");
                self.state = future_state;
            }
            Err(e) => {
                println!("⛔ [REJECTED] Logic Firewall triggered.");
                println!("   Reason: {}", e);
                println!("   Transaction dropped. State rolled back.");
            }
        }
    }
}

// --- SCENARIO RUNNER ---

fn main() {
    println!("#####################################################");
    println!("###   CRYSTALLINE PROTOCOL: LOGIC ENGINE v0.1     ###");
    println!("###   Implementing ZF-Set Theory for Governance   ###");
    println!("#####################################################\n");

    let mut cvm = CrystallineVM::new();

    // Init: Create some basic entities
    let genesis_tx = Transaction {
        id: "TX_001".to_string(),
        description: "Genesis: Create DAO_Treasury and Admin_Agent".to_string(),
        new_edges: vec![
            Relationship { 
                from: "Admin_Agent".to_string(), 
                to: "DAO_Treasury".to_string(), 
                rel_type: RelationType::Controls 
            }
        ],
    };
    cvm.submit_transaction(genesis_tx);

    // Scenario 1: Safe expansion
    // Sub-DAO is created by DAO, perfectly linear hierarchy.
    let safe_tx = Transaction {
        id: "TX_002".to_string(),
        description: "Expansion: DAO_Treasury creates Sub_Committee".to_string(),
        new_edges: vec![
            Relationship {
                from: "DAO_Treasury".to_string(),
                to: "Sub_Committee".to_string(),
                rel_type: RelationType::Funds
            }
        ],
    };
    cvm.submit_transaction(safe_tx);

    // Scenario 2: THE ATTACK (Governance Slime)
    // The Sub_Committee tries to pass a rule that gives it control over the Admin_Agent.
    // This creates a cycle: Admin -> DAO -> Sub -> Admin.
    // In traditional code, this might pass ("If voter has > 51%").
    // In Crystalline, this violates Regularity.
    
    let attack_tx = Transaction {
        id: "TX_003_ATTACK".to_string(),
        description: "Governance Attack: Sub_Committee votes to control Admin_Agent".to_string(),
        new_edges: vec![
            Relationship {
                from: "Sub_Committee".to_string(),
                to: "Admin_Agent".to_string(),
                rel_type: RelationType::Controls
            }
        ],
    };

    println!("\n!!! SIMULATING LOGICAL ATTACK !!!");
    cvm.submit_transaction(attack_tx);

    println!("\n>>> Simulation Complete. System Integrity: 100%");
}
