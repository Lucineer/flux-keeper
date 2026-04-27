# flux-keeper

Health monitoring and watchdog system for agent fleets. Tracks agent vitals (energy, memory, task completion, response time) and triggers alerts, automatic recovery, or apoptosis.

## Core Concept

Agents get sick. They run out of energy, get stuck in loops, lose connectivity, or accumulate errors. flux-keeper is the immune system — watching for problems and acting before they cascade.

```
Agent Vitals → Health Scorer → Alert / Recover / Apoptose
    ↓               ↓
Energy level    Green: healthy
Memory usage    Yellow: degraded
Response time   Red: intervention needed
Task throughput Orange: recovering
```

## Quick Start

```bash
git clone https://github.com/Lucineer/flux-keeper.git
cd flux-keeper
cargo test
```

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
