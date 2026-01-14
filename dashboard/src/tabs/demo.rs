// what: interactive demo tab comparing python vs wasm execution
// why: flagship demonstration merging ICS Guardian, Protocol Gateway, and Raft patterns
// relations: used by lib.rs, demonstrates 2oo3 voting, worker pools, and capability-based security

use leptos::*;
use wasm_bindgen::prelude::*;

// ============================================================================
// types and configuration
// ============================================================================

/// log entry for terminal output display
#[derive(Clone)]
struct LogEntry {
    level: String,   // info, success, warn, error
    message: String,
}

/// attack configuration with realistic python restart times
struct AttackConfig {
    name: &'static str,
    restart_ms: u32,          // python worker restart time (ms)
    error_msg: &'static str,
    wasm_trap: &'static str,
}

/// wasm instance state for 2oo3 voting visualization
#[derive(Clone, Copy, PartialEq)]
enum InstanceState {
    Healthy,
    Faulty,
}

fn get_attack_config(attack: &str) -> AttackConfig {
    match attack {
        "bufferOverflow" => AttackConfig {
            name: "Buffer Overflow",
            restart_ms: 1800,
            error_msg: "Segmentation fault (core dumped)",
            wasm_trap: "out of bounds memory access",
        },
        "dataExfil" => AttackConfig {
            name: "Data Exfiltration",
            restart_ms: 2100,
            error_msg: "Unauthorized network connection",
            wasm_trap: "capability not granted: network",
        },
        "pathTraversal" => AttackConfig {
            name: "Path Traversal",
            restart_ms: 1500,
            error_msg: "Unauthorized file access: /etc/passwd",
            wasm_trap: "capability not granted: filesystem",
        },
        _ => AttackConfig {
            name: "Unknown Attack",
            restart_ms: 1000,
            error_msg: "Unknown error",
            wasm_trap: "trap",
        },
    }
}

// ============================================================================
// wasm measurement (real webassembly api calls)
// ============================================================================

const MINIMAL_WASM: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, // magic
    0x01, 0x00, 0x00, 0x00, // version
    0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, // type section
    0x03, 0x02, 0x01, 0x00, // function section
    0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, // export section
    0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, // code
];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
    
    // Pyodide globals
    #[wasm_bindgen(js_namespace = window, js_name = pyodideReady)]
    static PYODIDE_READY: bool;
    
    #[wasm_bindgen(js_namespace = window, js_name = pyodideLoadTime)]
    static PYODIDE_LOAD_TIME: f64;
    
    // Run Python code via Pyodide
    #[wasm_bindgen(catch, js_namespace = window)]
    async fn runPython(code: &str) -> Result<JsValue, JsValue>;
}

async fn measure_instantiate_time() -> f64 {
    let array = js_sys::Uint8Array::from(MINIMAL_WASM);
    let compile_promise = js_sys::WebAssembly::compile(&array.buffer());
    let module: js_sys::WebAssembly::Module = wasm_bindgen_futures::JsFuture::from(compile_promise)
        .await
        .unwrap()
        .unchecked_into();
    
    // Run 10 iterations and average for more accurate sub-millisecond timing
    let iterations = 10;
    let start = now();
    
    for _ in 0..iterations {
        let instantiate_promise = js_sys::WebAssembly::instantiate_module(
            &module,
            &js_sys::Object::new()
        );
        let _ = wasm_bindgen_futures::JsFuture::from(instantiate_promise).await;
    }
    
    (now() - start) / iterations as f64
}

// ============================================================================
// demo component
// ============================================================================

