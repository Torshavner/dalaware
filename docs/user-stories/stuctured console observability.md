# User Story: Structured Console Observability

## User Story

As a developer, I need human-readable, structured console logs so that I can monitor application health and debug data pipeline events in real-time without external dependencies.

## Problem Statement

The current logging configuration is split between a compact console view and a machine-readable JSON/UDP layer. For local development, the console logs need to be more descriptive (including timestamps and levels) to effectively track the multi-threaded execution of Binance and Kraken clients.

## Acceptance Criteria

### Core Logging

* [ ] Given the application starts, when `init_tracing` is called, then logs are emitted to `stdout`.
* [ ] Given a background task (e.g., `BinanceClient`), when an event occurs, then the log output includes the task's context (fields like `symbol` or `exchange`).

### Configuration

* [ ] Log levels are controllable via the `RUST_LOG` environment variable (defaulting to `info`).
* [ ] Console output uses a "pretty" or "full" format rather than "compact" for better readability during deep-dive debugging.

### Testing (Test-First)

* [ ] Write a test in `logging.rs` to verify the `EnvFilter` correctly parses level strings.
* [ ] Verify that `init_tracing(false)` does not attempt to bind the UDP socket (preventing startup panics if port 9000 is blocked).

## Technical Context

We are using `tracing-subscriber`. The `Registry` pattern allows us to layer different outputs. We will modify `logging.rs` to use `fmt::layer().with_ansi(true)` for colorized output, which helps distinguish between `INFO` and `ERROR` at a glance.

**C# Comparison:**

* `tracing` + `tracing-subscriber` ≈ **Serilog** with **Sinks**.
* `EnvFilter` ≈ **MinimumLevel** configuration in `appsettings.json`.
* `tracing::info!(field = value)` ≈ **Structured Logging** (Message Templates) in .NET.

## Dependencies

* None

## Estimated Complexity

Low

---

### Implementation Plan

1. **Read** [logging.rs:43](https://www.google.com/search?q=logging.rs%23L43) to identify where the `console_layer` is defined.
2. **Refactor** `init_tracing` to allow format customization (e.g., swapping `.compact()` for `.pretty()` based on an environment flag).
3. **Verify** the `main.rs` [main.rs:65](https://www.google.com/search?q=main.rs%23L65) migration logic logs correctly to the console.

Would you like me to generate the refactored `logging.rs` code to satisfy these criteria?