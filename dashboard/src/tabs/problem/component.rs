// what: problem tab main component orchestrating sub-sections
// why: sets up the narrative explaining why WASM/WASI matters for ICS security
// relations: uses quotes, vulnerabilities, and comparison sub-components

use leptos::*;
use super::quotes::QuotesSection;
use super::vulnerabilities::VulnerabilitiesSection;
use super::comparison::ComparisonSection;

/// main problem tab component with vertical story flow
#[component]
pub fn Problem() -> impl IntoView {
    view! {
        <div class="tab-content problem-tab">
            <h2>"The Problem: Why Industrial Edge Security is Broken"</h2>
            <p class="tab-intro">
                "Docker solves cloud orchestration. WASM solves the last mile: secure, bandwidth-efficient logic "
                "that runs inside your containers. Industrial edge needs both."
            </p>
            
            // section 1: the tweets that started the conversation
            <QuotesSection />
            
            // section 2: real vulnerabilities and ICS attack data
            <VulnerabilitiesSection />
            
            // section 3: comparison table and CTA
            <ComparisonSection />
        </div>
    }
}
