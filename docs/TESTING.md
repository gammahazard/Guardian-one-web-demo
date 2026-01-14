# Testing — Guardian One Web-Demo

## Philosophy
- **No mock passes** — every test validates real behavior
- **Modular files** — each ~50-70 lines
- **Lowercase comments** — explain what/why

## Run Tests

```bash
cd dashboard && cargo test --lib
```

## Modules

### attack_logic.rs (8 tests)
Validates attack configuration accuracy and Python code validity.

| Test | What |
|------|------|
| `config_buffer_overflow_has_correct_wit_func` | WIT modal accuracy |
| `config_data_exfil_has_network_trap` | Trap message correct |
| `config_path_traversal_has_filesystem_trap` | Trap message correct |
| `all_security_attacks_have_restart_time` | No instant respawn |
| `unknown_attack_returns_default_config` | Edge case: bad input |
| `empty_attack_string_handled` | Edge case: empty input |
| `all_attack_names_are_unique` | No UI confusion |
| `python_code_has_result_variable` | Pyodide returns last expr |

### voting_logic.rs (8 tests)
Validates 2oo3 TMR voting and Raft-like leader election.

| Test | What |
|------|------|
| `two_healthy_one_faulty_returns_majority` | Core TMR guarantee |
| `exactly_two_healthy_is_minimum` | Boundary case |
| `one_healthy_two_faulty_fails` | Safety check |
| `all_faulty_returns_error` | System halt |
| `leader_crash_triggers_election` | Failover works |
| `rebuilt_instance_becomes_follower` | No split-brain |
| `leader_id_wraps_around` | Modulo 3 correct |
| `rapid_crashes_dont_corrupt_state` | No race conditions |

### measurement.rs (7 tests)
Validates metric calculations and sanity bounds.

| Test | What |
|------|------|
| `speedup_calculation_is_correct` | Thesis claim (~50000x) |
| `speedup_handles_zero_wasm_time` | Divide by zero |
| `downtime_accumulates_correctly` | 3 × 1500ms = 4500ms |
| `wasm_downtime_always_zero` | Core thesis |
| `negative_time_rejected` | Clock skew |
| `python_time_has_upper_bound` | Detect hung Pyodide |
| `reset_clears_all_metrics` | Clean state |

### state_invariants.rs (5 tests)
Validates system-wide invariants.

| Test | What |
|------|------|
| `instance_count_always_three` | TMR requirement |
| `faulty_count_never_exceeds_one` | Demo constraint |
| `running_all_completes_in_order` | No overlap |
| `is_running_prevents_double_trigger` | State protection |
| `worker_pool_always_has_active` | Python never dead |

## Total: 28 tests
