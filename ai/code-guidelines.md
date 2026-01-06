# Rust Code Guidelines

## Philosophy
Code must be self-documenting. Type signatures, function names, and module organization convey intent. Avoid comments except for:
- Public API documentation (`///`)
- Non-obvious algorithmic decisions (e.g., "Use median instead of mean to avoid outlier skew")
- Safety invariants (`// SAFETY: ...`)

## Naming Conventions

### Casing Standards
- **Types (structs, enums, traits):** `PascalCase`
  ```rust
  struct VwapResult { ... }
  enum ExchangeClientError { ... }
  trait PricePublisher { ... }
  ```

- **Functions, methods, variables:** `snake_case`
  ```rust
  fn calculate_vwap(&self, trades: &[Trade]) -> VwapResult { ... }
  let trade_buffer: Vec<Trade> = Vec::new();
  ```

- **Constants, statics:** `SCREAMING_SNAKE_CASE`
  ```rust
  const BINANCE_WEBSOCKETS_URL: &str = "wss://...";
  const DEFAULT_WINDOW_SECONDS: u64 = 60;
  ```

- **Lifetimes, type parameters:** Single lowercase letter or descriptive name
  ```rust
  fn process<'a, T: Send>(data: &'a T) -> &'a str { ... }
  ```

### Semantic Naming

**Prefer verb phrases for functions:**
```rust
// Good
fn calculate_vwap(...) -> VwapResult
fn write_to_database(...) -> Result<()>
fn subscribe_to_channel(...) -> Receiver<Trade>

// Avoid
fn vwap(...) -> VwapResult
fn database(...) -> Result<()>
fn channel(...) -> Receiver<Trade>
```

**Prefer noun phrases for types:**
```rust
// Good
struct VwapAggregator { ... }
struct TradeWriter { ... }

// Avoid
struct AggregateVwap { ... }
struct WriteTrade { ... }
```

**Use domain language from specifications:**
```rust
// Good (matches financial domain)
struct VwapResult { vwap: Decimal, ... }
fn calculate_spread_bps(...) -> Decimal

// Avoid (generic naming)
struct PriceAverage { avg: Decimal, ... }
fn compute_difference(...) -> Decimal
```

**C# Comparison:**
- Rust `snake_case` methods ≈ C# `PascalCase` methods (cultural difference)
- Rust field visibility explicit (`pub`) ≈ C# properties with getters

## Type System Usage

### Prefer Newtype Pattern for Domain Concepts
```rust
// Good: Type safety prevents mixing exchanges
struct BinanceTradeId(u64);
struct KrakenTradeId(u64);

// Avoid: Primitive obsession
fn process_trade(id: u64) { ... } // Which exchange?
```

**C# Comparison:** Similar to `readonly record struct TradeId(long Value)`

### Use Enums for State Machines
```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { since: Instant },
    Reconnecting { attempt: u32 },
}

impl ConnectionState {
    fn is_connected(&self) -> bool {
        matches!(self, ConnectionState::Connected { .. })
    }
}
```

**C# Comparison:** Rust enums ≈ C# discriminated unions (F# style), more powerful than C# enums

### Leverage Type State Pattern
```rust
struct ConnectionBuilder<State> {
    url: String,
    state: PhantomData<State>,
}

struct Unvalidated;
struct Validated;

impl ConnectionBuilder<Unvalidated> {
    fn validate(self) -> Result<ConnectionBuilder<Validated>> { ... }
}

impl ConnectionBuilder<Validated> {
    fn connect(self) -> Connection { ... } // Only validated can connect
}
```

**C# Comparison:** No direct equivalent; enforces state transitions at compile time

## Error Handling

