// what: demo module re-exports
// why: organizes demo tab into submodules for maintainability
// relations: parent module for types.rs, attacks.rs, wasm.rs, component.rs

pub mod types;
pub mod attacks;
pub mod wasm;
mod component;

#[cfg(test)]
mod tests;

// Re-export the Demo component for use by parent module
pub use component::Demo;
