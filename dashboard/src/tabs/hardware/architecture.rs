// what: purdue model architecture visualization component
// why: shows iec 62443 zones with actual hardware placement for credibility
// relations: used by hardware/component.rs as one of four sub-sections

use leptos::*;

/// renders the purdue model zones diagram showing hardware at each level
#[component]
pub fn ArchitectureSection() -> impl IntoView {
    view! {
        <div class="architecture-section">
            <h3>"Purdue Model — IEC 62443 Zones"</h3>
            
            <div class="purdue-diagram">
                // level 3: operations management (enterprise it)
                <div class="purdue-level level-3">
                    <div class="level-header">
                        <span class="level-badge">"L3"</span>
                        <span class="level-name">"Operations Management"</span>
                    </div>
                    <div class="level-components">
                        <HardwareCard icon="📊" name="QNAP NAS" role="Historian" />
                        <HardwareCard icon="📈" name="InfluxDB" role="Time-Series" />
                        <HardwareCard icon="🖥️" name="Grafana" role="Dashboard" />
                    </div>
                </div>
                
                <div class="flow-arrow">"▼"</div>
                
                // level 2: supervisory control (guardian cluster with 3 pis)
                <div class="purdue-level level-2 guardian-zone">
                    <div class="level-header">
                        <span class="level-badge">"L2"</span>
                        <span class="level-name">"Supervisory Control — Guardian Cluster"</span>
                    </div>
                    
                    // wit and wasm technology badges
                    <div class="tech-badges">
                        <div class="tech-badge">
                            <img src="diagrams/wasm_icon.png" alt="WASM" class="tech-icon" />
                            <span>"WASM Runtime"</span>
                        </div>
                        <div class="tech-badge">
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
                </div>
                
                <div class="flow-arrow">"▼"</div>
                
                // level 1: local control (siemens plc)
                <div class="purdue-level level-1">
                    <div class="level-header">
                        <span class="level-badge">"L1"</span>
                        <span class="level-name">"Local Control"</span>
                    </div>
                    <div class="level-components">
                        <HardwareCard icon="🏭" name="Siemens S7-1200" role="Industrial PLC" />
                    </div>
                    <div class="protocol-label">"Modbus RTU via USB-RS485"</div>
                </div>
                
                <div class="flow-arrow">"▼"</div>
                
                // level 0: field devices (sensors and actuators)
                <div class="purdue-level level-0">
                    <div class="level-header">
                        <span class="level-badge">"L0"</span>
                        <span class="level-name">"Field Devices"</span>
                    </div>
                    <div class="level-components">
                        <HardwareCard icon="🌡️" name="BME280" role="Sensor (I2C)" />
                        <HardwareCard icon="💨" name="Industrial Fan" role="Actuator (120V)" />
                    </div>
                </div>
            </div>
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
