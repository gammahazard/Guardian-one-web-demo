// what: tests for attack configuration accuracy and python code validity
// why: ensures wit modal displays correct capabilities and pyodide can parse attack code

use crate::tabs::demo::attacks::{get_attack_config, ATTACK_BUFFER_OVERFLOW, ATTACK_DATA_EXFIL, ATTACK_PATH_TRAVERSAL};

#[test]
fn config_buffer_overflow_has_correct_wit_func() {
    // what: verify buffer overflow attack maps to malloc-large capability
    // why: wit modal displays this to user, must be accurate
    let config = get_attack_config("bufferOverflow");
    assert_eq!(config.wit_func, "malloc-large()");
}

#[test]
fn config_data_exfil_has_network_trap() {
    // what: verify data exfil shows network-related trap
    // why: logs must accurately reflect what wasm would block
    let config = get_attack_config("dataExfil");
    assert!(config.wasm_trap.to_lowercase().contains("network"));
}

#[test]
fn config_path_traversal_has_filesystem_trap() {
    // what: verify path traversal shows filesystem-related trap
    // why: logs must accurately reflect what wasm would block
    let config = get_attack_config("pathTraversal");
    assert!(config.wasm_trap.to_lowercase().contains("filesystem"));
}

#[test]
fn all_security_attacks_have_restart_time() {
    // what: all security attacks should have restart_ms > 500
    // why: prevents unrealistic instant respawn in demo
    let attacks = ["bufferOverflow", "dataExfil", "pathTraversal"];
    for attack in attacks {
        let config = get_attack_config(attack);
        assert!(config.restart_ms > 500, "{} should have restart_ms > 500", attack);
    }
}

#[test]
fn unknown_attack_returns_default_config() {
    // what: unknown attack type should return default, not panic
    // why: edge case protection for bad input
    let config = get_attack_config("nonexistent_attack_xyz");
    assert_eq!(config.name, "Unknown Attack");
}

#[test]
fn empty_attack_string_handled() {
    // what: empty string should return default, not panic
    // why: edge case protection for empty input
    let config = get_attack_config("");
    assert_eq!(config.name, "Unknown Attack");
}

#[test]
fn all_attack_names_are_unique() {
    // what: no two attacks should have the same display name
    // why: prevents ui confusion in attack selector
    let attacks = ["bufferOverflow", "dataExfil", "pathTraversal", "killLeader", "heartbeatTimeout"];
    let names: Vec<&str> = attacks.iter().map(|a| get_attack_config(a).name).collect();
    let unique_count = names.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(unique_count, names.len(), "attack names must be unique");
}

#[test]
fn python_code_has_result_variable() {
    // what: each attack script should end with 'result' as the return value
    // why: pyodide returns last expression, we expect 'result' to be it
    let codes = [ATTACK_BUFFER_OVERFLOW, ATTACK_DATA_EXFIL, ATTACK_PATH_TRAVERSAL];
    for code in codes {
        let trimmed = code.trim();
        assert!(trimmed.ends_with("result"), "python code should end with 'result'");
    }
}
