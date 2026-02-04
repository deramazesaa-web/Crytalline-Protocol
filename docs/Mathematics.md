# 🧮 Mathematical Formalism of Crystalline Protocol

This document outlines the logical and set-theoretic foundations of the Crystalline engine.

---

## 1. Deontic Logic & Conflict Resolution

In Crystalline, every governance action $a$ is evaluated through a deontic lens. We define the set of rules $R$ as a collection of triples:
$$Rule = (id, \mathcal{M}, W)$$
where:
* $\mathcal{M} \in \{O, P, F\}$ represents the **Modality**: Obligatory ($O$), Permitted ($P$), or Forbidden ($F$).
* $W \in \mathbb{R}^+$ represents the **Axiomatic Weight** (Priority).

### ⚖️ Paradox Resolution
When a conflict arises—for example, an action $a$ is both $P(a)$ and $F(a)$—the **Axiom of Choice Resolver** applies the following reduction:

$$
\text{Decision}(a) = 
\begin{cases} 
\text{Valid}, & \text{if } \sum W_{O,P} \geq \sum W_{F} \\ 
\text{Rejected}, & \text{if } \sum W_{F} > \sum W_{O,P} 
\end{cases}
$$

---

## 2. Axiomatic State Integrity (ZF-Foundation)

The protocol state $S$ is modeled as a set within the **Zermelo-Fraenkel (ZF)** framework. To prevent logical recursion (circular rules), we strictly enforce the **Axiom of Regularity**:
$$\forall S \neq \emptyset, \exists x \in S : (x \cap S = \emptyset)$$

This ensures that no rule can be its own prerequisite, preventing "governance loops" and infinite execution attacks.

---

## 3. Cryptographic Witnessing

Every transition from State $S_n$ to $S_{n+1}$ via rule $r$ must generate a witness $\omega$. The integrity of the transition is defined by the commitment function:

$$H(S_{n+1}) = \text{Blake3}(H(S_n) \parallel r_{id} \parallel \text{Witness}(\omega))$$

Where:
* $H(S)$ is the **State Root**.
* $\parallel$ denotes the concatenation operator.
* $\omega$ is the cryptographic proof generated in `proof.rs`.

---

## 4. Market-Weighted Priority (Oracle Logic)

The priority $W$ of a rule is not static but a function of market volatility $V$:
$$W_{final} = W_{base} \cdot (1 - \sigma(V))$$
where $\sigma$ is a sigmoid scaling function that reduces the priority of aggressive executive actions during high-volatility periods to favor security axioms.
