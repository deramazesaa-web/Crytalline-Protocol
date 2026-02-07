use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use crate::deontic_engine::{DeonticEngine, Norm, ActionStatus};
use crate::axiomatics::{AxiomaticEngine, LogicPartition};

/// Initialize the Crystalline Core.
/// Returns 1 if successful, 0 if failed.
#[no_mangle]
pub extern "C" fn crystalline_init() -> c_int {
    // In a real scenario, we might set up global state or loggers here.
    // For now, we confirm the core is responsive.
    1
}

/// The main entry point for mobile apps.
/// Takes a C-string (from Swift/Kotlin), checks axioms, and returns:
/// 1 = Allowed (Secure)
/// 0 = Forbidden (Insecure or Error)
#[no_mangle]
pub unsafe extern "C" fn crystalline_check_compliance(payload: *const c_char) -> c_int {
    if payload.is_null() {
        return 0;
    }

    // 1. Convert C-string (unsafe) to Rust string (safe)
    let c_str = CStr::from_ptr(payload);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0, // UTF-8 error
    };

    // 2. Spin up the Engines
    let engine = AxiomaticEngine::new();
    let mut deontic = DeonticEngine::new();

    // 3. Setup Default Security Norms
    deontic.add_norm(Norm::new("MOBILE_DATA_INTEGRITY"));

    // 4. Axiomatic Check (Regularity)
    if !engine.verify_regularity(str_slice) {
        return 0; // Violation of Axiom of Regularity
    }

    // 5. Deontic Check
    match deontic.check_compliance(str_slice) {
        ActionStatus::Allowed => 1,
        ActionStatus::Forbidden => 0,
    }
}

/// Create a secure partition check.
/// Returns 1 if the partition name is valid, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn crystalline_verify_partition(label: *const c_char) -> c_int {
    if label.is_null() {
        return 0;
    }

    let c_str = CStr::from_ptr(label);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let partition = LogicPartition::new(str_slice);
    if partition.is_valid() { 1 } else { 0 }
}
