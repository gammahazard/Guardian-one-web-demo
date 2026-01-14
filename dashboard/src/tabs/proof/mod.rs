// what: proof tab module
// why: organizes the proof, benchmarking, and ota comparison components
// relations: exports Proof component to tabs/mod.rs, ota_simulator used internally

mod component;
mod ota_simulator;

pub use component::Proof;
