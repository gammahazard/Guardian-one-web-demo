// what: ota update bandwidth comparison simulator
// why: demonstrates the key business case for wasm - bandwidth savings on ota updates
// relations: used by proof/component.rs, imported via proof/mod.rs

use leptos::*;

// ============================================================================
// network type constants
// ============================================================================

// network speeds in mbps
const ETHERNET_SPEED_MBPS: f64 = 100.0;
const CELLULAR_SPEED_MBPS: f64 = 10.0;
const SATELLITE_SPEED_MBPS: f64 = 1.0;

// network costs per MB in USD
const ETHERNET_COST_PER_MB: f64 = 0.001;
const CELLULAR_COST_PER_MB: f64 = 0.10;
const SATELLITE_COST_PER_MB: f64 = 10.0;

// update sizes in MB
const DOCKER_UPDATE_SIZE_MB: f64 = 50.0;  // minimal alpine + python app
const WASM_UPDATE_SIZE_MB: f64 = 0.05;    // 50KB compiled rust module

// ============================================================================
// network type enum
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    Ethernet,
    Cellular,
    Satellite,
}

impl NetworkType {
    fn speed_mbps(&self) -> f64 {
        match self {
            NetworkType::Ethernet => ETHERNET_SPEED_MBPS,
            NetworkType::Cellular => CELLULAR_SPEED_MBPS,
            NetworkType::Satellite => SATELLITE_SPEED_MBPS,
        }
    }
    
    fn cost_per_mb(&self) -> f64 {
        match self {
            NetworkType::Ethernet => ETHERNET_COST_PER_MB,
            NetworkType::Cellular => CELLULAR_COST_PER_MB,
            NetworkType::Satellite => SATELLITE_COST_PER_MB,
        }
    }
}

// ============================================================================
// calculation helpers
// ============================================================================

/// calculates download time in seconds for given size (MB) and speed (Mbps)
fn calc_download_time_secs(size_mb: f64, speed_mbps: f64) -> f64 {
    // MB to Mb = multiply by 8
    let size_mbits = size_mb * 8.0;
    size_mbits / speed_mbps
}

/// formats time in human readable format
fn format_time(secs: f64) -> String {
    if secs < 1.0 {
        format!("{:.0}ms", secs * 1000.0)
    } else if secs < 60.0 {
        format!("{:.1}s", secs)
    } else if secs < 3600.0 {
        format!("{:.1} min", secs / 60.0)
    } else {
        format!("{:.1} hrs", secs / 3600.0)
    }
}

/// formats currency
fn format_currency(amount: f64) -> String {
    if amount < 1.0 {
        format!("${:.2}", amount)
    } else if amount < 1000.0 {
        format!("${:.0}", amount)
    } else if amount < 1_000_000.0 {
        format!("${:.1}K", amount / 1000.0)
    } else {
        format!("${:.2}M", amount / 1_000_000.0)
    }
}

// ============================================================================
// main component
// ============================================================================

