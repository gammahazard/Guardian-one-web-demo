// what: docker vs wasm comparison table
// why: clear side-by-side showing architectural differences
// relations: used by problem/component.rs as final section before CTA

use leptos::*;

/// renders the comparison table and CTA button
#[component]
pub fn ComparisonSection() -> impl IntoView {
    view! {
        <div class="comparison-section">
            <h3>"⚖️ Docker vs WASM: Architectural Differences"</h3>
            
            <div class="comparison-table">
                <div class="comparison-row header">
                    <span class="comparison-cell">"Property"</span>
                    <span class="comparison-cell docker">"🐳 Docker"</span>
                    <span class="comparison-cell wasm">"🦀 WASM + WASI"</span>
                </div>
                
                <ComparisonRow 
                    property="Isolation Level"
                    docker="Process (shared kernel)"
                    docker_bad=true
                    wasm="Instruction (no kernel)"
                    wasm_good=true
                />
                <ComparisonRow 
                    property="Cold Start"
                    docker="500ms - 5 seconds"
                    docker_bad=true
                    wasm="< 1 millisecond"
                    wasm_good=true
                />
                <ComparisonRow 
                    property="Binary Size"
                    docker="50 - 200 MB"
                    docker_bad=true
                    wasm="15 - 70 KB"
                    wasm_good=true
                />
                <ComparisonRow 
                    property="Default Permissions"
                    docker="Network + FS access"
                    docker_bad=true
                    wasm="DENY all (capability grants)"
                    wasm_good=true
                />
                <ComparisonRow 
                    property="Container Escape"
                    docker="CVEs every year"
                    docker_bad=true
                    wasm="Reduced surface (runtime only)"
                    wasm_good=true
                />
                <ComparisonRow 
                    property="Recovery from Crash"
                    docker="Restart container (seconds)"
                    docker_bad=true
                    wasm="TRAP + reinit (microseconds)"
                    wasm_good=true
                />
            </div>
            
            <div class="problem-cta">
                <p>"Ready to see this in action?"</p>
                <button class="cta-button" 
                   onclick="Array.from(document.querySelectorAll('.tab')).find(t => t.textContent.includes('Demo'))?.click(); window.scrollTo(0,0);">
                    "🚀 Go to The Demo"
                </button>
            </div>
        </div>
    }
}

/// single comparison row
#[component]
fn ComparisonRow(
    property: &'static str,
    docker: &'static str,
    docker_bad: bool,
    wasm: &'static str,
    wasm_good: bool,
) -> impl IntoView {
    let docker_class = if docker_bad { "comparison-cell docker bad" } else { "comparison-cell docker" };
    let wasm_class = if wasm_good { "comparison-cell wasm good" } else { "comparison-cell wasm" };
    
    view! {
        <div class="comparison-row">
            <span class="comparison-cell property">{property}</span>
            <span class={docker_class}>
                {if docker_bad { "❌ " } else { "" }}
                {docker}
            </span>
            <span class={wasm_class}>
                {if wasm_good { "✅ " } else { "" }}
                {wasm}
            </span>
        </div>
    }
}
