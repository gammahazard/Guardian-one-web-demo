// what: hardware component showcase section with click-to-toggle tooltips
// why: displays all physical components grouped by purdue level with mobile-friendly tooltips
// relations: used by hardware/component.rs, lists components organized by architecture level

use leptos::*;

// category tooltip constants
const L0_CAT_TOOLTIP: &str = "Physical sensors and actuators that interface directly with the industrial process.";
const L1_CAT_TOOLTIP: &str = "Industrial PLC and power systems that execute real-time control logic.";
const L2_CAT_TOOLTIP: &str = "The Guardian Cluster: 3 Raspberry Pis running WASM workers with Raft consensus. For this demo, wasmtime runs natively on Linux. In production, this would typically run inside a Docker container (Mothership pattern) for fleet orchestration.";
const INFRA_CAT_TOOLTIP: &str = "Network infrastructure and data storage supporting the industrial stack.";
const VISUAL_CAT_TOOLTIP: &str = "Visual indicators showing system status: TMR voting results and real-time metrics.";

/// renders grid of hardware component cards grouped by purdue level
#[component]
pub fn ComponentsSection() -> impl IntoView {
    view! {
        <div class="components-section">
            <h3>"Hardware Components"</h3>
            <p class="section-hint">"💡 Organized by Purdue Level — tap ⓘ for details"</p>
            
            <div class="components-grid">
                // level 0: field devices first (bottom of purdue model)
                <ComponentCategory 
                    title="🌡️ L0: Field Devices" 
                    tooltip=L0_CAT_TOOLTIP
                >
                    <ComponentCard 
                        name="BME280 Sensor" 
                        role="Temp/Humidity/Pressure (I2C)" 
                        zone="Level 0"
                    />
                    <ComponentCard 
                        name="SainSmart Relay" 
                        role="GPIO Actuator Control" 
                        zone="Level 0"
                    />
                    <ComponentCard 
                        name="Industrial Fan (120V)" 
                        role="Physical Actuator" 
                        zone="Level 0"
                    />
                </ComponentCategory>
                
                // level 1: industrial control
                <ComponentCategory 
                    title="🏭 L1: Industrial Control" 
                    tooltip=L1_CAT_TOOLTIP
                >
                    <ComponentCard 
                        name="Siemens S7-1200 PLC" 
                        role="Modbus Master Controller" 
                        zone="Level 1"
                    />
                    <ComponentCard 
                        name="Mean Well 24V PSU" 
                        role="PLC Power Supply" 
                        zone="Level 1"
                    />
                </ComponentCategory>
                
                // level 2: guardian cluster (the star of the show)
                <ComponentCategory 
                    title="🖥️ L2: Guardian Cluster" 
                    tooltip=L2_CAT_TOOLTIP
                >
                    <ComponentCard 
                        name="Raspberry Pi 4 (4GB)" 
                        role="Cluster Leader / Gateway" 
                        zone="Level 2"
                    />
                    <ComponentCard 
                        name="Pi Zero 2W ×2" 
                        role="Raft Followers / TMR Voters" 
                        zone="Level 2"
                    />
                </ComponentCategory>
                
                // infrastructure: level 3 + network
                <ComponentCategory 
                    title="🌐 L3: Infrastructure" 
                    tooltip=INFRA_CAT_TOOLTIP
                >
                    <ComponentCard 
                        name="QNAP NAS" 
                        role="Historian + External Audit Log" 
                        zone="Level 3"
                    />
                    <ComponentCard 
                        name="UniFi Switch" 
                        role="Industrial Zone Segmentation" 
                        zone="Network"
                    />
                </ComponentCategory>
                
                // visual feedback
                <ComponentCategory 
                    title="💡 Visual Indicators" 
                    tooltip=VISUAL_CAT_TOOLTIP
                >
                    <ComponentCard 
                        name="WS2812B LED Strip" 
                        role="TMR Voting Status" 
                        zone="Visual"
                    />
                    <ComponentCard 
                        name="RGB OLED Display" 
                        role="HMI Dashboard" 
                        zone="Visual"
                    />
                    <ComponentCard 
                        name="USB-RS485 Adapter" 
                        role="Modbus RTU Bridge" 
                        zone="Protocol"
                    />
                </ComponentCategory>
            </div>
        </div>
    }
}

/// category wrapper with click-to-toggle tooltip
#[component]
fn ComponentCategory(
    title: &'static str, 
    tooltip: &'static str,
    children: Children,
) -> impl IntoView {
    let (show_tooltip, set_show_tooltip) = create_signal(false);
    
    view! {
        <div class="component-category">
            <h4>
                {title}
                <button 
                    class="info-btn"
                    on:click=move |_| set_show_tooltip.update(|v| *v = !*v)
                >
                    "ⓘ"
                </button>
            </h4>
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
            <div class="component-list">
                {children()}
            </div>
        </div>
    }
}

/// individual component card with name, role, and zone
#[component]
fn ComponentCard(
    name: &'static str, 
    role: &'static str, 
    zone: &'static str,
) -> impl IntoView {
    view! {
        <div class="component-card">
            <div class="card-name">{name}</div>
            <div class="card-role">{role}</div>
            <span class="card-zone">{zone}</span>
        </div>
    }
}
