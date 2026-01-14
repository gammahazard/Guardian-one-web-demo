// what: problem tab module with modular sub-sections
// why: organizes problem tab content into reusable components
// relations: exposes Problem component to parent tabs module

mod component;
mod quotes;
mod vulnerabilities;
mod comparison;

pub use component::Problem;
