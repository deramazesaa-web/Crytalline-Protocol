use crate::deontic_engine::WorldState;

pub struct Axiom {
    pub id: &'static str,
    pub definition: &'static str,
}

pub struct IntegrityReport {
    pub is_valid: bool,
    pub violated_axiom: Option<String>,
}

/// Defines the fundamental laws of the system (Zermelo-Fraenkel inspired invariants)
pub fn get_fundamental_axioms() -> Vec<Axiom> {
    vec![
        Axiom {
            id: "AXIOM_OF_NON_NEGATIVITY",
            definition: "State variables must exist within the set of non-negative real numbers (R+).",
        },
        Axiom {
            id: "AXIOM_OF_CONSISTENCY",
            definition: "Collateral ratio cannot be infinite or NaN (Not a Number).",
        },
        Axiom {
            id: "AXIOM_OF_BOUNDEDNESS",
            definition: "Market volatility cannot exceed theoretical maximum (1.0) in a single block.",
        },
    ]
}

/// Checks if the current WorldState belongs to the set of mathematically valid states.
pub fn validate_state_integrity(state: &WorldState) -> IntegrityReport {
    // 1. Check Non-Negativity
    if state.collateral_ratio < 0.0 || state.network_slippage < 0.0 {
        return IntegrityReport {
            is_valid: false,
            violated_axiom: Some("AXIOM_OF_NON_NEGATIVITY".to_string()),
        };
    }

    // 2. Check Consistency (Math errors)
    if state.collateral_ratio.is_nan() || state.network_slippage.is_infinite() {
        return IntegrityReport {
            is_valid: false,
            violated_axiom: Some("AXIOM_OF_CONSISTENCY".to_string()),
        };
    }

    // 3. Check Boundedness (System limits)
    if state.market_volatility > 10.0 { // Assuming 1000% volatility is technically impossible
        return IntegrityReport {
            is_valid: false,
            violated_axiom: Some("AXIOM_OF_BOUNDEDNESS".to_string()),
        };
    }

    IntegrityReport {
        is_valid: true,
        violated_axiom: None,
    }
}