#[component]
pub fn Demo() -> impl IntoView {
    // ========================================================================
    // wasm metrics (real measurements)
    // ========================================================================
    let (wasm_instantiate_ms, set_wasm_instantiate_ms) = create_signal(0.0f64);
    let (measurements_done, set_measurements_done) = create_signal(false);
    
    // ========================================================================
    // 2oo3 voting state (three wasm instances)
    // ========================================================================
    let (instance_states, set_instance_states) = create_signal([
        InstanceState::Healthy,
        InstanceState::Healthy,
        InstanceState::Healthy,
    ]);
    let (faulty_instance, set_faulty_instance) = create_signal(Option::<u8>::None);
    
    // ========================================================================
    // python worker state
    // ========================================================================
    let (python_workers, set_python_workers) = create_signal([true, true, true]);
    let (python_active_worker, set_python_active_worker) = create_signal(0u8);
    let (python_restarting, set_python_restarting) = create_signal(false);
    
    // ========================================================================
    // metrics tracking
    // ========================================================================
    let (python_processed, set_python_processed) = create_signal(0u32);
    let (python_crashed, set_python_crashed) = create_signal(0u32);
    let (python_downtime_ms, set_python_downtime_ms) = create_signal(0u64);
    
    let (wasm_processed, set_wasm_processed) = create_signal(0u32);
    let (wasm_rejected, set_wasm_rejected) = create_signal(0u32);
    // WASM always 0 downtime due to 2oo3 voting
    
    // ========================================================================
    // terminal logs
    // ========================================================================
    let (python_logs, set_python_logs) = create_signal(Vec::<LogEntry>::new());
    let (wasm_logs, set_wasm_logs) = create_signal(Vec::<LogEntry>::new());
    
    // ========================================================================
    // control state
    // ========================================================================
    let (is_running, set_is_running) = create_signal(false);
    let (selected_attack, set_selected_attack) = create_signal("bufferOverflow".to_string());
    
    // ========================================================================
    // measure real wasm performance on mount
    // ========================================================================
    create_effect(move |_| {
        if !measurements_done.get() {
            spawn_local(async move {
                let instantiate_time = measure_instantiate_time().await;
                set_wasm_instantiate_ms.set(instantiate_time);
                set_measurements_done.set(true);
            });
        }
    });
    
    // ========================================================================
    // attack handler
    // ========================================================================
    let trigger_attack = move |_| {
        if is_running.get() { return; }
        set_is_running.set(true);
        
        let attack = selected_attack.get();
        let config = get_attack_config(&attack);
        let current_active = python_active_worker.get();
        
        // initialize if first run
        if python_logs.get().is_empty() {
            set_python_logs.set(vec![
                LogEntry { level: "info".into(), message: "$ python gateway.py --workers 3".into() },
                LogEntry { level: "success".into(), message: "[OK] Worker pool: W0 active, W1/W2 standby".into() },
            ]);
            set_python_processed.set(5);
        }
        
        if wasm_logs.get().is_empty() {
            set_wasm_logs.set(vec![
                LogEntry { level: "info".into(), message: "$ wasmtime gateway.wasm --mode 2oo3".into() },
                LogEntry { level: "success".into(), message: "[OK] 2oo3 TMR: I0, I1, I2 initialized".into() },
                LogEntry { level: "info".into(), message: format!("[METRICS] Instantiate: {:.2}ms (real)", wasm_instantiate_ms.get()) },
            ]);
            set_wasm_processed.set(5);
        }
        
        // show incoming attack
        set_python_logs.update(|logs| {
            logs.push(LogEntry { level: "warn".into(), message: format!("[ATTACK] Incoming: {}", config.name) });
        });
        set_wasm_logs.update(|logs| {
            logs.push(LogEntry { level: "warn".into(), message: format!("[ATTACK] Incoming: {}", config.name) });
        });
        
        // after 500ms: attack hits
        let restart_ms = config.restart_ms;
        let error_msg = config.error_msg.to_string();
        let wasm_trap = config.wasm_trap.to_string();
        
        set_timeout(move || {
            // ================================================================
            // python: current worker crashes, next takes over
            // ================================================================
            let next_active = (current_active + 1) % 3;
            
            set_python_logs.update(|logs| {
                logs.push(LogEntry { level: "error".into(), message: format!("[CRASH] {}", error_msg) });
                logs.push(LogEntry { level: "error".into(), message: format!("💥 W{} CRASHED - switching to W{}...", current_active, next_active) });
            });
            
            // mark current worker as dead
            let mut workers = [true, true, true];
            workers[current_active as usize] = false;
            set_python_workers.set(workers);
            set_python_active_worker.set(next_active);
            set_python_restarting.set(true);
            set_python_crashed.update(|n| *n += 1);
            
            // simulate restart with downtime
            let restart_ms_copy = restart_ms;
            set_timeout(move || {
                set_python_workers.set([true, true, true]);
                set_python_restarting.set(false);
                set_python_downtime_ms.update(|d| *d += restart_ms_copy as u64);
                set_python_logs.update(|logs| {
                    logs.push(LogEntry { 
                        level: "success".into(), 
                        message: format!("[OK] W{} respawned ({}ms) - pool restored", current_active, restart_ms_copy)
                    });
                });
                set_is_running.set(false);
            }, std::time::Duration::from_millis(restart_ms as u64));
            
            // ================================================================
            // wasm: 2oo3 voting catches the fault instantly
            // ================================================================
            let faulty_idx = (js_sys::Math::random() * 3.0) as u8;
            set_faulty_instance.set(Some(faulty_idx));
            
            let mut states = instance_states.get();
            states[faulty_idx as usize] = InstanceState::Faulty;
            set_instance_states.set(states);
            
            let healthy: Vec<u8> = (0..3).filter(|&i| i != faulty_idx).collect();
            
            set_wasm_logs.update(|logs| {
                logs.push(LogEntry { level: "warn".into(), message: format!("[TRAP] I{}: {}", faulty_idx, wasm_trap) });
                logs.push(LogEntry { level: "info".into(), message: format!("[VOTE] I{:?} agree, I{} disagrees", healthy, faulty_idx) });
                logs.push(LogEntry { level: "success".into(), message: "[VOTE] 2/3 majority - attack rejected safely".into() });
                logs.push(LogEntry { level: "success".into(), message: "[OK] Zero downtime - 2/3 continues processing".into() });
            });
            
            set_wasm_rejected.update(|n| *n += 1);
            
            // rebuild faulty instance (real async measurement)
            spawn_local(async move {
                let rebuild_time = measure_instantiate_time().await;
                
                let mut states = instance_states.get();
                states[faulty_idx as usize] = InstanceState::Healthy;
                set_instance_states.set(states);
                set_faulty_instance.set(None);
                
                set_wasm_logs.update(|logs| {
                    logs.push(LogEntry { 
                        level: "success".into(), 
                        message: format!("[OK] I{} rebuilt in {:.2}ms (real) - pool healthy", faulty_idx, rebuild_time)
                    });
                });
            });
            
        }, std::time::Duration::from_millis(500));
    };
    
    // ========================================================================
    // run all attacks
    // ========================================================================
    let run_all_attacks = move |_| {
        if is_running.get() { return; }
        
        let attacks = ["bufferOverflow", "dataExfil", "pathTraversal"];
        
        for (i, attack_name) in attacks.iter().enumerate() {
            let attack = attack_name.to_string();
            let delay = (i as u64) * 3000;
            
            set_timeout(move || {
                set_selected_attack.set(attack);
                set_timeout(move || {
                    trigger_attack(());
                }, std::time::Duration::from_millis(100));
            }, std::time::Duration::from_millis(delay));
        }
    };
    
    // ========================================================================
    // reset
    // ========================================================================
    let reset_demo = move |_| {
        set_python_logs.set(Vec::new());
        set_wasm_logs.set(Vec::new());
        set_python_processed.set(0);
        set_python_crashed.set(0);
        set_python_downtime_ms.set(0);
        set_wasm_processed.set(0);
        set_wasm_rejected.set(0);
        set_instance_states.set([InstanceState::Healthy; 3]);
        set_faulty_instance.set(None);
        set_python_workers.set([true, true, true]);
        set_python_active_worker.set(0);
        set_python_restarting.set(false);
        set_is_running.set(false);
    };

    // ========================================================================
    // view
    // ========================================================================
    view! {
        <div class="tab-content demo-tab">
            <h2>"The Demo: Python vs WASM Side-by-Side"</h2>
            
            // Initialization Time section
            <div class="demo-section">
                <h3>"⏱️ Initialization Time"</h3>
                <p class="section-desc">"Compare cold-start performance between runtimes"</p>
                
                // metrics banner
                <div class="metrics-banner">
                    <div class="metric-item" title="Measured using WebAssembly API (10 iterations averaged)">
                        <span class="metric-label">"WASM Instantiate (real)"</span>
                        <span class="metric-value">{move || format!("{:.2}ms", wasm_instantiate_ms.get())}</span>
                    </div>
                    <div class="metric-item" title="Typical Python worker spawn time">
                        <span class="metric-label">"Python Worker Spawn"</span>
                        <span class="metric-value warning">"~1800ms"</span>
                    </div>
                    <div class="metric-item speedup">
                        <span class="metric-label">"Speedup"</span>
                        <span class="metric-value">{move || {
                            let wasm = wasm_instantiate_ms.get();
                            if wasm > 0.0 {
                                format!("{:.0}x faster", 1800.0 / wasm)
                            } else {
                                "∞x faster".to_string()
                            }
                        }}</span>
                    </div>
                </div>
            </div>
            
            // terminals side by side
            <div class="terminals-container">
                // python terminal
                <div class="terminal-panel python-panel">
                    <div class="terminal-header">
                        <span class="terminal-title">"🐍 Python (multiprocessing)"</span>
                        <span class="terminal-status" class:crashed=move || python_restarting.get()>
                            {move || if python_restarting.get() { "⏳ SPAWNING" } else { "🟢 UP" }}
                        </span>
                    </div>
                    <div class="terminal" id="python-terminal">
                        {move || {
                            let entries = python_logs.get();
                            if entries.is_empty() {
                                view! { <p class="terminal-line info">"$ ready"</p> }.into_view()
                            } else {
                                entries.into_iter().map(|e| {
                                    view! { <p class=format!("terminal-line {}", e.level)>{e.message}</p> }
                                }).collect_view()
                            }
                        }}
                    </div>
                    // worker boxes
                    <div class="workers-panel">
                        <span class="workers-label">"Workers:"</span>
                        {move || {
                            let workers = python_workers.get();
                            let active = python_active_worker.get();
                            (0..3).map(|i| {
                                let is_active = i == active as usize && workers[i];
                                let is_dead = !workers[i];
                                view! {
                                    <div class="worker-box"
                                        class:active=is_active
                                        class:dead=is_dead
                                        class:idle=!is_active && !is_dead
                                    >
                                        {format!("W{}", i)}
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
                
                // wasm terminal
                <div class="terminal-panel wasm-panel">
                    <div class="terminal-header">
                        <span class="terminal-title">"🦀 WASM (2oo3 TMR)"</span>
                        <span class="terminal-status">"🟢 UP"</span>
                    </div>
                    <div class="terminal" id="wasm-terminal">
                        {move || {
                            let entries = wasm_logs.get();
                            if entries.is_empty() {
                                view! { <p class="terminal-line info">"$ ready"</p> }.into_view()
                            } else {
                                entries.into_iter().map(|e| {
                                    view! { <p class=format!("terminal-line {}", e.level)>{e.message}</p> }
                                }).collect_view()
                            }
                        }}
                    </div>
                    // instance boxes
                    <div class="instances-panel">
                        <span class="instances-label">"2oo3 TMR:"</span>
                        {move || {
                            let states = instance_states.get();
                            let faulty = faulty_instance.get();
                            (0..3).map(|i| {
                                let is_faulty = faulty == Some(i as u8);
                                view! {
                                    <div class="instance-box"
                                        class:healthy=states[i] == InstanceState::Healthy && !is_faulty
                                        class:faulty=is_faulty
                                    >
                                        {format!("I{}", i)}
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </div>
            
            // stats comparison
            <div class="stats-container">
                <div class="stats-panel python-stats">
                    <h4>"🐍 Python Stats"</h4>
                    <div class="stats-row">
                        <div class="stat-item">
                            <span class="stat-value">{python_processed}</span>
                            <span class="stat-label">"Processed"</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-value error">{python_crashed}</span>
                            <span class="stat-label">"Crashed"</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-value error">{move || format!("{}ms", python_downtime_ms.get())}</span>
                            <span class="stat-label">"Downtime"</span>
                        </div>
                    </div>
                </div>
                
                <div class="stats-panel wasm-stats">
                    <h4>"🦀 WASM Stats"</h4>
                    <div class="stats-row">
                        <div class="stat-item">
                            <span class="stat-value">{wasm_processed}</span>
                            <span class="stat-label">"Processed"</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-value warn">{wasm_rejected}</span>
                            <span class="stat-label">"Voted Out"</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-value success">"0ms"</span>
                            <span class="stat-label">"Downtime"</span>
                        </div>
                    </div>
                </div>
            </div>
            
            // attack controls
            <div class="attack-section">
                <h3>"☠️ Attack Scenarios"</h3>
                <p class="section-desc">"Click an attack to simulate it, or run all sequentially"</p>
                <div class="attack-buttons">
                    <button 
                        class="attack-btn"
                        class:running=move || selected_attack.get() == "bufferOverflow" && is_running.get()
                        disabled=move || is_running.get()
                        title="Memory corruption attack - causes process crash"
                        on:click=move |_| {
                            set_selected_attack.set("bufferOverflow".to_string());
                            trigger_attack(());
                        }
                    >
                        "💥 Buffer Overflow"
                    </button>
                    <button 
                        class="attack-btn"
                        class:running=move || selected_attack.get() == "dataExfil" && is_running.get()
                        disabled=move || is_running.get()
                        title="Attempts unauthorized network connection"
                        on:click=move |_| {
                            set_selected_attack.set("dataExfil".to_string());
                            trigger_attack(());
                        }
                    >
                        "📤 Data Exfil"
                    </button>
                    <button 
                        class="attack-btn"
                        class:running=move || selected_attack.get() == "pathTraversal" && is_running.get()
                        disabled=move || is_running.get()
                        title="Attempts to read sensitive files"
                        on:click=move |_| {
                            set_selected_attack.set("pathTraversal".to_string());
                            trigger_attack(());
                        }
                    >
                        "📁 Path Traversal"
                    </button>
                </div>
                <div class="attack-actions">
                    <button 
                        class="action-btn runall" 
                        title="Run all 3 attacks sequentially"
                        disabled=move || is_running.get() 
                        on:click=move |_| run_all_attacks(())
                    >
                        "🔥 Run All Attacks"
                    </button>
                    <button 
                        class="action-btn reset" 
                        title="Reset all stats and terminals"
                        on:click=move |_| reset_demo(())
                    >
                        "🔄 Reset"
                    </button>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// helper functions
// ============================================================================

fn set_timeout<F: FnOnce() + 'static>(cb: F, dur: std::time::Duration) {
    use wasm_bindgen::closure::Closure;
    let window = web_sys::window().unwrap();
    let closure = Closure::once(cb);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(), dur.as_millis() as i32
    ).unwrap();
    closure.forget();
}