### Use `thiserror` for Library Errors
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VwapCalculationError {
    #[error("Insufficient data: need at least {min} trades, got {actual}")]
    InsufficientData { min: usize, actual: usize },

    #[error("Invalid window duration: {0}")]
    InvalidWindow(#[from] std::time::SystemTimeError),
}
```

### Use `anyhow` for Application Code
```rust
use anyhow::{Context, Result};

async fn run_pipeline() -> Result<()> {
    let pool = create_db_pool()
        .await
        .context("Failed to initialize database pool")?;

    Ok(())
}
```

**C# Comparison:**
- `thiserror::Error` ≈ Custom exception classes
- `anyhow::Result` ≈ Generic `Exception` handling with context

### Never Use `unwrap()` in Production Paths
```rust
// Good: Startup/initialization only
let url = Url::parse(WEBSOCKET_URL).expect("Hardcoded URL must be valid");

// Good: Propagate errors
let pool = PgPoolOptions::new()
    .connect(&db_url)
    .await?; // Propagates error to caller

// Avoid: Runtime unwrap
let trade = self.cache.get(&symbol).unwrap(); // PANIC if missing!

// Use instead:
let trade = self.cache.get(&symbol).ok_or(CacheError::SymbolNotFound)?;
```

### Handle Channel Errors Explicitly
```rust
match receiver.recv().await {
    Ok(data) => { /* process */ }
    Err(RecvError::Lagged(count)) => {
        tracing::warn!(skipped = count, "Consumer lagging");
        // Decision: continue or fail fast
    }
    Err(RecvError::Closed) => {
        tracing::info!("Channel closed, shutting down");
        break;
    }
}
```

## Async Patterns

### Spawn Long-Running Tasks with Named Handles (Future)
```rust
// Good: Named handle for observability
let binance_handle = tokio::spawn(async move {
    binance_client.run().await
}).name("binance-websocket");

// Current: Acceptable for now
tokio::spawn(binance_client.run());
```

### Use `tokio::select!` for Concurrent Operations
```rust
loop {
    tokio::select! {
        result = receiver.recv() => {
            match result {
                Ok(data) => process(data),
                Err(e) => break,
            }
        }
        _ = interval.tick() => {
            flush_buffer().await;
        }
        _ = shutdown_signal.recv() => {
            cleanup().await;
            break;
        }
    }
}
```

**C# Comparison:** `tokio::select!` ≈ `Task.WhenAny` but more ergonomic

### Avoid Blocking in Async Context
```rust
// Wrong: Blocks the async runtime
async fn bad_example() {
    std::thread::sleep(Duration::from_secs(1)); // NO!
}

// Correct: Non-blocking delay
async fn good_example() {
    tokio::time::sleep(Duration::from_secs(1)).await; // YES
}
```

**C# Comparison:** Same as `Thread.Sleep` vs `Task.Delay` in async methods

## Ownership & Borrowing

### Prefer Borrowing Over Cloning
```rust
// Good: Borrow when read-only access suffices
fn calculate_vwap(&self, trades: &[Trade]) -> VwapResult { ... }

// Avoid: Unnecessary clone
fn calculate_vwap(&self, trades: Vec<Trade>) -> VwapResult { ... }
```

### Use `Arc` for Shared Ownership Across Tasks
```rust
let state = Arc::new(VwapState::default());
let state_clone = Arc::clone(&state);

tokio::spawn(async move {
    state_clone.update(...).await;
});
```

**C# Comparison:** `Arc` ≈ Shared reference, but explicit (no GC)

### Use `Arc<RwLock<T>>` for Shared Mutable State
```rust
struct PriceCache {
    inner: Arc<RwLock<HashMap<Exchange, Price>>>,
}

impl PriceCache {
    async fn get(&self, exchange: &Exchange) -> Option<Price> {
        let guard = self.inner.read().await;
        guard.get(exchange).cloned()
    }

    async fn update(&self, exchange: Exchange, price: Price) {
        let mut guard = self.inner.write().await;
        guard.insert(exchange, price);
    }
}
```

**C# Comparison:**
- `RwLock` ≈ `ReaderWriterLockSlim`
- `Arc<RwLock<T>>` ≈ `ConcurrentDictionary<K,V>` (conceptually, different implementation)

### Consume `self` for Builder/Runner Patterns
```rust
impl VWapAggregator {
    pub async fn run(mut self) {
        // Takes ownership, preventing reuse
        loop { ... }
    }
}

// Usage:
let aggregator = VWapAggregator::new(...);
tokio::spawn(aggregator.run()); // Moved, can't use aggregator again
```

**C# Comparison:** No direct equivalent; enforces single-use at compile time

## Module Organization

### One Concern Per Module
```
application/src/pipeline/
├── filters/
│   ├── mod.rs          // Re-exports
│   └── vwap_aggregator.rs
├── sinks/
│   ├── mod.rs
│   ├── trade_writer.rs
│   └── vwap_writer.rs
└── mod.rs
```

### Use `mod.rs` for Re-exports Only
```rust
// application/src/pipeline/filters/mod.rs
pub mod vwap_aggregator;

pub use vwap_aggregator::VWapAggregator;
```

### Prefer Flat Module Hierarchies
```rust
// Good: Flat, discoverable
use application::pipeline::filters::VWapAggregator;

// Avoid: Deep nesting
use application::pipeline::filters::aggregators::vwap::VWapAggregator;
```

**C# Comparison:**
- Rust modules ≈ C# namespaces
- `mod.rs` ≈ No direct equivalent (namespace defined per file in C#)

## Trait Usage

### Define Traits in `core`, Implement in Outer Layers
```rust
// core/src/exchange_client.rs
#[async_trait]
pub trait ExchangeClient: Send {
    async fn run(self) -> Result<(), ExchangeClientError>;
}

// infrastructure/src/binance/client.rs
#[async_trait]
impl ExchangeClient for BinanceClient {
    async fn run(self) -> Result<(), ExchangeClientError> {
        // Implementation
    }
}
```

### Use `async_trait` for Async Trait Methods
```rust
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn save(&self, trade: &Trade) -> Result<()>;
}
```

**Reason:** Rust does not natively support async in traits (as of 2024). `async_trait` macro provides workaround.

### Prefer Trait Bounds Over `dyn Trait` for Performance
```rust
// Good: Monomorphization, zero-cost abstraction
fn process<T: PricePublisher>(publisher: T) { ... }

