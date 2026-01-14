// what: docker vulnerabilities and ics attack statistics section
// why: provides concrete evidence of container security issues
// relations: used by problem/component.rs as a sub-section

use leptos::*;

// CVE data from research
const CVE_LEAKY_VESSELS: &str = "CVE-2024-21626";
const CVE_LEAKY_DESC: &str = "runc container escape — attackers can access host filesystem via file descriptor manipulation";
const CVE_LEAKY_SCORE: &str = "8.6";

const CVE_BUILDKIT_RACE: &str = "CVE-2024-23651";
const CVE_BUILDKIT_DESC: &str = "Symlink race condition during Docker build allows reading host files";
const CVE_BUILDKIT_SCORE: &str = "7.4";

const CVE_DESKTOP: &str = "CVE-2025-9074";
const CVE_DESKTOP_DESC: &str = "Docker Desktop escape — malicious container gains full host access without authentication";
const CVE_DESKTOP_SCORE: &str = "9.3";

/// renders the vulnerabilities section with CVE cards and ICS stats
#[component]
pub fn VulnerabilitiesSection() -> impl IntoView {
    view! {
        <div class="vulnerabilities-section">
            <h3>"🔓 Container Escape Vulnerabilities"</h3>
            <p class="section-hint">"Real CVEs from 2024-2025 — container escapes happen regularly"</p>
            
            <div class="cve-cards">
                <CveCard 
                    cve=CVE_LEAKY_VESSELS
                    name="Leaky Vessels"
                    desc=CVE_LEAKY_DESC
                    score=CVE_LEAKY_SCORE
                    severity="critical"
                />
                <CveCard 
                    cve=CVE_BUILDKIT_RACE
                    name="Buildkit Race"
                    desc=CVE_BUILDKIT_DESC
                    score=CVE_BUILDKIT_SCORE
                    severity="high"
                />
                <CveCard 
                    cve=CVE_DESKTOP
                    name="Desktop Escape"
                    desc=CVE_DESKTOP_DESC
                    score=CVE_DESKTOP_SCORE
                    severity="critical"
                />
            </div>
            
            <div class="ics-stats">
                <h4>"📊 ICS Attack Landscape (CISA 2023-2024)"</h4>
                <div class="stat-cards">
                    <StatCard 
                        value="40%"
                        label="increase in internet-exposed ICS devices"
                        source="CISA/SocRadar 2024"
                    />
                    <StatCard 
                        value="33%"
                        label="of ICS vulnerabilities have no available patch"
                        source="SecurityWeek/CISA H1 2023"
                    />
                    <StatCard 
                        value="44%"
                        label="of ICS attacks target critical manufacturing"
                        source="IndustrialCyber 2023"
                    />
                </div>
            </div>
            
            <div class="wasm-contrast">
                <p>
                    <strong>"With WASM: "</strong>
                    "Container escapes are architecturally impossible. There's no host kernel to escape to — the sandbox is at the instruction level, not process level."
                </p>
            </div>
        </div>
    }
}

/// individual CVE card with severity badge
#[component]
fn CveCard(
    cve: &'static str,
    name: &'static str,
    desc: &'static str,
    score: &'static str,
    severity: &'static str,
) -> impl IntoView {
    let badge_class = format!("severity-badge {}", severity);
    view! {
        <div class="cve-card">
            <div class="cve-header">
                <span class="cve-id">{cve}</span>
                <span class={badge_class}>"CVSS " {score}</span>
            </div>
            <div class="cve-name">{name}</div>
            <p class="cve-desc">{desc}</p>
        </div>
    }
}

/// statistic card for ICS data
#[component]
fn StatCard(
    value: &'static str,
    label: &'static str,
    source: &'static str,
) -> impl IntoView {
    view! {
        <div class="stat-card">
            <span class="stat-value">{value}</span>
            <span class="stat-label">{label}</span>
            <span class="stat-source">{source}</span>
        </div>
    }
}
