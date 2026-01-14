# Reliability Triad Console

**Industrial Edge Security Demonstration — Python vs WASM Side-by-Side**

[![Status](https://img.shields.io/badge/status-in_development-yellow.svg)]()
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![WASI](https://img.shields.io/badge/WASI-0.2-blueviolet.svg)](https://wasi.dev/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-blue.svg)](https://leptos.dev/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> A story-driven dashboard demonstrating WASI 0.2 capabilities vs traditional Python/Docker approaches through **real code execution** in the browser.

---

## The Thesis

| Challenge | Industry Today | WASI 0.2 Approach |
|:----------|:---------------|:------------------|
| **Isolation** | Docker namespaces (shared kernel) | WASM sandbox (boundary separation) |
| **Fault Recovery** | Process restart (2-3 seconds) | Instance re-instantiation (~0.2ms) |
| **Binary Size** | 50-500 MB container images | 15-70 KB component binaries |
| **Security Model** | Allow-then-block (iptables) | Deny-by-default (capability-based) |

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

The console uses a **narrative-driven** approach to walk viewers through the systems engineering lifecycle:

| Tab | Purpose |
|-----|---------|
| **The Problem** | Why traditional ICS security fails — attack surface comparison |
| **The Hardware** | Architecture we're simulating — Purdue Model zones |
| **The Demo** | Live Python (Pyodide) vs WASM comparison with attack scenarios |
| **The Proof** | Metrics, hardware videos, links to foundation projects |

---

## Quick Start

```bash
# Install trunk
cargo install trunk

# Run locally
cd dashboard && trunk serve
# Opens http://localhost:8080
```

---

## Project Structure

```
reliability-triad/
├── dashboard/           # Leptos frontend
│   └── src/
│       ├── tabs/        # Story-driven tab components
│       └── components/  # Reusable UI widgets
├── wasm-modules/        # Rust WASM components
│   ├── sensor-driver/   # BME280 telemetry logic
│   └── modbus-parser/   # Industrial protocol parser
├── python-equivalents/  # Python code for Pyodide comparison
├── wit/                 # WASI interface definitions
├── diagrams/            # Architecture diagrams
└── docs/                # Deep-dive documentation
```

---

## Metrics Accuracy

| Source | Measurement Method |
|--------|-------------------|
| **WASM** | Live measurement with `performance.now()` |
| **Python** | Real Pyodide execution in browser |
| **Binary sizes** | Actual `.wasm` file sizes |

> Python "industry estimates" are clearly labeled; WASM values are measured live.

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

## Related Projects (The Reliability Triad)

| Project | Focus | Demo |
|---------|-------|------|
| [ICS Guardian](https://github.com/gammahazard/vanguard-ics-guardian) | **Containment** — Capability sandboxing | [Live](https://vanguard-ics-guardian.vercel.app) |
| [Protocol Gateway](https://github.com/gammahazard/protocol-gateway-sandbox) | **Availability** — 2oo3 crash recovery | [Live](https://protocol-gateway-sandbox.vercel.app) |
| [Raft Consensus](https://github.com/gammahazard/Raft-Consensus) | **Consistency** — Distributed consensus | [Live](https://raft-consensus.vercel.app) |
| [Guardian-One](https://github.com/gammahazard/guardian-one) | Hardware implementation | *Requires hardware* |

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design and component interactions |
| [SECURITY.md](docs/SECURITY.md) | Capability model and threat analysis |
| [BRANCHING.md](docs/BRANCHING.md) | Git workflow and merge policies |

---

## License

MIT © 2026
