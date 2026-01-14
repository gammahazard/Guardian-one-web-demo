// what: hardware tab showing the physical architecture being simulated
// why: grounds the demo in real industrial edge topology
// relations: used by mod.rs, second tab in story flow

use leptos::*;

#[component]
pub fn Hardware() -> impl IntoView {
    view! {
        <div class="tab-content hardware-tab">
            <h2>"The Hardware: Industrial Edge Architecture"</h2>
            
            <div class="architecture-diagram">
                <div class="purdue-level level-3">
                    <span class="level-label">"Level 3: Operations"</span>
                    <div class="level-content">
                        <span class="component">"📊 QNAP Historian"</span>
                        <span class="component">"📈 Grafana Dashboard"</span>
                    </div>
                </div>
                
                <div class="purdue-level level-2">
                    <span class="level-label">"Level 2: Supervisory"</span>
                    <div class="level-content">
                        <span class="component guardian">"🔒 Guardian Cluster (WASM Runtime)"</span>
                        <span class="component">"🔀 UniFi Switch"</span>
                    </div>
                </div>
                
                <div class="purdue-level level-1">
                    <span class="level-label">"Level 1: Field I/O"</span>
                    <div class="level-content">
                        <span class="component">"🌡️ BME280 Sensors"</span>
                        <span class="component">"⚡ Relay Modules"</span>
                    </div>
                </div>
                
                <div class="purdue-level level-0">
                    <span class="level-label">"Level 0: Process"</span>
                    <div class="level-content">
                        <span class="component">"🏭 Siemens S7-1200 (Future)"</span>
                    </div>
                </div>
            </div>
            
            <div class="network-info">
                <p><strong>"Network Zone:"</strong>" 192.168.40.x (isolated via UniFi Switch)"</p>
                <p><strong>"Nodes:"</strong>" Pi 4 (Leader) + 2x Pi Zero 2W (Followers)"</p>
            </div>
        </div>
    }
}
