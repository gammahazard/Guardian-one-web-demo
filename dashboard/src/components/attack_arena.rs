// what: attack simulation buttons for demonstrating security boundaries
// why: shows how wasm traps vs python crashes on malicious input
// relations: used by tabs/demo.rs

use leptos::*;

#[component]
pub fn AttackArena() -> impl IntoView {
    let (attack_result, set_attack_result) = create_signal(String::new());
    
    let simulate_buffer_overflow = move |_| {
        set_attack_result.set("Buffer Overflow: WASM traps safely, Python segfaults".to_string());
    };
    
    let simulate_data_exfil = move |_| {
        set_attack_result.set("Data Exfiltration: WASM blocked (no network cap), Python succeeds".to_string());
    };
    
    let simulate_path_traversal = move |_| {
        set_attack_result.set("Path Traversal: WASM blocked (no FS cap), Python reads /etc/passwd".to_string());
    };

    view! {
        <div class="attack-arena">
            <h3>"Attack Scenarios"</h3>
            
            <div class="attack-buttons">
                <button class="attack-btn" on:click=simulate_buffer_overflow>
                    "💥 Buffer Overflow"
                </button>
                <button class="attack-btn" on:click=simulate_data_exfil>
                    "📤 Data Exfiltration"
                </button>
                <button class="attack-btn" on:click=simulate_path_traversal>
                    "📁 Path Traversal"
                </button>
            </div>
            
            <div class="attack-result" class:visible=move || !attack_result.get().is_empty()>
                <p>{move || attack_result.get()}</p>
            </div>
        </div>
    }
}
