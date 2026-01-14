// what: hardware module re-exports and organization
// why: organizes hardware tab into submodules for maintainability like demo tab
// relations: parent module for architecture.rs, components.rs, compliance.rs, toolchain.rs, component.rs

pub mod architecture;
pub mod components;
pub mod compliance;
pub mod toolchain;
mod component;

// re-export the hardware component for use by parent module
pub use component::Hardware;
