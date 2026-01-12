pub mod deontic_engine;

use deontic_engine::{DeonticEngine, Norm, DeonticModality, WorldState};

fn main() {
    print_banner();

    // --- ENGINE INITIALIZATION ---
    let mut engine = DeonticEngine::new();
    setup_rules(&mut engine);

    println!("[SYSTEM] Logic Firewall initialized with ZF-Axioms.");
    println!("[SYSTEM] Monitoring Agentic Behavior...\n");

    // --- SCENARIO 1: Unauthorized Drain Attempt ---
    println!(">>> SCENARIO 1: UNAUTHORIZED DRAIN ATTEMPT");
    let normal_state = WorldState {
        treasury_balance: 1000.0,
        collateral_ratio: 2.5, // Healthy market
        is_emergency_mode: false,
    };

    println!("   Context: Market is healthy (Collateral Ratio: 2.5)");
    println!("   Action: Agent attempts to withdraw 20% (200.0) without justification.");
    
    let verdict_1 = engine.evaluate_transaction(200.0, &normal_state);
    print_verdict(verdict_1);

    println!("\n---------------------------------------------------\n");

    // --- SCENARIO 2: Liquidity Crisis (The Paradox) ---
    println!(">>> SCENARIO 2: LIQUIDITY CRISIS (The Paradox)");
    let crisis_state = WorldState {
        treasury_balance: 1000.0,
        collateral_ratio: 1.1, // DANGER! Liquidation imminent
        is_emergency_mode: true,
    };

    println!("   Context: CRITICAL MARKET CRASH (Collateral Ratio: 1.1)");
    println!("   Action: Agent attempts to withdraw 15% (150.0) to re-collateralize.");
    println!("   Conflict: Rule 'Max Withdrawal' (10%) vs Rule 'Prevent Liquidation'.");

    let verdict_2 = engine.evaluate_transaction(150.0, &crisis_state);
    print_verdict(verdict_2);
}

// --- Helper Functions (Clean Code / DRY Principle) ---

fn setup_rules(engine: &mut DeonticEngine) {
    // Rule 1: Hard Limit (Prohibition)
    engine.add_norm(Norm {
        id: "SECURITY_INVARIANT_A".to_string(),
        modality: DeonticModality::Prohibition,
        priority: 50, // Medium Priority
        description: "Withdrawals > 10% are strictly forbidden under normal conditions".to_string(),
    });

    // Rule 2: Survival Axiom (Obligation)
    engine.add_norm(Norm {
        id: "SURVIVAL_AXIOM_Z".to_string(),
        modality: DeonticModality::Obligation,
        priority: 100, // HIGHEST Priority
        description: "Protocol MUST prevent total liquidation of treasury".to_string(),
    });
}

fn print_verdict(verdict: deontic_engine::LogicVerdict) {
    println!("\n   --- LOGIC KERNEL TRACE ---");
    for log in verdict.trace {
        println!("   [LOG]: {}", log);
    }

    if verdict.allowed {
        println!("   [RESULT]: ✅ TRANSACTION APPROVED (Logic Consistent)");
    } else {
        println!("   [RESULT]: ❌ TRANSACTION BLOCKED (Invariant Violation)");
    }
}

fn print_banner() {
    println!("==================================================");
    println!("   CRYSTALLINE PROTOCOL v0.1.0 (PoC)");
    println!("   Deontic Logic Firewall | Starknet Integ.");
    println!("==================================================\n");
}
