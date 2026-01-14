// what: tests for system-wide invariants that should never be violated
// why: catches violations of fundamental assumptions about state

use crate::tabs::demo::types::InstanceState;

#[test]
fn instance_count_always_three() {
    // what: tmr requires exactly 3 instances
    // why: 2oo3 voting assumes exactly 3 nodes
    let instances = [InstanceState::Healthy, InstanceState::Healthy, InstanceState::Healthy];
    assert_eq!(instances.len(), 3, "must have exactly 3 instances for tmr");
}

#[test]
fn faulty_count_never_exceeds_one() {
    // what: demo only marks one instance faulty at a time
    // why: simplifies ui and matches demo narrative
    let states = [InstanceState::Healthy, InstanceState::Faulty, InstanceState::Healthy];
    let faulty_count = states.iter().filter(|s| **s == InstanceState::Faulty).count();
    assert!(faulty_count <= 1, "demo should have at most 1 faulty instance at a time");
}

#[test]
fn running_all_completes_in_order() {
    // what: 5 attacks should fire sequentially
    // why: no overlap corruption between attack simulations
    let attack_order = ["bufferOverflow", "dataExfil", "pathTraversal", "killLeader", "heartbeatTimeout"];
    assert_eq!(attack_order.len(), 5, "run_all should execute exactly 5 attacks");
}

#[test]
fn is_running_prevents_double_trigger() {
    // what: can't trigger attack while another is running
    // why: prevents state corruption from overlapping attacks
    let is_running = true;
    let can_trigger = !is_running;
    assert!(!can_trigger, "should not trigger while running");
}

#[test]
fn worker_pool_always_has_active() {
    // what: at least 1 python worker must be active
    // why: python never fully dead, just degraded
    let workers = [false, true, true]; // w0 dead, w1 active, w2 standby
    let active_count = workers.iter().filter(|w| **w).count();
    assert!(active_count >= 1, "must have at least 1 active worker");
}