/// interactive ota update comparison simulator
#[component]
pub fn OtaSimulator() -> impl IntoView {
    // state signals
    let (fleet_size, set_fleet_size) = create_signal(1000u32);
    let (network_type, set_network_type) = create_signal(NetworkType::Cellular);
    
    // derived calculations
    let docker_time_per_device = move || {
        calc_download_time_secs(DOCKER_UPDATE_SIZE_MB, network_type.get().speed_mbps())
    };
    
    let wasm_time_per_device = move || {
        calc_download_time_secs(WASM_UPDATE_SIZE_MB, network_type.get().speed_mbps())
    };
    
    let docker_total_bandwidth_mb = move || {
        DOCKER_UPDATE_SIZE_MB * fleet_size.get() as f64
    };
    
    let wasm_total_bandwidth_mb = move || {
        WASM_UPDATE_SIZE_MB * fleet_size.get() as f64
    };
    
    let docker_cost = move || {
        docker_total_bandwidth_mb() * network_type.get().cost_per_mb()
    };
    
    let wasm_cost = move || {
        wasm_total_bandwidth_mb() * network_type.get().cost_per_mb()
    };
    
    let yearly_savings = move || {
        // assume 12 updates per year
        (docker_cost() - wasm_cost()) * 12.0
    };
    
    let bandwidth_ratio = move || {
        DOCKER_UPDATE_SIZE_MB / WASM_UPDATE_SIZE_MB
    };

    view! {
        <div class="ota-simulator">
            <h3>"📦 OTA Update Comparison"</h3>
            <p class="section-desc">"Compare bandwidth and cost for fleet-wide updates"</p>
            
            // controls row
            <div class="ota-controls">
                <div class="control-group">
                    <label>"Fleet Size: "<strong>{fleet_size}</strong>" devices"</label>
                    <input 
                        type="range" 
                        min="100" 
                        max="10000" 
                        step="100"
                        class="fleet-slider"
                        prop:value=move || fleet_size.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<u32>().unwrap_or(1000);
                            set_fleet_size.set(val);
                        }
                    />
                    <div class="slider-labels">
                        <span>"100"</span>
                        <span>"10,000"</span>
                    </div>
                </div>
                
                <div class="control-group">
                    <label>"Network Type"</label>
                    <select 
                        class="network-select"
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            let net = match val.as_str() {
                                "ethernet" => NetworkType::Ethernet,
                                "satellite" => NetworkType::Satellite,
                                _ => NetworkType::Cellular,
                            };
                            set_network_type.set(net);
                        }
                    >
                        <option value="ethernet">"Ethernet (100 Mbps) - $0.001/MB"</option>
                        <option value="cellular" selected>"Cellular 4G (10 Mbps) - $0.10/MB"</option>
                        <option value="satellite">"Satellite (1 Mbps) - $10/MB"</option>
                    </select>
                </div>
            </div>
            
            // comparison cards
            <div class="ota-comparison">
                <div class="ota-card docker">
                    <div class="ota-card-header">
                        <span class="ota-icon">"🐳"</span>
                        <span class="ota-title">"Docker Update"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value">"50 MB"</span>
                        <span class="ota-label">"per device"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value">{move || format_time(docker_time_per_device())}</span>
                        <span class="ota-label">"download time"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value warning">{move || format!("{:.0} GB", docker_total_bandwidth_mb() / 1000.0)}</span>
                        <span class="ota-label">"total bandwidth"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value warning">{move || format_currency(docker_cost())}</span>
                        <span class="ota-label">"per update cycle"</span>
                    </div>
                </div>
                
                <div class="ota-card wasm">
                    <div class="ota-card-header">
                        <span class="ota-icon">"🦀"</span>
                        <span class="ota-title">"WASM Update"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value">"50 KB"</span>
                        <span class="ota-label">"per device"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value success">{move || format_time(wasm_time_per_device())}</span>
                        <span class="ota-label">"download time"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value success">{move || format!("{:.0} MB", wasm_total_bandwidth_mb())}</span>
                        <span class="ota-label">"total bandwidth"</span>
                    </div>
                    <div class="ota-stat">
                        <span class="ota-value success">{move || format_currency(wasm_cost())}</span>
                        <span class="ota-label">"per update cycle"</span>
                    </div>
                </div>
            </div>
            
            // savings summary
            <div class="ota-savings">
                <div class="savings-stat">
                    <span class="savings-value">{move || format!("{:.0}x", bandwidth_ratio())}</span>
                    <span class="savings-label">"smaller updates"</span>
                </div>
                <div class="savings-stat highlight">
                    <span class="savings-value">{move || format_currency(yearly_savings())}</span>
                    <span class="savings-label">"yearly savings (12 updates)"</span>
                </div>
            </div>
            
            <p class="ota-note">
                "💡 "<em>"For remote sites on satellite/cellular, WASM's smaller footprint translates directly to lower operational costs."</em>
            </p>
        </div>
    }
}
