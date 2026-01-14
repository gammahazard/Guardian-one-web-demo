// what: iec 62443 compliance visualization section with click-to-toggle tooltips
// why: demonstrates understanding of industrial security standards with mobile-friendly tooltips
// relations: used by hardware/component.rs, shows security architecture

use leptos::*;

// zone tooltip constants
const ZONE3_TOOLTIP: &str = "Enterprise IT Zone (Low Risk): Contains monitoring and analytics systems. Fully isolated from industrial control via network segmentation.";
const ZONE2_TOOLTIP: &str = "Guardian Cluster DMZ: The WASM runtime acts as a security boundary. Workers are compiled to WebAssembly and execute in a sandboxed environment. WASI (WebAssembly System Interface) provides capability-based security: each module must be explicitly granted access to specific resources like Modbus, GPIO, or Network. This is fundamentally different from containers which inherit host kernel trust.";
const ZONE1_TOOLTIP: &str = "Industrial Control Zone (High Risk): Contains the PLC and field devices. Only validated commands from Zone 2 can reach this zone via Modbus RTU.";

/// renders iec 62443 zone and conduit model diagram with click-to-toggle tooltips
#[component]
pub fn ComplianceSection() -> impl IntoView {
    view! {
        <div class="compliance-section">
            <h3>"IEC 62443 Zone & Conduit Model"</h3>
            <p class="section-hint">"💡 Tap ⓘ for security details"</p>
            
            <div class="compliance-diagram">
                // zone 3: enterprise it (green - low risk)
                <SecurityZone 
                    color="green" 
                    name="Zone 3: Enterprise IT" 
                    desc="Grafana, InfluxDB, QNAP NAS"
                    tooltip=ZONE3_TOOLTIP
                />
                
                <div class="conduit">
                    "┃ Encrypted TLS (Historian API) ┃"
                </div>
                
                // zone 2: dmz / guardian cluster (yellow - medium risk)
                <SecurityZone 
                    color="yellow" 
                    name="Zone 2: DMZ / Guardian Cluster" 
                    desc="WASM Runtime enforces capability boundary"
                    tooltip=ZONE2_TOOLTIP
                />
                
                <div class="conduit">
                    "┃ WIT Contract (Modbus only) ┃"
                </div>
                
                // zone 1: industrial control (red - high risk)
                <SecurityZone 
                    color="red" 
                    name="Zone 1: Industrial Control" 
                    desc="S7-1200 PLC, BME280, Relays"
                    tooltip=ZONE1_TOOLTIP
                />
            </div>
            
            <div class="compliance-note">
                <strong>"Key Security Property: "</strong>
                "The Guardian Cluster acts as a logical data diode / secure gateway. Telemetry flows UP, but no external commands can reach the PLC without WIT contract validation."
            </div>
        </div>
    }
}

/// security zone card with click-to-toggle tooltip
#[component]
fn SecurityZone(
    color: &'static str, 
    name: &'static str, 
    desc: &'static str,
    tooltip: &'static str,
) -> impl IntoView {
    let (show_tooltip, set_show_tooltip) = create_signal(false);
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
                <span class="zone-name">
                    {name}
                    <button 
                        class="info-btn"
                        on:click=move |_| set_show_tooltip.update(|v| *v = !*v)
                    >
                        "ⓘ"
                    </button>
                </span>
                <span class="zone-desc">{desc}</span>
            </div>
            <Show when=move || show_tooltip.get()>
                <div 
                    class="tooltip-overlay" 
                    on:click=move |_| set_show_tooltip.set(false)
                />
                <div class="tooltip-popup">
                    <div class="tooltip-content">{tooltip}</div>
                    <button 
                        class="tooltip-close"
                        on:click=move |_| set_show_tooltip.set(false)
                    >
                        "✕"
                    </button>
                </div>
            </Show>
        </div>
    }
}
