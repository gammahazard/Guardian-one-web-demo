// what: interactive demo tab comparing python vs wasm execution
// why: core proof-of-concept showing real performance differences
// relations: used by lib.rs, uses components/metrics and components/attack_arena

use leptos::*;
use crate::components::metrics::Metrics;
use crate::components::attack_arena::AttackArena;

#[component]
pub fn Demo() -> impl IntoView {
    view! {
        <div class="tab-content demo-tab">
            <h2>"The Demo: Python vs WASM Side-by-Side"</h2>
            
            <Metrics />
            
            <AttackArena />
            
            <div class="demo-panels">
                <div class="panel python-panel">
                    <h3>"🐍 Python (Pyodide)"</h3>
                    <div class="terminal">
                        <p class="terminal-line">"$ python sensor_driver.py"</p>
                        <p class="terminal-line success">"[OK] Reading temperature..."</p>
                        <p class="terminal-line">"Waiting for Pyodide..."</p>
                    </div>
                </div>
                
                <div class="panel wasm-panel">
                    <h3>"🦀 WASM (Rust)"</h3>
                    <div class="terminal">
                        <p class="terminal-line">"$ wasmtime sensor_driver.wasm"</p>
                        <p class="terminal-line success">"[OK] Module instantiated in 0.2ms"</p>
                        <p class="terminal-line success">"[OK] Reading temperature..."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
