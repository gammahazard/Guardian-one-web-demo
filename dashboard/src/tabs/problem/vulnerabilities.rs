// what: docker vulnerabilities and ics attack statistics section
// why: provides concrete evidence of container security issues
// relations: used by problem/component.rs as a sub-section

use leptos::*;

// CVE data from research
const CVE_LEAKY_VESSELS: &str = "CVE-2024-21626";
const CVE_LEAKY_DESC: &str = "runc container escape — attackers can access host filesystem via file descriptor manipulation";
const CVE_LEAKY_SCORE: &str = "8.6";
const CVE_LEAKY_URL: &str = "https://nvd.nist.gov/vuln/detail/CVE-2024-21626";

const CVE_BUILDKIT_RACE: &str = "CVE-2024-23651";
const CVE_BUILDKIT_DESC: &str = "Symlink race condition during Docker build allows reading host files";
const CVE_BUILDKIT_SCORE: &str = "7.4";
const CVE_BUILDKIT_URL: &str = "https://nvd.nist.gov/vuln/detail/CVE-2024-23651";

const CVE_DESKTOP: &str = "CVE-2025-9074";
const CVE_DESKTOP_DESC: &str = "Docker Desktop escape — malicious container gains full host access without authentication";
const CVE_DESKTOP_SCORE: &str = "9.3";
const CVE_DESKTOP_URL: &str = "https://nvd.nist.gov/vuln/detail/CVE-2025-9074";

// ICS stats with source URLs
const STAT1_URL: &str = "https://socradar.io/blog/cisa-industrial-control-systems-ics-advisories-2025/";
const STAT2_URL: &str = "https://www.securityweek.com/670-ics-vulnerabilities-disclosed-by-cisa-in-first-half-of-2023-analysis/";
const STAT3_URL: &str = "https://industrialcyber.co/industrial-cyber-attacks/new-ics-vulnerabilities-report-highlights-trends-and-increases-in-cves-despite-fewer-cisa-advisories/";
const STAT4_URL: &str = "https://media.txone.com/prod/uploads/2024/02/TXOne-Annual-Report-OT-ICS-Cybersecurity-2023-v.pdf";

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
                    url=CVE_LEAKY_URL
                />
                <CveCard 
                    cve=CVE_BUILDKIT_RACE
                    name="Buildkit Race"
                    desc=CVE_BUILDKIT_DESC
                    score=CVE_BUILDKIT_SCORE
                    severity="high"
                    url=CVE_BUILDKIT_URL
                />
                <CveCard 
                    cve=CVE_DESKTOP
                    name="Desktop Escape"
                    desc=CVE_DESKTOP_DESC
                    score=CVE_DESKTOP_SCORE
                    severity="critical"
                    url=CVE_DESKTOP_URL
                />
            </div>
            
            <div class="ics-stats">
                <h4>"📊 ICS Attack Landscape (CISA 2023-2024)"</h4>
                <div class="stat-cards">
                    <StatCard 
                        value="40%"
                        label="increase in internet-exposed ICS devices"
                        source="SocRadar 2024"
                        url=STAT1_URL
                    />
                    <StatCard 
                        value="33%"
                        label="of ICS vulnerabilities have no available patch"
                        source="SecurityWeek 2023"
                        url=STAT2_URL
                    />
                    <StatCard 
                        value="44%"
                        label="of ICS vulnerabilities are in manufacturing equipment"
                        source="ICS Advisory Project 2023"
                        url=STAT3_URL
                    />
                    <StatCard 
                        value="97%"
                        label="of IT incidents eventually impact OT environments"
                        source="TXOne 2023 Report"
                        url=STAT4_URL
                    />
                </div>
            </div>
            
            <div class="wasm-contrast">
                <p>
                    <strong>"With WASM: "</strong>
                    "The attack surface shifts from the kernel to the runtime. No syscalls, no filesystem, no network — unless explicitly granted via capability handles. The sandbox is at the instruction level."
                </p>
            </div>
        </div>
    }
}

/// individual CVE card with severity badge and NVD link
#[component]
fn CveCard(
    cve: &'static str,
    name: &'static str,
    desc: &'static str,
    score: &'static str,
    severity: &'static str,
    url: &'static str,
) -> impl IntoView {
    let badge_class = format!("severity-badge {}", severity);
    view! {
        <div class="cve-card">
            <div class="cve-header">
                <a href={url} target="_blank" rel="noopener" class="cve-id">{cve}</a>
                <span class={badge_class}>"CVSS " {score}</span>
            </div>
            <div class="cve-name">{name}</div>
            <p class="cve-desc">{desc}</p>
        </div>
    }
}

/// statistic card for ICS data with clickable source link
#[component]
fn StatCard(
    value: &'static str,
    label: &'static str,
    source: &'static str,
    url: &'static str,
) -> impl IntoView {
    view! {
        <div class="stat-card">
            <span class="stat-value">{value}</span>
            <span class="stat-label">{label}</span>
            <a href={url} target="_blank" rel="noopener" class="stat-source">{source}" →"</a>
        </div>
    }
}
