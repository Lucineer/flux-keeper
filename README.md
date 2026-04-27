# flux-keeper

> Health monitoring, stuck detection, watchdog — the guardian of agent uptime.

## What It Is

Rust library for agent health monitoring in the FLUX ecosystem. Detects stuck agents, manages watchdog timers, and triggers recovery procedures.

## Usage

```toml
[dependencies]
flux-keeper = "0.1"
```

## Fleet Context

Part of the [FLUX agent simulation](https://github.com/Lucineer/flux-agent-sim) ecosystem. See also:
- [flux-telepathy](https://github.com/Lucineer/flux-telepathy) — agent messaging
- [flux-confidence](https://github.com/Lucineer/flux-confidence) — belief propagation

## License

MIT / Apache-2.0
