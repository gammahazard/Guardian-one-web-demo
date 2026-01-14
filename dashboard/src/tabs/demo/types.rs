// what: shared types for demo tab components
// why: separates data structures from UI logic for better maintainability
// relations: used by component.rs, attacks.rs; part of tabs/demo module

/// log entry for terminal output display
#[derive(Clone)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
}

/// attack configuration with realistic python restart times
#[allow(dead_code)] // restart_ms used via pyodide_load_ms fallback
pub struct AttackConfig {
    pub name: &'static str,
    pub restart_ms: u32,
    pub wasm_trap: &'static str,
    pub wit_func: &'static str,
}

/// wasm instance state for 2oo3 voting visualization
#[derive(Clone, Copy, PartialEq)]
pub enum InstanceState {
    Healthy,
    Faulty,
}