// Acceptable: Dynamic dispatch when flexibility needed
fn process(publisher: &dyn PricePublisher) { ... }
```

**C# Comparison:**
- `T: Trait` ≈ Generic constraint `where T : IInterface`
- `&dyn Trait` ≈ Interface reference `IInterface` (always virtual in C#)

## Testing Patterns

### Unit Test Module Convention
```rust
// In same file as implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vwap_calculation() {
        let trades = vec![
            Trade { price: dec!(100), quantity: dec!(1), ... },
            Trade { price: dec!(101), quantity: dec!(2), ... },
        ];

        let result = calculate_vwap(&trades);
        assert_eq!(result.vwap, dec!(100.666666...));
    }
}
```

### Integration Tests in `tests/` Directory
```rust
// infrastructure/tests/binance_connector_test.rs
use infrastructure::binance::BinanceClient;

#[tokio::test]
async fn test_binance_connection() {
    let (tx, _rx) = broadcast::channel(10);
    let client = BinanceClient::new(tx);

    // Test connection logic
}
```

### Use `test-log` for Tracing in Tests
```rust
use test_log::test;

#[test(tokio::test)]
async fn test_with_logging() {
    tracing::info!("This will be visible if test fails");
    // Test logic
}
```

**C# Comparison:**
- `#[cfg(test)]` ≈ Test project separation
- `#[tokio::test]` ≈ `async Task` test methods in xUnit/NUnit

## Serialization

### Use `serde` Derive Macros
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: u64,
    pub symbol: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    pub quantity: Decimal,
    pub timestamp: u64,
}
```

### Handle Field Renaming with Attributes
```rust
#[derive(Deserialize)]
struct BinanceTrade {
    #[serde(rename = "t")]
    trade_id: u64,
    #[serde(rename = "s")]
    symbol: String,
}
```

**C# Comparison:**
- `#[derive(Serialize, Deserialize)]` ≈ `[JsonSerializable]` source generator
- `#[serde(rename = "...")]` ≈ `[JsonPropertyName("...")]`

## Logging & Observability

### Use Structured Logging with Fields
```rust
// Good: Structured fields for querying
tracing::info!(
    exchange = "binance",
    symbol = %trade.symbol,
    price = %trade.price,
    "Trade received"
);

// Avoid: String interpolation
tracing::info!("Trade received: {} at {}", trade.symbol, trade.price);
```

**LogQL Query:**
```logql
{service="rust-app-local"} | json | exchange="binance"
```

### Use Appropriate Log Levels
```rust
tracing::error!(error = ?e, "Fatal: Database connection lost");
tracing::warn!(count = lagged, "Consumer lagging");
tracing::info!("WebSocket connection established");
tracing::debug!(msg = ?message, "Received message");
tracing::trace!("Entering function");
```

### Log Errors with `?` Debug Formatting
```rust
match operation().await {
    Ok(result) => { /* ... */ }
    Err(e) => {
        tracing::error!(error = ?e, "Operation failed"); // Use ?e for Debug
    }
}
```

