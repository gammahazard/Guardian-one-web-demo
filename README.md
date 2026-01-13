# Reliability Triad Console

**Industrial Edge Security Demonstration — Python vs WASM Side-by-Side**

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![WASI](https://img.shields.io/badge/WASI-0.2-blueviolet.svg)](https://wasi.dev/)
[![Leptos](https://img.shields.io/badge/Leptos-0.6-blue.svg)](https://leptos.dev/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> A story-driven dashboard demonstrating WASI 0.2 capabilities vs traditional Python/Docker approaches through **real code execution** in the browser.

---

## Tabs

| Tab | Purpose |
|-----|---------|
| **The Problem** | Why traditional ICS security fails |
| **The Hardware** | Architecture we're simulating |
| **The Demo** | Live Python vs WASM comparison |
| **The Proof** | Metrics, videos, links |

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
├── wasm-modules/        # Rust WASM components
├── python-equivalents/  # Python code for comparison
├── wit/                 # WASI interface definitions
└── docs/               # Documentation
```

---

## Related Projects

| Project | Focus | Demo |
|---------|-------|------|
| [ICS Guardian](https://github.com/gammahazard/vanguard-ics-guardian) | Capability sandboxing | [Live](https://vanguard-ics-guardian.vercel.app) |
| [Protocol Gateway](https://github.com/gammahazard/protocol-gateway-sandbox) | Modbus/MQTT translation | [Live](https://protocol-gateway-sandbox.vercel.app) |
| [Raft Consensus](https://github.com/gammahazard/Raft-Consensus) | Distributed consensus | [Live](https://raft-consensus.vercel.app) |
| [Guardian-One](https://github.com/gammahazard/guardian-one) | Hardware implementation | *Requires hardware* |

---

## License

MIT © 2026
