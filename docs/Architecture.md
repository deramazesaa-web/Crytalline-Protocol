# 🏛 System Architecture: Crystalline Protocol

This document describes the high-level architecture of the Crystalline Node and the data flow between its core components.

---

## 🏗 Modular Structure

The protocol is designed with **Strict Separation of Concerns**. Each module represents a specific layer of the axiomatic state machine.

### 1. Data Ingestion Layer (`market_data.rs`)
The **Oracle** acts as the gateway to the external world. 
- **Responsibility:** Fetches environmental data (prices, volatility).
- **Transformation:** Translates raw numbers into "Axiomatic Sentiment" which influences rule weights.

### 2. Logic Layer (`deontic_engine.rs` & `ethical_rules.rs`)
Defines the **Ontology** of the system.
- **Responsibility:** Categorizes actions as Obligatory, Permitted, or Prohibited.
- **Constraints:** Ensures that rules are properly formatted before reaching the resolver.

### 3. Resolution Layer (`resolver.rs`)
The "Brain" of the protocol, implementing **Formal Conflict Resolution**.
- **Mechanism:** Uses the **Axiom of Choice** logic to compare weights $W$.
- **Error Handling:** If a paradox is found that violates the *Axiom of Regularity*, it triggers a `CrystallineError`.

### 4. Persistence Layer (`axiomatics.rs`)
The "Source of Truth" (The State).
- **Responsibility:** Stores the history of committed rules.
- **State Root:** Updates the hash of the system after every successful resolution.

### 5. Verification Layer (`proof.rs`)
Ensures **Public Verifiability**.
- **Responsibility:** Generates a cryptographic Witness for every decision.
- **Security:** Allows light clients to verify the state without re-running the entire resolution logic.

---

## 🔄 Data Flow (The Epoch Cycle)

The system operates in a linear sequence of events called an **Epoch**:

1. **Observe:** `MarketOracle` detects current environmental state.
2. **Postulate:** System creates competing `Rules` based on current state.
3. **Resolve:** `ConflictResolver` selects the winning rule via priority reduction.
4. **Commit:** `AxiomaticState` records the rule and updates the `state_root`.
5. **Witness:** `ProofGenerator` signs the transition, creating an immutable proof.



---

## 🛡 Security Guarantees

* **Mathematical Finality:** Once a proof is generated, the state transition is logically immutable.
* **Deterministic Execution:** Given the same inputs and state, the Resolver will always produce the same winner.
* **Paradox Prevention:** The `errors.rs` module ensures that "logical bombs" (self-referential rules) are rejected at the gate.
