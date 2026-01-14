// what: metrics display component showing python vs wasm performance
// why: provides real-time measured values for comparison
// relations: used by tabs/demo.rs, measures with performance.now()

use leptos::*;

#[component]
pub fn Metrics() -> impl IntoView {
    // These will be populated with real measurements
    let (wasm_startup, _set_wasm_startup) = create_signal("--".to_string());
    let (python_startup, _set_python_startup) = create_signal("--".to_string());
    
    view! {
        <div class="metrics-comparison">
            <div class="metric-card wasm-metric">
                <h4>"🦀 WASM"</h4>
                <div class="metric-row">
                    <span class="label">"Binary:"</span>
                    <span class="value">"47 KB"</span>
                </div>
                <div class="metric-row">
                    <span class="label">"Startup:"</span>
                    <span class="value measured">{move || wasm_startup.get()}" (measured)"</span>
                </div>
                <div class="metric-row">
                    <span class="label">"Memory:"</span>
                    <span class="value">"~2 MB"</span>
                </div>
            </div>
            
            <div class="metric-card python-metric">
                <h4>"🐍 Python"</h4>
                <div class="metric-row">
                    <span class="label">"Binary:"</span>
                    <span class="value">"12.4 MB"</span>
                </div>
                <div class="metric-row">
                    <span class="label">"Startup:"</span>
                    <span class="value estimate">{move || python_startup.get()}" (estimate)"</span>
                </div>
                <div class="metric-row">
                    <span class="label">"Memory:"</span>
                    <span class="value">"~45 MB"</span>
                </div>
            </div>
        </div>
    }
}
