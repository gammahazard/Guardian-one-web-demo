// what: tia portal integration section with click-to-toggle tooltips
// why: shows professional engineering workflow using real industrial tools
// relations: used by hardware/component.rs, demonstrates enterprise integration

use leptos::*;

// toolchain tooltips
const TIA_TOOLTIP: &str = "Siemens TIA Portal is industry-standard PLC programming software. It provides ladder logic programming, device configuration, and live monitoring. Using real engineering tools (not hobbyist alternatives) demonstrates enterprise readiness.";
const PLC_TOOLTIP: &str = "The S7-1200 receives ladder logic programs via TIA Portal over Ethernet. Once programmed, it operates autonomously, executing control logic and communicating with the Guardian Cluster via Modbus RTU.";
const GUARDIAN_TOOLTIP: &str = "The Guardian Cluster intercepts all Modbus traffic. WIT contracts define exactly which Modbus registers can be read/written. Deny-by-default: any capability not explicitly granted is blocked. This is impossible with traditional Docker containers.";

/// renders tia portal integration diagram with click-to-toggle tooltips
#[component]
pub fn ToolchainSection() -> impl IntoView {
    view! {
        <div class="toolchain-section">
            <h3>"TIA Portal Integration"</h3>
            <p class="toolchain-intro">"Real industrial engineering tools — the same software used by automation professionals worldwide."</p>
            <p class="section-hint">"💡 Tap ⓘ for details"</p>
            
            <div class="toolchain-diagram">
                // engineering workstation with tia portal
                <ToolBox 
                    icon="💻" 
                    name="TIA Portal V20" 
                    desc="(Trial License)"
                    tooltip=TIA_TOOLTIP
                    features=vec![
                        "Program S7-1200 ladder logic",
                        "Configure Modbus TCP/RTU",
                        "Monitor I/O in real-time",
                    ]
                />
                
                <div class="tool-flow">"↓ Ethernet Download"</div>
                
                // the plc
                <ToolBox 
                    icon="🏭" 
                    name="Siemens S7-1200 PLC"
                    desc="Industrial Controller"
                    tooltip=PLC_TOOLTIP
                    features=vec![
                        "Executes ladder logic autonomously",
                        "Modbus RTU ↔ Guardian Cluster",
                    ]
                />
                
                <div class="tool-flow">"↓ Modbus RTU (Validated)"</div>
                
                // guardian cluster
                <ToolBox 
                    icon="🛡️" 
                    name="Guardian Cluster"
                    desc="WIT Contract Validation"
                    tooltip=GUARDIAN_TOOLTIP
                    features=vec![
                        "WASM sandbox for every worker",
                        "WIT contracts: deny-by-default",
                        "Byzantine fault tolerance (2oo3)",
                    ]
                />
            </div>
            
            <div class="toolchain-value">
                <h4>"Why This Matters"</h4>
                <ul>
                    <li>"Same software industrial engineers use daily"</li>
                    <li>"Industry-standard PLC programming environment"</li>
                    <li>"Full visibility: PLC program + Guardian interception"</li>
                </ul>
            </div>
        </div>
    }
}

/// toolchain item box with click-to-toggle tooltip
#[component]
fn ToolBox(
    icon: &'static str,
    name: &'static str,
    desc: &'static str,
    tooltip: &'static str,
    features: Vec<&'static str>,
) -> impl IntoView {
    let (show_tooltip, set_show_tooltip) = create_signal(false);
    
    view! {
        <div class="tool-box">
            <span class="tool-icon">{icon}</span>
            <span class="tool-name">
                {name}
                <button 
                    class="info-btn"
                    on:click=move |_| set_show_tooltip.update(|v| *v = !*v)
                >
                    "ⓘ"
                </button>
            </span>
            <span class="tool-desc">{desc}</span>
            <Show when=move || show_tooltip.get()>
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
            <ul class="tool-features">
                {features.into_iter().map(|f| view! {
                    <li>{f}</li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
