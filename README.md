# flux-keeper 🏥

**Health monitoring, stuck detection, and watchdog for agent fleets.** Tracks vitals (energy, memory, task completion, response time) and triggers alerts, recovery, or apoptosis on failure cascades.

```rust
use flux_keeper::{Keeper, CheckStatus};

let mut kp = Keeper::new();
kp.add_check(1, "engine-temp", /* interval: */ 30, /* timeout: */ 60, /* max_failures: */ 3);

kp.report_ok(1, 1000);  // heartbeat received
kp.report_failure(1, 1010);  // first failure → Warning

let alerts = kp.tick(1100);  // periodic eval
println!("Alerts: {:?}", alerts);
```

## Why Keeper?

Agents get sick. They OOM, deadlock, lose connectivity, or spin in loops. Keeper catches it before it cascades:

- **Configurable per-check** — interval, timeout, failure threshold
- **Three status states** — Ok → Warning → Critical (automatic escalation)
- **Timeout detection** — if a check hasn't reported in `timeout` seconds, it fires
- **Consecutive failure limit** — transient glitch ≠ death (3 failures = alert)
- **No deps** — pure Rust, no runtime, fits in embedded

## API

```rust
let mut kp = Keeper::new();

// Register a health check
kp.add_check(1, "engine-temp", 30, 60, 3);

// Report status
kp.report_ok(1, now);
kp.report_failure(1, now);  // increments consecutive_failures

// Periodic evaluation
let new_alerts: Vec<String> = kp.tick(now);

// Read alerts
for alert in kp.alerts() {
    eprintln!("KEEPER: {}", alert);
}

// Status query
println!("Running: {}", kp.running());
let status = kp.status(1);  // Option<&CheckStatus>
```

### Alert Escalation

| State | Cue | Action |
|-------|-----|--------|
| Ok | Heartbeat received | Nothing |
| Warning | 1-2 failures | Log alert, continue |
| Critical | ≥3 consecutive failures | Apoptosis trigger |

## Cargo.toml

```toml
[dependencies]
flux-keeper = { git = "https://github.com/Lucineer/flux-keeper" }
```

## Fleet Context

Part of the Lucineer/Cocapn fleet. Sister crate to [flux-telepathy](https://github.com/Lucineer/flux-telepathy) (alert routing) and [flux-confidence](https://github.com/Lucineer/flux-confidence) (calibrated decision-making under uncertainty).
