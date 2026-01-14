// what: purdue model architecture visualization component with tooltips
// why: shows iec 62443 zones with actual hardware placement and educational tooltips
// relations: used by hardware/component.rs as one of four sub-sections

use leptos::*;

// tooltip text constants for easy editing
const L3_TOOLTIP: &str = "Enterprise IT zone: Stores historical data, dashboards, and analytics. Separated from control systems by network segmentation.";
const L2_TOOLTIP: &str = "WASM Runtime with WIT Contracts: The Guardian Cluster runs WebAssembly modules in a sandboxed environment. Each capability (Modbus, GPIO, Network) must be explicitly granted via WIT contracts. This 'deny-by-default' model means even compromised code cannot access hardware it wasn't designed for. Visit 'The Demo' tab to see the actual WIT contract!";
const L1_TOOLTIP: &str = "Industrial PLC: Executes real-time control logic. Receives validated commands from Level 2 via Modbus RTU protocol.";
const L0_TOOLTIP: &str = "Physical sensors and actuators: Direct hardware interface. BME280 reads temperature/humidity, relay controls the industrial fan.";

/// renders the purdue model zones diagram showing hardware at each level
#[component]
pub fn ArchitectureSection() -> impl IntoView {
    view! {
        <div class="architecture-section">
            <h3>"Purdue Model — IEC 62443 Zones"</h3>
            <p class="section-hint">"💡 Hover over each level for details"</p>
            
            <div class="purdue-diagram">
                // level 3: operations management (enterprise it)
                <PurdueLevel 
                    level="L3"
                    name="Operations Management"
                    tooltip=L3_TOOLTIP
                    class="level-3"
                >
                    <div class="level-components">
                        <HardwareCard icon="📊" name="QNAP NAS" role="Historian" />
                        <HardwareCard icon="📈" name="InfluxDB" role="Time-Series" />
                        <HardwareCard icon="🖥️" name="Grafana" role="Dashboard" />
                    </div>
                </PurdueLevel>
                
                <div class="flow-arrow">"▼"</div>
                
                // level 2: supervisory control (guardian cluster with 3 pis)
                <PurdueLevel 
                    level="L2"
                    name="Supervisory Control — Guardian Cluster"
                    tooltip=L2_TOOLTIP
                    class="level-2 guardian-zone"
                >
                    // wit and wasm technology badges
                    <div class="tech-badges">
                        <div class="tech-badge" title="WebAssembly sandboxed runtime - instant cold start, memory-safe execution">
                            <img src="diagrams/wasm_icon.png" alt="WASM" class="tech-icon" />
                            <span>"WASM Runtime"</span>
                        </div>
                        <div class="tech-badge" title="WebAssembly Interface Types - explicit capability grants, deny-by-default security">
                            <img src="diagrams/wit_icon.png" alt="WIT" class="tech-icon" />
                            <span>"WIT Contract"</span>
                        </div>
                    </div>
                    
                    <div class="level-components cluster-nodes">
                        <ClusterNode status="blue" name="Pi 4" role="LEADER" />
                        <ClusterNode status="green" name="Pi Zero" role="FOLLOWER" />
                        <ClusterNode status="green" name="Pi Zero" role="FOLLOWER" />
                    </div>
                    <div class="cluster-label">"2oo3 TMR + Raft Consensus"</div>
                </PurdueLevel>
                
                <div class="flow-arrow">"▼"</div>
                
                // level 1: local control (siemens plc)
                <PurdueLevel 
                    level="L1"
                    name="Local Control"
                    tooltip=L1_TOOLTIP
                    class="level-1"
                >
                    <div class="level-components">
                        <HardwareCard icon="🏭" name="Siemens S7-1200" role="Industrial PLC" />
                    </div>
                    <div class="protocol-label">"Modbus RTU via USB-RS485"</div>
                </PurdueLevel>
                
                <div class="flow-arrow">"▼"</div>
                
                // level 0: field devices (sensors and actuators)
                <PurdueLevel 
                    level="L0"
                    name="Field Devices"
                    tooltip=L0_TOOLTIP
                    class="level-0"
                >
                    <div class="level-components">
                        <HardwareCard icon="🌡️" name="BME280" role="Sensor (I2C)" />
                        <HardwareCard icon="💨" name="Industrial Fan" role="Actuator (120V)" />
                    </div>
                </PurdueLevel>
            </div>
        </div>
    }
}

/// purdue level wrapper with tooltip support
#[component]
fn PurdueLevel(
    level: &'static str,
    name: &'static str,
    tooltip: &'static str,
    class: &'static str,
    children: Children,
) -> impl IntoView {
    let full_class = format!("purdue-level has-tooltip {}", class);
    view! {
        <div class={full_class} title={tooltip}>
            <div class="level-header">
                <span class="level-badge">{level}</span>
                <span class="level-name">{name}</span>
                <span class="info-icon">"ⓘ"</span>
            </div>
            <div class="tooltip-content">{tooltip}</div>
            {children()}
        </div>
    }
}

/// reusable hardware card for zone components
#[component]
fn HardwareCard(icon: &'static str, name: &'static str, role: &'static str) -> impl IntoView {
    view! {
        <div class="hw-card">
            <span class="hw-icon">{icon}</span>
            <span class="hw-name">{name}</span>
            <span class="hw-role">{role}</span>
        </div>
    }
}

/// cluster node card with status led indicator
#[component]
fn ClusterNode(status: &'static str, name: &'static str, role: &'static str) -> impl IntoView {
    let status_class = format!("status-led {}", status);
    view! {
        <div class="hw-card node">
            <span class={status_class}>"●"</span>
            <span class="hw-icon">"🍓"</span>
            <span class="hw-name">{name}</span>
            <span class="hw-role">{role}</span>
        </div>
    }
}
