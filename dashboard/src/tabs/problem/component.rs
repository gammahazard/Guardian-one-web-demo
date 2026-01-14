// what: problem tab component explaining why traditional ics security fails
// why: sets up the narrative before showing the solution
// relations: used by mod.rs, first tab in story flow

use leptos::*;

#[component]
pub fn Problem() -> impl IntoView {
    view! {
        <div class="tab-content problem-tab">
            <h2>"The Problem: Why Traditional ICS Security Fails"</h2>
            
            <div class="comparison-grid">
                <div class="comparison-card docker">
                    <h3>"🐳 Docker Container"</h3>
                    <ul>
                        <li>"Linux kernel: ~25M lines of code"</li>
                        <li>"Container escape CVEs: 50+ in 2024"</li>
                        <li>"Network access by default"</li>
                        <li>"Image size: 50-200 MB"</li>
                    </ul>
                </div>
                
                <div class="comparison-card wasm">
                    <h3>"🦀 WASM Sandbox"</h3>
                    <ul>
                        <li>"Runtime: ~50K lines of code"</li>
                        <li>"No syscalls by default"</li>
                        <li>"Network = capability grant"</li>
                        <li>"Binary size: 15-70 KB"</li>
                    </ul>
                </div>
            </div>
            
            <div class="attack-examples">
                <h3>"Real-World ICS Attacks"</h3>
                <ul>
                    <li><strong>"Triton (2017)"</strong>" — Malware targeting safety systems"</li>
                    <li><strong>"Colonial Pipeline (2021)"</strong>" — Ransomware disrupting fuel supply"</li>
                    <li><strong>"Industroyer2 (2022)"</strong>" — Ukraine power grid attacks"</li>
                </ul>
            </div>
        </div>
    }
}
