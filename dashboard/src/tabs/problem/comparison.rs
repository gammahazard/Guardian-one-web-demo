// what: docker + wasm complementary architecture section
// why: shows the "mothership pattern" - how docker and wasm work together
// relations: used by problem/component.rs as final section before CTA

use leptos::*;

/// renders the complementary architecture section and CTA button
#[component]
pub fn ComparisonSection() -> impl IntoView {
    view! {
        <div class="comparison-section">
            <h3>"🚀 The Mothership Pattern: Docker + WASM"</h3>
            <p class="section-intro">
                "Docker handles orchestration. WASM handles fault isolation. Together, they're stronger."
            </p>
            
            // The Mothership architecture table
            <div class="mothership-table">
                <div class="mothership-row header">
                    <span class="mothership-cell">"Layer"</span>
                    <span class="mothership-cell docker">"🐳 Docker Provides"</span>
                    <span class="mothership-cell wasm">"🦀 WASM Adds"</span>
                </div>
                
                <MothershipRow 
                    layer="Deployment"
                    docker="Fleet orchestration, container registry"
                    wasm="~50KB logic patches (vs 50MB images)"
                />
                <MothershipRow 
                    layer="Isolation"
                    docker="Process namespaces, cgroups"
                    wasm="Instruction-level sandbox (no kernel)"
                />
                <MothershipRow 
                    layer="Fault Recovery"
                    docker="Container restart (~1-5s)"
                    wasm="Module TRAP + rebuild (~0.04ms)"
                />
                <MothershipRow 
                    layer="Security"
                    docker="Image signing, network policies"
                    wasm="Capability deny-by-default (WIT)"
                />
            </div>
            
            // Where WASM shines section
            <div class="wasm-shines">
                <h4>"✨ Where WASM Shines (Inside Docker)"</h4>
                <ul>
                    <li>
                        <strong>"Fault Isolation: "</strong>
                        "One WASM module crashes → container survives. No cold-start penalty."
                    </li>
                    <li>
                        <strong>"OTA Bandwidth: "</strong>
                        "Ship 50KB logic patch over satellite, not 50MB container layer."
                    </li>
                    <li>
                        <strong>"Protocol Parsing: "</strong>
                        "Run untrusted parsers (Modbus, DNP3) in sandbox. Memory bugs can't escape."
                    </li>
                    <li>
                        <strong>"Fail-Stop Faults: "</strong>
                        "Attacks produce explicit TRAP, not silence. TMR voting proceeds instantly."
                    </li>
                </ul>
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

/// single mothership row showing complementary roles
#[component]
fn MothershipRow(
    layer: &'static str,
    docker: &'static str,
    wasm: &'static str,
) -> impl IntoView {
    view! {
        <div class="mothership-row">
            <span class="mothership-cell layer">{layer}</span>
            <span class="mothership-cell docker">{docker}</span>
            <span class="mothership-cell wasm">{wasm}</span>
        </div>
    }
}