**C# Comparison:**
- Structured logging ≈ `ILogger.Log(LogLevel, new { Exchange = "binance", ... })`
- `?e` ≈ `{@Exception}` in Serilog

## Performance Considerations

### Prefer `&str` Over `String` for Function Parameters
```rust
// Good: Avoids allocation
fn parse_symbol(symbol: &str) -> CoinPair { ... }

// Avoid: Forces caller to own
fn parse_symbol(symbol: String) -> CoinPair { ... }
```

### Use `Vec::with_capacity` When Size Known
```rust
// Good: Single allocation
let mut buffer = Vec::with_capacity(1000);
for i in 0..1000 {
    buffer.push(i);
}

// Avoid: Multiple reallocations
let mut buffer = Vec::new();
for i in 0..1000 {
    buffer.push(i);
}
```

### Avoid Cloning in Hot Paths
```rust
// Hot path: Millions of trades per second
for trade in trades.iter() { // Borrow
    process(&trade); // Pass by reference
}

// Avoid:
for trade in trades.iter() {
    let owned = trade.clone(); // Unnecessary allocation
    process(&owned);
}
```

**C# Comparison:**
- `&str` vs `String` ≈ `ReadOnlySpan<char>` vs `string`
- `Vec::with_capacity` ≈ `new List<T>(capacity)`

## Cargo & Dependencies

### Prefer Workspace Dependencies
```toml
# Cargo.toml (workspace root)
[workspace.dependencies]
tokio = { version = "1.48", features = ["full"] }
serde = { version = "1", features = ["derive"] }

# core/Cargo.toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
```

### Minimize Feature Flags
```toml
# Good: Only enable needed features
tokio = { version = "1", features = ["macros", "rt-multi-thread", "sync"] }

# Avoid: Bloat
tokio = { version = "1", features = ["full"] }
```

### Lock Dependency Versions
- Commit `Cargo.lock` for binary crates (applications)
- Ignore `Cargo.lock` for library crates
- Use `cargo update` explicitly to update dependencies

**C# Comparison:**
- Workspace dependencies ≈ `Directory.Build.props` with `<PackageVersion>`
- `Cargo.lock` ≈ `packages.lock.json` in NuGet

## Code Review Checklist

Before submitting code, verify:

- [ ] No `unwrap()` or `expect()` in non-startup paths
- [ ] Errors logged with structured fields
- [ ] `clippy` warnings addressed (`cargo clippy`)
- [ ] Code formatted with `rustfmt` (`cargo fmt`)
- [ ] Public APIs have `///` documentation
- [ ] Tests added for new functionality
- [ ] No `println!` (use `tracing` instead)
- [ ] Channel receivers handle `Lagged` and `Closed` errors
- [ ] Async functions use `tokio::time::sleep`, not `std::thread::sleep`
- [ ] Dependencies added to workspace `Cargo.toml`

## Anti-Patterns

**Avoid:**
- **God structs:** Structs with too many responsibilities. Split into focused types.
- **Stringly-typed APIs:** Using `String` for domain concepts. Use enums or newtypes.
- **Ignoring errors:** `let _ = operation();` without logging. Always handle or propagate.
- **Mutex everywhere:** Overuse of `Mutex` when `RwLock` or message passing suffices.
- **Premature optimization:** Profile before optimizing. Clarity first, speed second.

**C# Comparison:**
- God structs ≈ Violating Single Responsibility Principle
- Stringly-typed ≈ Primitive obsession code smell

## Useful Clippy Lints

Enable strict lints in `Cargo.toml`:
```toml
[lints.clippy]
all = "warn"
pedantic = "warn"
unwrap_used = "warn"
expect_used = "warn"
```

Common lints to address:
- `clippy::unwrap_used` - Forces explicit error handling
- `clippy::large_enum_variant` - Suggests `Box<T>` for large variants
- `clippy::too_many_arguments` - Suggests struct for parameter grouping
- `clippy::module_name_repetitions` - Avoids `vwap::VwapCalculator` (just `vwap::Calculator`)

## Additional Resources

**For C# Developers Learning Rust:**
- Ownership ≈ Deterministic finalization (but compile-time enforced)
- Traits ≈ Interfaces + Extension methods + Typeclasses
- Enums ≈ Discriminated unions (F# style)
- `Result<T, E>` ≈ Railway-oriented programming
- Macros ≈ Roslyn source generators (but more powerful)

**Style Guides:**
- Official Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- The Rust Book: https://doc.rust-lang.org/book/
