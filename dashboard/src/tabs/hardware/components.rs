// what: hardware component showcase section
// why: displays all physical components grouped by category with role and zone info
// relations: used by hardware/component.rs, lists components from architecture diagram

use leptos::*;

/// renders grid of hardware component cards grouped by category
#[component]
pub fn ComponentsSection() -> impl IntoView {
    view! {
        <div class="components-section">
            <h3>"Hardware Components"</h3>
            
            <div class="components-grid">
                // compute cluster (3 pis for tmr + raft)
                <ComponentCategory title="🖥️ Compute Cluster">
                    <ComponentCard name="Raspberry Pi 4 (4GB)" role="Cluster Leader / Gateway" zone="Level 2" />
                    <ComponentCard name="Pi Zero 2W ×2" role="Raft Followers / Voters" zone="Level 2" />
                </ComponentCategory>
                
                // industrial control (siemens plc + power)
                <ComponentCategory title="🏭 Industrial Control">
                    <ComponentCard name="Siemens S7-1200 PLC" role="Modbus Master Controller" zone="Level 1" />
                    <ComponentCard name="Mean Well 24V PSU" role="PLC Power Supply" zone="Level 1" />
                </ComponentCategory>
                
                // field devices (sensor + actuator)
                <ComponentCategory title="🌡️ Field Devices">
                    <ComponentCard name="BME280 Sensor" role="Temp/Humidity/Pressure (I2C)" zone="Level 0" />
                    <ComponentCard name="SainSmart Relay" role="GPIO Actuator Control" zone="Level 0" />
                    <ComponentCard name="Industrial Fan (120V)" role="Physical Actuator" zone="Level 0" />
                </ComponentCategory>
                
                // visual feedback + protocol bridges
                <ComponentCategory title="💡 Visual & Protocol">
                    <ComponentCard name="WS2812B LED Strip" role="TMR Voting Status" zone="Visual" />
                    <ComponentCard name="RGB OLED Display" role="HMI Dashboard" zone="Visual" />
                    <ComponentCard name="USB-RS485 Adapter" role="Modbus RTU Bridge" zone="Protocol" />
                </ComponentCategory>
                
                // network infrastructure
                <ComponentCategory title="🌐 Infrastructure">
                    <ComponentCard name="UniFi Switch" role="Industrial Zone Segmentation" zone="Network" />
                    <ComponentCard name="QNAP NAS" role="Historian + Raft Storage" zone="Level 3" />
                </ComponentCategory>
            </div>
        </div>
    }
}

/// category wrapper for grouping related components
#[component]
fn ComponentCategory(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="component-category">
            <h4>{title}</h4>
            <div class="component-list">
                {children()}
            </div>
        </div>
    }
}

/// individual component card with name, role, and purdue zone
#[component]
fn ComponentCard(name: &'static str, role: &'static str, zone: &'static str) -> impl IntoView {
    view! {
        <div class="component-card">
            <div class="card-name">{name}</div>
            <div class="card-role">{role}</div>
            <span class="card-zone">{zone}</span>
        </div>
    }
}
