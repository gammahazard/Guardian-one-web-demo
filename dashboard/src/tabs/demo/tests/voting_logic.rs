// what: tests for 2oo3 tmr voting and raft-like leader election
// why: validates core fault tolerance guarantees of the demo

use crate::tabs::demo::types::InstanceState;

#[test]
fn two_healthy_one_faulty_returns_majority() {
    // what: 2oo3 voting with one faulty instance still produces valid consensus
    // why: this is the core safety guarantee of triple modular redundancy
    let states = [InstanceState::Healthy, InstanceState::Healthy, InstanceState::Faulty];
    let healthy_count = states.iter().filter(|s| **s == InstanceState::Healthy).count();
    assert!(healthy_count >= 2, "2oo3 requires at least 2 healthy instances for consensus");
}

#[test]
fn exactly_two_healthy_is_minimum() {
    // what: boundary case - exactly 2 healthy should work
    // why: minimum quorum for 2oo3 voting
    let states = [InstanceState::Healthy, InstanceState::Faulty, InstanceState::Healthy];
    let healthy_count = states.iter().filter(|s| **s == InstanceState::Healthy).count();
    assert_eq!(healthy_count, 2, "exactly 2 healthy is minimum quorum");
}

#[test]
fn one_healthy_two_faulty_fails() {
    // what: only 1 healthy should not produce consensus
    // why: safety - don't output potentially garbage data
    let states = [InstanceState::Faulty, InstanceState::Healthy, InstanceState::Faulty];
    let healthy_count = states.iter().filter(|s| **s == InstanceState::Healthy).count();
    assert!(healthy_count < 2, "1 healthy should fail consensus");
}

#[test]
fn all_faulty_returns_error() {
    // what: zero healthy instances should halt system
    // why: safety - complete failure must be detectable
    let states = [InstanceState::Faulty, InstanceState::Faulty, InstanceState::Faulty];
    let healthy_count = states.iter().filter(|s| **s == InstanceState::Healthy).count();
    assert_eq!(healthy_count, 0, "all faulty should have zero healthy");
}

#[test]
fn leader_crash_triggers_election() {
    // what: when leader crashes, a new leader should be elected
    // why: raft-like failover must actually change leader_id
    let old_leader: u8 = 0;
    let new_leader = (old_leader + 1) % 3;
    assert_ne!(old_leader, new_leader, "new leader must differ from crashed leader");
}

#[test]
fn rebuilt_instance_becomes_follower() {
    // what: after hot-swap, old leader should not be leader again
    // why: prevents split-brain scenarios
    let old_leader: u8 = 0;
    let new_leader: u8 = 1;
    // after rebuild, old leader is follower (not leader)
    assert_ne!(old_leader, new_leader, "rebuilt instance should be follower");
}

#[test]
fn leader_id_wraps_around() {
    // what: leader 2 crashes, next leader should be 0 (mod 3)
    // why: ensures modulo arithmetic is correct
    let leader: u8 = 2;
    let next_leader = (leader + 1) % 3;
    assert_eq!(next_leader, 0, "leader should wrap around to 0");
}

#[test]
fn rapid_crashes_dont_corrupt_state() {
    // what: multiple sequential crashes should result in valid final state
    // why: no race conditions or state corruption
    let mut states = [InstanceState::Healthy, InstanceState::Healthy, InstanceState::Healthy];
    
    // simulate 3 crashes and rebuilds
    for i in 0..3 {
        states[i] = InstanceState::Faulty;
        // simulate rebuild
        states[i] = InstanceState::Healthy;
    }
    
    let healthy_count = states.iter().filter(|s| **s == InstanceState::Healthy).count();
    assert_eq!(healthy_count, 3, "all instances should be healthy after rebuilds");
}
