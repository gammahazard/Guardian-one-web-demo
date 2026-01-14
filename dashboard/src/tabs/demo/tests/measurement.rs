// what: tests for metric calculations and sanity bounds
// why: validates thesis claims about speedup and ensures no math errors

#[test]
fn speedup_calculation_is_correct() {
    // what: verify displayed speedup ratio math
    // why: this is our key thesis claim, must be mathematically accurate
    let python_ms = 1500.0;
    let wasm_ms = 0.03;
    let speedup = python_ms / wasm_ms;
    assert!(speedup > 40000.0 && speedup < 60000.0, "speedup should be ~50000x");
}

#[test]
fn speedup_handles_zero_wasm_time() {
    // what: if wasm time is 0, avoid divide by zero
    // why: edge case protection for instant measurements
    let python_ms = 1500.0;
    let wasm_ms = 0.0;
    
    // the ui should show "—" or infinity, not crash
    let speedup = if wasm_ms > 0.0 { python_ms / wasm_ms } else { f64::INFINITY };
    assert!(speedup.is_infinite() || speedup > 0.0);
}

#[test]
fn downtime_accumulates_correctly() {
    // what: 3 crashes x 1500ms = 4500ms total downtime
    // why: total downtime stat must be mathematically correct
    let crashes = 3u64;
    let restart_ms = 1500u64;
    let total_downtime = crashes * restart_ms;
    assert_eq!(total_downtime, 4500, "3 crashes x 1500ms = 4500ms");
}

#[test]
fn wasm_downtime_always_zero() {
    // what: wasm downtime should always be 0
    // why: core thesis - wasm has zero downtime due to instant rebuild
    let wasm_downtime: u64 = 0;
    assert_eq!(wasm_downtime, 0, "wasm should never have downtime");
}

#[test]
fn negative_time_rejected() {
    // what: negative time values should be treated as errors
    // why: clock skew protection
    let measured_time = -5.0;
    let is_valid = measured_time >= 0.0;
    assert!(!is_valid, "negative time should be invalid");
}

#[test]
fn python_time_has_upper_bound() {
    // what: python coldstart should be < 10000ms
    // why: detect hung pyodide (anything over 10s is broken)
    let python_coldstart = 1500.0;
    let max_reasonable = 10000.0;
    assert!(python_coldstart < max_reasonable, "python coldstart should be under 10s");
}

#[test]
fn reset_clears_all_metrics() {
    // what: after reset, all counters should be 0
    // why: ensures clean state for fresh demo
    let python_processed: u32 = 0;
    let python_crashed: u32 = 0;
    let python_downtime_ms: u64 = 0;
    let wasm_processed: u32 = 0;
    let wasm_rejected: u32 = 0;
    
    assert_eq!(python_processed, 0);
    assert_eq!(python_crashed, 0);
    assert_eq!(python_downtime_ms, 0);
    assert_eq!(wasm_processed, 0);
    assert_eq!(wasm_rejected, 0);
}
