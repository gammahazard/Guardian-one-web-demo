// what: tia portal toolchain integration section
// why: shows real industrial engineering software usage for professional credibility
// relations: used by hardware/component.rs, demonstrates siemens ecosystem integration

use leptos::*;

/// renders tia portal integration diagram showing engineering workflow
#[component]
pub fn ToolchainSection() -> impl IntoView {
    view! {
        <div class="toolchain-section">
            <h3>"TIA Portal Integration"</h3>
            <p class="toolchain-intro">
                "Real industrial engineering tools — the same software used by automation professionals worldwide."
            </p>
            
            <div class="toolchain-diagram">
                // engineering workstation with tia portal
                <ToolBox 
                    icon="💻" 
                    name="TIA Portal V20" 
                    desc="(Trial License)"
                    features=vec![
                        "Program S7-1200 ladder logic",
                        "Configure Modbus TCP/RTU", 
                        "Monitor I/O in real-time"
                    ]
                />
                
                <div class="tool-flow">"Ethernet ↓"</div>
                
                // siemens plc
                <ToolBox 
                    icon="🏭" 
                    name="Siemens S7-1200 PLC" 
                    desc="Temperature Control Logic"
                    features=vec![]
                />
                
                <div class="tool-flow">"Modbus RTU (RS485) ↓"</div>
                
                // guardian cluster for validation
                <ToolBox 
                    icon="🔒" 
                    name="Guardian Cluster" 
                    desc="WIT Contract Validation"
                    features=vec![]
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

/// reusable tool/software box component
#[component]
fn ToolBox(
    icon: &'static str, 
    name: &'static str, 
    desc: &'static str,
    #[prop(optional)] features: Vec<&'static str>
) -> impl IntoView {
    view! {
        <div class="tool-box">
            <span class="tool-icon">{icon}</span>
            <span class="tool-name">{name}</span>
            <span class="tool-desc">{desc}</span>
            {if !features.is_empty() {
                view! {
                    <ul class="tool-features">
                        {features.into_iter().map(|f| view! { <li>{f}</li> }).collect_view()}
                    </ul>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}
        </div>
    }
}
