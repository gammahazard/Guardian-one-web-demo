// what: hardware component showcase section with tooltips
// why: displays all physical components grouped by purdue level with educational tooltips
// relations: used by hardware/component.rs, lists components organized by architecture level

use leptos::*;

// category tooltip constants
const L0_CAT_TOOLTIP: &str = "Physical sensors and actuators that interface directly with the industrial process.";
const L1_CAT_TOOLTIP: &str = "Industrial PLC and power systems that execute real-time control logic.";
const L2_CAT_TOOLTIP: &str = "The Guardian Cluster: 3 Raspberry Pis running WASM workers with Raft consensus for fault tolerance.";
const INFRA_CAT_TOOLTIP: &str = "Network infrastructure and data storage supporting the industrial stack.";
const VISUAL_CAT_TOOLTIP: &str = "Visual indicators showing system status: TMR voting results and real-time metrics.";

/// renders grid of hardware component cards grouped by purdue level
#[component]
pub fn ComponentsSection() -> impl IntoView {
    view! {
        <div class="components-section">
            <h3>"Hardware Components"</h3>
            <p class="section-hint">"💡 Organized by Purdue Level — hover for details"</p>
            
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
                        tooltip="Environmental sensor providing real-time temperature, humidity, and barometric pressure readings via I2C bus."
                    />
                    <ComponentCard 
                        name="SainSmart Relay" 
                        role="GPIO Actuator Control" 
                        zone="Level 0"
                        tooltip="Optically-isolated relay module for switching 120V industrial loads safely from 3.3V GPIO signals."
                    />
                    <ComponentCard 
                        name="Industrial Fan (120V)" 
                        role="Physical Actuator" 
                        zone="Level 0"
                        tooltip="Real industrial-grade fan demonstrating actual process control, not a simulation."
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
                        tooltip="Industry-standard programmable logic controller. Programmed via TIA Portal, communicates with Guardian Cluster over Modbus RTU."
                    />
                    <ComponentCard 
                        name="Mean Well 24V PSU" 
                        role="PLC Power Supply" 
                        zone="Level 1"
                        tooltip="Industrial-grade 24VDC power supply for PLC and field devices. DIN-rail mounted."
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
                        tooltip="Raft consensus leader node. Hosts the WASM runtime and WIT contract validator. Bridges enterprise IT to industrial control."
                    />
                    <ComponentCard 
                        name="Pi Zero 2W ×2" 
                        role="Raft Followers / TMR Voters" 
                        zone="Level 2"
                        tooltip="Two follower nodes for 2-out-of-3 Triple Modular Redundancy voting. If one fails, the cluster continues operating."
                    />
                </ComponentCategory>
                
                // infrastructure: level 3 + network
                <ComponentCategory 
                    title="🌐 L3: Infrastructure" 
                    tooltip=INFRA_CAT_TOOLTIP
                >
                    <ComponentCard 
                        name="QNAP NAS" 
                        role="Historian + Raft Storage" 
                        zone="Level 3"
                        tooltip="Stores time-series data, Raft log persistence, and provides network storage for the cluster."
                    />
                    <ComponentCard 
                        name="UniFi Switch" 
                        role="Industrial Zone Segmentation" 
                        zone="Network"
                        tooltip="Managed switch providing VLAN isolation between enterprise IT and industrial control networks."
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
                        tooltip="NeoPixel strip showing real-time TMR voting status: green=consensus, yellow=degraded, red=fault."
                    />
                    <ComponentCard 
                        name="RGB OLED Display" 
                        role="HMI Dashboard" 
                        zone="Visual"
                        tooltip="Small OLED screen displaying current sensor values, cluster health, and Raft leader status."
                    />
                    <ComponentCard 
                        name="USB-RS485 Adapter" 
                        role="Modbus RTU Bridge" 
                        zone="Protocol"
                        tooltip="Serial-to-RS485 adapter enabling Modbus RTU communication between Pi and PLC."
                    />
                </ComponentCategory>
            </div>
        </div>
    }
}

/// category wrapper for grouping related components with tooltip
#[component]
fn ComponentCategory(
    title: &'static str, 
    tooltip: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="component-category has-tooltip" title={tooltip}>
            <h4>
                {title}
                <span class="info-icon">"ⓘ"</span>
            </h4>
            <div class="tooltip-content">{tooltip}</div>
            <div class="component-list">
                {children()}
            </div>
        </div>
    }
}

/// individual component card with name, role, zone, and tooltip
#[component]
fn ComponentCard(
    name: &'static str, 
    role: &'static str, 
    zone: &'static str,
    tooltip: &'static str,
) -> impl IntoView {
    view! {
        <div class="component-card has-tooltip" title={tooltip}>
            <div class="card-name">{name}</div>
            <div class="card-role">{role}</div>
            <span class="card-zone">{zone}</span>
        </div>
    }
}
