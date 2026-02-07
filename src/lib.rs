pub mod axiomatics;
pub mod deontic_engine;
pub mod ffi;

// Re-exporting for easier access in examples
pub use axiomatics::{AxiomaticEngine, LogicPartition};
pub use deontic_engine::{DeonticEngine, Norm, ActionStatus};
