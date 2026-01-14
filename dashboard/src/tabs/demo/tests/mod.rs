// what: exports all test modules for demo tab
// why: keeps tests organized and discoverable via cargo test

#[cfg(test)]
mod attack_logic;

#[cfg(test)]
mod voting_logic;

#[cfg(test)]
mod measurement;

#[cfg(test)]
mod state_invariants;
