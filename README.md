# Guardian One Web-Demo

**Industrial Edge Security Demonstration — Python vs WASM Side-by-Side**

[![Status](https://img.shields.io/badge/status-demo_complete-green.svg)]()
[![Tests](https://img.shields.io/badge/tests-28_passing-brightgreen.svg)]()
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![WASI](https://img.shields.io/badge/WASI-0.2-blueviolet.svg)](https://wasi.dev/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-blue.svg)](https://leptos.dev/)
[![Pyodide](https://img.shields.io/badge/Pyodide-0.24-yellow.svg)](https://pyodide.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> A story-driven dashboard demonstrating WASI 0.2 capabilities vs traditional Python/Docker approaches through **real code execution** in the browser.

---

## The Thesis

| Challenge | Industry Today | WASI 0.2 Approach |
|:----------|:---------------|:------------------|
| **Isolation** | Docker namespaces (shared kernel) | WASM sandbox (boundary separation) |
| **Fault Recovery** | Process restart (~1.5s cold-start) | **Zero-Downtime** (2oo3 TMR masks faults) |
| **Failure Mode** | Catastrophic Process Crash (Fatal) | **Byzantine Fault** (Contained & Voted Out) |
| **Security Model** | Allow-then-block (iptables) | Deny-by-default (capability-based) |

---

## Live Demo Features

- **Real Pyodide Execution** — Python attacks run via actual Pyodide runtime
- **Real WASM Measurements** — Instantiation times measured with WebAssembly API
- **2oo3 TMR Voting** — Demonstrates Byzantine fault tolerance
- **Raft-like Leader Election** — Sub-ms failover vs ~1.5s Python respawn
- **WIT Contract Modal** — View the actual capability boundary definition

> **💡 Key Technical Insight:**
>
> WASM's strict isolation turns **catastrophic process crashes** into **manageable Byzantine faults**.
>
> When a WASM module panics or violates security capabilities, it traps. The Host catches this trap, marks that single instance as "Faulty," and the **2oo3 TMR Voting** logic instantly ignores it. The system continues with **zero downtime**, and the faulty instance is hot-swapped in sub-milliseconds.


---

## Architecture

<details>
<summary><strong>📐 IEC 62443 Zone Diagram</strong></summary>

![IEC 62443 Zones & Conduits](diagrams/architecture-zones.jpg)

</details>

<details>
<summary><strong>🔧 Hardware Layer Diagram</strong></summary>

![Hardware Layers](diagrams/hardware-layers.jpg)

</details>

---

## Dashboard Tabs

| Tab | What You'll See |
|-----|-----------------|
| **The Problem** | Attack surface comparison — why traditional ICS security fails |
| **The Hardware** | Architecture diagram — Purdue Model zones we're simulating |
| **The Demo** | Live attack simulations — Python (Pyodide) vs WASM side-by-side |
| **The Proof** | Real metrics + foundation project links |

<details>
<summary><strong>📸 Screenshots</strong></summary>

### The Problem
![Problem Tab](diagrams/screenshot-problem.png)

### The Hardware
![Hardware Tab](diagrams/screenshot-hardware.png)

### The Demo
![Demo Tab](diagrams/screenshot-demo.png)

### The Proof
![Proof Tab](diagrams/screenshot-proof.png)

</details>

---

## Quick Start

```bash
# Install trunk
cargo install trunk

# Run locally
cd dashboard && trunk serve --open
# Opens http://localhost:8080
```

---

## Project Structure

```
guardian-one-web-demo/
├── dashboard/               # Leptos frontend
│   └── src/
│       └── tabs/            # Story-driven tab components
│           ├── problem/     # Tab 1: Problem explanation
│           ├── hardware/    # Tab 2: Architecture diagram
│           ├── demo/        # Tab 3: Interactive attack demo
│           │   ├── types.rs
│           │   ├── attacks.rs
│           │   ├── wasm.rs
│           │   └── component.rs
│           └── proof/       # Tab 4: Metrics & foundation projects
├── wasm-modules/            # Rust WASM components
│   ├── sensor-driver/       # BME280 telemetry logic
│   └── modbus-parser/       # Industrial protocol parser
├── python-equivalents/      # Python code for Pyodide comparison
│   ├── sensor_driver.py
│   ├── modbus_parser.py
│   └── attacks/             # Attack scenario scripts
├── wit/                     # WASI interface definitions
│   └── attacks.wit          # Capability boundary contract
├── diagrams/                # Architecture diagrams
└── vercel.json              # Deployment configuration
```

---

## Metrics Accuracy

| Source | Measurement Method |
|--------|-------------------|
| **WASM Cold-Start** | Live measurement with `WebAssembly.instantiate()` (10 iterations avg) |
| **Python Cold-Start** | Real Pyodide reload measured fresh each simulation |
| **Attack Exceptions** | Real Python execution via Pyodide — actual exceptions |
| **Binary sizes** | Static values (actual `.wasm` file sizes) |

> All timing values are measured live in your browser. Python restart times use real Pyodide cold-start ± 200ms jitter for realistic variance.

---

## Testing

28 tests validate the demo's core guarantees:

| Module | Tests | Focus |
|--------|-------|-------|
| Attack Logic | 8 | WIT config accuracy, edge cases |
| Voting Logic | 8 | 2oo3 TMR, leader election |
| Measurement | 7 | Speedup math, bounds |
| State Invariants | 5 | System-wide guarantees |

```bash
cd dashboard && cargo test --lib
```

![Test Results](diagrams/tests.png)

See [docs/TESTING.md](docs/TESTING.md) for full test documentation.

---

## Git Workflow

```
main ──────────────────────────────► Production (Vercel)
  │                                      ▲
  ▼                                      │
develop ───────────────────────────► Preview
  │         ▲         ▲
  ▼         │         │
feature/*   feature/* feature/*
```

---

## Related Projects (The Guardian One Foundation)

| Project | Focus | Demo |
|---------|-------|------|
| [ICS Guardian](https://github.com/gammahazard/vanguard-ics-guardian) | **Containment** — Capability sandboxing | [Live](https://vanguard-ics-guardian.vercel.app) |
| [Protocol Gateway](https://github.com/gammahazard/protocol-gateway-sandbox) | **Availability** — 2oo3 crash recovery | [Live](https://protocol-gateway-sandbox.vercel.app) |
| [Raft Consensus](https://github.com/gammahazard/Raft-Consensus) | **Consistency** — Distributed consensus | [Live](https://raft-consensus.vercel.app) |

---

## License

MIT © 2026

