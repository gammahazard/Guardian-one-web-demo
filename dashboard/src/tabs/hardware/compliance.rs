// what: iec 62443 compliance visualization section
// why: demonstrates understanding of industrial security standards and zone model
// relations: used by hardware/component.rs, shows security architecture from architecture diagram

use leptos::*;

/// renders iec 62443 zone and conduit model diagram
#[component]
pub fn ComplianceSection() -> impl IntoView {
    view! {
        <div class="compliance-section">
            <h3>"IEC 62443 Zone & Conduit Model"</h3>
            
            <div class="compliance-diagram">
                // zone 3: enterprise it (green - low risk)
                <SecurityZone 
                    color="green" 
                    name="Zone 3: Enterprise IT" 
                    desc="Grafana, InfluxDB, QNAP NAS" 
                />
                
                <div class="conduit">"┃ Encrypted TLS (Historian API) ┃"</div>
                
                // zone 2: dmz / guardian cluster (yellow - medium risk)
                <SecurityZone 
                    color="yellow" 
                    name="Zone 2: DMZ / Guardian Cluster" 
                    desc="WASM Runtime enforces capability boundary" 
                />
                
                <div class="conduit">"┃ WIT Contract (Modbus only) ┃"</div>
                
                // zone 1: industrial control (red - high risk)
                <SecurityZone 
                    color="red" 
                    name="Zone 1: Industrial Control" 
                    desc="S7-1200 PLC, BME280, Relays" 
                />
            </div>
            
            <div class="compliance-note">
                <strong>"Key Security Property: "</strong>
                "The Guardian Cluster acts as a one-way data diode. Telemetry flows UP, but no external commands can reach the PLC without WIT contract validation."
            </div>
        </div>
    }
}

/// security zone card with color-coded risk level
#[component]
fn SecurityZone(color: &'static str, name: &'static str, desc: &'static str) -> impl IntoView {
    let badge = match color {
        "green" => "🟢",
        "yellow" => "🟡",
        "red" => "🔴",
        _ => "⚪",
    };
    let zone_class = format!("zone {}", color);
    
    view! {
        <div class={zone_class}>
            <span class="zone-badge">{badge}</span>
            <div class="zone-info">
                <span class="zone-name">{name}</span>
                <span class="zone-desc">{desc}</span>
            </div>
        </div>
    }
}
