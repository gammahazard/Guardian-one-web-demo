// what: main hardware tab component with section navigation
// why: orchestrates the four hardware sub-sections with tabbed ui
// relations: uses architecture.rs, components.rs, compliance.rs, toolchain.rs
//            exported by mod.rs for use in main app tabs

use leptos::*;
use super::architecture::ArchitectureSection;
use super::components::ComponentsSection;
use super::compliance::ComplianceSection;
use super::toolchain::ToolchainSection;

/// main hardware tab with sub-section navigation
#[component]
pub fn Hardware() -> impl IntoView {
    // state for active section
    let (active_section, set_active_section) = create_signal("architecture");

    view! {
        <div class="tab-content hardware-tab">
            <h2>"The Hardware: Industrial Edge Architecture"</h2>
            <p class="hardware-intro">
                "Real hardware. Real protocols. No simulation."
            </p>
            
            // section navigation buttons
            <div class="section-nav">
                <SectionButton 
                    id="architecture" 
                    label="🏗️ Architecture" 
                    active=active_section 
                    set_active=set_active_section 
                />
                <SectionButton 
                    id="components" 
                    label="🔧 Components" 
                    active=active_section 
                    set_active=set_active_section 
                />
                <SectionButton 
                    id="compliance" 
                    label="📋 IEC 62443" 
                    active=active_section 
                    set_active=set_active_section 
                />
                <SectionButton 
                    id="toolchain" 
                    label="⚙️ TIA Portal" 
                    active=active_section 
                    set_active=set_active_section 
                />
            </div>

            // section content (renders based on active section)
            <div class="section-content">
                {move || match active_section.get() {
                    "architecture" => view! { <ArchitectureSection /> }.into_view(),
                    "components" => view! { <ComponentsSection /> }.into_view(),
                    "compliance" => view! { <ComplianceSection /> }.into_view(),
                    "toolchain" => view! { <ToolchainSection /> }.into_view(),
                    _ => view! { <ArchitectureSection /> }.into_view(),
                }}
            </div>
        </div>
    }
}

/// reusable section navigation button
#[component]
fn SectionButton(
    id: &'static str,
    label: &'static str,
    active: ReadSignal<&'static str>,
    set_active: WriteSignal<&'static str>,
) -> impl IntoView {
    view! {
        <button 
            class=move || if active.get() == id { "section-btn active" } else { "section-btn" }
            on:click=move |_| set_active.set(id)
        >
            {label}
        </button>
    }
}
