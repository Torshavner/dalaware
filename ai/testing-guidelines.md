# Testing Guidelines

Testing strategy aligned with Clean Architecture and learning objectives.

## Test-First Development (MANDATORY)

**Rule:** When implementing new features, ALWAYS write tests first.

**Process:**
1. **Read user story acceptance criteria** → These become your test cases
2. **Write failing tests** → Translate Given-When-Then into test functions
3. **Implement minimum code** → Make tests pass
4. **Refactor** → Improve code while keeping tests green
5. **Verify** → Run `cargo test` and `cargo check`

**Example workflow:**
```bash
# Step 1: Create test file (user story: lead-lag analytics)
# application/tests/lead_lag_analyzer_test.rs

#[test]
#[allow(non_snake_case)]
fn given__binance_price_updates_first__when__analyze__then__binance_is_leader() {
    // given (will not compile yet)
    let analyzer = LeadLagAnalyzer::new(...);

    // when
    let result = analyzer.analyze(...);

    // then
    assert_eq!(result.leader, Exchange::Binance);
}

# Step 2: Run test (will fail to compile)
cargo test

# Step 3: Implement LeadLagAnalyzer struct and methods to make it compile
# Step 4: Implement logic to make test pass
# Step 5: Refactor while keeping test green
```

**Why test-first for this project:**
- **Learning:** Forces you to think about API design before implementation
- **Documentation:** Tests demonstrate intended usage patterns
- **Confidence:** Prevents regressions during refactoring (common when learning Rust)
- **Architecture:** Ensures loosely coupled, testable designs
- **Feedback:** Compiler errors guide implementation (Rust's strong suit)

**When to skip test-first:**
- Exploratory coding (spike solutions that will be thrown away)
- Trivial changes (typo fixes, formatting)
- Infrastructure setup (Docker configs, migrations)

**C# Comparison:**
- Same TDD/BDD principles as C# (Red-Green-Refactor)
- User story Given-When-Then maps directly to test names
- Rust compiler provides stronger feedback loop than C# during TDD

## Testing Philosophy

**Goals:**
1. **Confidence:** Tests verify correctness, not just coverage percentage
2. **Documentation:** Tests demonstrate intended usage
3. **Regression Prevention:** Catch bugs before production
4. **Learning:** Tests teach Rust idioms and patterns

**Principles:**
- **Test-first:** Write tests before implementation (see above)
- Test behavior, not implementation
- Unit test pure logic, integration test boundaries
- Prefer real dependencies over mocks (when feasible)
- Write tests that would fail if behavior changes

## Test Pyramid

```
        ╱───────────────╲
       ╱   E2E Tests     ╲     ← Few (expensive, slow)
      ╱─────────────────────╲
     ╱  Integration Tests   ╲  ← Some (moderate cost)
    ╱───────────────────────────╲
   ╱      Unit Tests           ╲ ← Many (cheap, fast)
  ╱─────────────────────────────────╲
```

**For this project:**
- **Unit Tests:** 60% - Pure functions, domain logic
- **Integration Tests:** 35% - Database, WebSockets, channels
- **E2E Tests:** 5% - Full pipeline (future, low priority for learning)

## Test Organization

### File Structure

**Unit tests (same file):**
```rust
// application/src/pipeline/filters/vwap_aggregator.rs

impl VWapAggregator {
    fn calculate_vwap(&self, trades: &[Trade]) -> VwapResult {
        // Implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vwap_calculation_with_multiple_trades() {
        // Test implementation
    }
}
```

**Integration tests (separate directory):**
```
infrastructure/
  tests/
    binance_connector_test.rs
    kraken_connector_test.rs
```

**Component tests (crate tests directory):**
```
application/
  tests/
    vwap_aggregator_test.rs
    lead_lag_comparator_test.rs
```

### Naming Conventions

**MANDATORY: Given-When-Then Pattern**

All tests MUST follow the Given-When-Then naming convention using double underscores for clarity:

```rust
#[test]
#[allow(non_snake_case)]
fn given__{preconditions}__when__{action}__then__{expected_outcome}()

// Template breakdown:
// given__ - Initial state/setup (can have multiple conditions with _and_)
// __when__ - The action being tested
// __then__ - Expected outcome/assertion

// Examples:
#[allow(non_snake_case)]
fn given__empty_trades__when__calculate_vwap__then__returns_zero()

#[allow(non_snake_case)]
fn given__channel_lagged__when__recv__then__logs_warning()

#[allow(non_snake_case)]
fn given__invalid_api_key__when__auth_middleware__then__returns_401()
```

**Multiple preconditions:**
```rust
#[allow(non_snake_case)]
fn given__buffer_full_and_timeout_elapsed__when__flush__then__writes_all_trades()

#[allow(non_snake_case)]
fn given__duplicate_trade_and_conflict__when__insert__then__ignores_duplicate()
```

**No preconditions (rare):**
```rust
#[allow(non_snake_case)]
fn when__create_coin_pair__then__formats_uppercase()
```

**Test modules:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod vwap_calculation {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__equal_prices__when__calculate__then__returns_price() { }

        #[test]
        #[allow(non_snake_case)]
        fn given__zero_volume__when__calculate__then__returns_last_price() { }
    }
}
```

**Why Given-When-Then with Double Underscores?**
- **Clarity:** Double underscores (`__`) clearly separate Given-When-Then sections
- **Readability:** Easier to parse test names at a glance
- **Documentation:** Tests read like behavioral specifications
- **BDD Alignment:** Maps to Gherkin/Cucumber scenarios in user stories
- **Consistency:** Same pattern across entire codebase
- **Learning:** Teaches thinking in behavior-driven terms
- **Warning Suppression:** Use `#[allow(non_snake_case)]` to suppress Rust warnings

## Unit Testing Patterns

### Pure Function Testing

**Example: VWAP Calculation**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    #[allow(non_snake_case)]
    fn given__single_trade__when__calculate_vwap__then__returns_trade_price() {
        let aggregator = create_test_aggregator();
        let trades = vec![
            Trade {
                trade_id: 1,
                symbol: "BTC/USDT".to_string(),
                price: dec!(50000),
                quantity: dec!(1),
                timestamp: 1234567890,
            }
        ];

        let result = aggregator.calculate_vwap(&trades);

        assert_eq!(result.vwap, dec!(50000));
        assert_eq!(result.symbol, CoinPair::new("BTC", "USDT").unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__weighted_trades__when__calculate_vwap__then__returns_volume_weighted_price() {
        let aggregator = create_test_aggregator();
        let trades = vec![
            Trade { price: dec!(100), quantity: dec!(1), /* ... */ },
            Trade { price: dec!(102), quantity: dec!(2), /* ... */ },
        ];

        let result = aggregator.calculate_vwap(&trades);

        // VWAP = (100*1 + 102*2) / (1+2) = 304/3 = 101.333...
        assert_eq!(result.vwap, dec!(101.333333333333333333333333333));
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__zero_volume_trade__when__calculate_vwap__then__returns_last_price() {
        let aggregator = create_test_aggregator();
        let trades = vec![
            Trade { price: dec!(100), quantity: dec!(0), /* ... */ }
        ];

        let result = aggregator.calculate_vwap(&trades);

        assert_eq!(result.vwap, dec!(100));
    }

    // Test helper
    fn create_test_aggregator() -> VWapAggregator {
        let (trade_tx, trade_rx) = broadcast::channel(10);
        let (vwap_tx, _) = broadcast::channel(10);
        VWapAggregator::new(
            CoinPair::new("BTC", "USDT").unwrap(),
            trade_rx,
            vwap_tx,
            60,
        )
    }
}
```

**C# Comparison:**
- Test helper functions ≈ Test fixture setup in xUnit/NUnit
- `#[test]` ≈ `[Fact]` or `[Test]` attribute
- `assert_eq!` ≈ `Assert.Equal()`

### Error Path Testing

```rust
#[test]
#[allow(non_snake_case)]
fn given__empty_symbol__when__create_coin_pair__then__returns_error() {
    let result = CoinPair::new("", "USDT");

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Base symbol cannot be empty"
    );
}

#[test]
#[allow(non_snake_case)]
#[should_panic(expected = "Invalid timestamp")]
fn given__future_timestamp__when__validate__then__panics() {
    let future_timestamp = u64::MAX;
    validate_timestamp(future_timestamp);
}
```

### Async Function Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]  // Not #[test]
    #[allow(non_snake_case)]
    async fn given__valid_trade__when__database_write__then__inserts_record() {
        let pool = create_test_pool().await;
        let writer = TradeWriter::new(rx, pool.clone());

        let trade = Trade { /* ... */ };
        writer.write_trade(&trade).await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM trades")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count, 1);
    }
}
```

## Integration Testing Patterns

### Testcontainers for Database Tests

**Pattern:**
```rust
use testcontainers::{clients::Cli, GenericImage};

#[tokio::test]
async fn test_trade_repository_insert_and_query() {
    // Setup: Spin up container
    let docker = Cli::default();
    let postgres = docker.run(
        GenericImage::new("timescale/timescaledb-ha", "pg18.1-ts2.23.0-all")
            .with_env_var("POSTGRES_PASSWORD", "password")
            .with_env_var("POSTGRES_DB", "test_db")
    );

    let port = postgres.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:password@localhost:{}/test_db", port);

    // Create connection pool
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .unwrap();

    // Run migrations
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .unwrap();

    // Test: Insert trade
    let trade = Trade { /* ... */ };
    sqlx::query!(
        "INSERT INTO trades (timestamp, pair, trade_id, price, quantity) VALUES ($1, $2, $3, $4, $5)",
        trade.timestamp as i64,
        trade.symbol,
        trade.trade_id as i64,
        trade.price,
        trade.quantity
    )
    .execute(&pool)
    .await
    .unwrap();

    // Verify: Query back
    let result = sqlx::query!("SELECT * FROM trades WHERE trade_id = $1", trade.trade_id as i64)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(result.price.unwrap(), trade.price);
    assert_eq!(result.quantity.unwrap(), trade.quantity);

    // Cleanup: Container auto-drops when `postgres` goes out of scope (RAII)
}
```

**C# Comparison:**
- Testcontainers ≈ `IClassFixture<PostgresContainer>` in xUnit
- RAII cleanup ≈ `IDisposable` pattern
- Container scope = test function scope (not test class)

### Channel Testing

```rust
#[tokio::test]
async fn test_aggregator_receives_trades_and_emits_vwap() {
    // Setup channels
    let (trade_tx, trade_rx) = broadcast::channel(10);
    let (vwap_tx, mut vwap_rx) = broadcast::channel(10);

    // Create aggregator with 1-second window
    let aggregator = VWapAggregator::new(
        CoinPair::new("BTC", "USDT").unwrap(),
        trade_rx,
        vwap_tx,
        1,
    );

    // Spawn aggregator task
    let handle = tokio::spawn(aggregator.run());

    // Send test trades
    trade_tx.send(Trade { price: dec!(100), quantity: dec!(1), /* ... */ }).unwrap();
    trade_tx.send(Trade { price: dec!(102), quantity: dec!(2), /* ... */ }).unwrap();

    // Wait for window to complete (1 second + buffer)
    tokio::time::sleep(Duration::from_millis(1100)).await;

    // Verify VWAP emitted
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        vwap_rx.recv()
    ).await.unwrap().unwrap();

    assert_eq!(result.vwap, dec!(101.333333333333333333333333333));

    // Cleanup
    drop(trade_tx);  // Close sender, triggers aggregator shutdown
    handle.await.unwrap();
}
```

### WebSocket Testing (Mock Server)

```rust
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn test_binance_client_handles_trade_message() {
    // Start mock WebSocket server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut write, _) = ws_stream.split();

        // Send mock trade message
        let trade_json = r#"{"t": 1234, "s": "BTCUSDT", "p": "50000", "q": "1"}"#;
        write.send(Message::Text(trade_json.to_string())).await.unwrap();
    });

    // Test client connection
    let (trade_tx, mut trade_rx) = broadcast::channel(10);
    let url = format!("ws://{}", addr);
    let client = BinanceClient::new_with_url(url, trade_tx);

    tokio::spawn(client.run());

    // Verify trade received
    let trade = trade_rx.recv().await.unwrap();
    assert_eq!(trade.trade_id, 1234);
    assert_eq!(trade.price, dec!(50000));
}
```

## Test Data Builders

**Pattern: Fluent Builder for Test Data**

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;

    pub struct TradeBuilder {
        trade: Trade,
    }

    impl TradeBuilder {
        pub fn new() -> Self {
            Self {
                trade: Trade {
                    trade_id: 1,
                    symbol: "BTC/USDT".to_string(),
                    price: dec!(50000),
                    quantity: dec!(1),
                    timestamp: 1234567890,
                }
            }
        }

        pub fn with_id(mut self, id: u64) -> Self {
            self.trade.trade_id = id;
            self
        }

        pub fn with_price(mut self, price: Decimal) -> Self {
            self.trade.price = price;
            self
        }

        pub fn with_quantity(mut self, quantity: Decimal) -> Self {
            self.trade.quantity = quantity;
            self
        }

        pub fn build(self) -> Trade {
            self.trade
        }
    }
}

// Usage in tests:
#[test]
fn test_with_builder() {
    let trade = TradeBuilder::new()
        .with_price(dec!(60000))
        .with_quantity(dec!(0.5))
        .build();

    assert_eq!(trade.price, dec!(60000));
}
```

**C# Comparison:**
- Builder pattern ≈ Same in C# (fluent interface)
- Test helpers ≈ Test utility classes

## Property-Based Testing (Advanced)

**Future enhancement using `proptest`:**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_vwap_is_within_min_max_price_range(
        trades in prop::collection::vec(arbitrary_trade(), 1..100)
    ) {
        let aggregator = create_test_aggregator();
        let result = aggregator.calculate_vwap(&trades);

        let min_price = trades.iter().map(|t| t.price).min().unwrap();
        let max_price = trades.iter().map(|t| t.price).max().unwrap();

        prop_assert!(result.vwap >= min_price);
        prop_assert!(result.vwap <= max_price);
    }
}

fn arbitrary_trade() -> impl Strategy<Value = Trade> {
    (1u64..1000000, 1u64..100000, 1u64..10).prop_map(|(id, price, qty)| {
        Trade {
            trade_id: id,
            symbol: "BTC/USDT".to_string(),
            price: Decimal::from(price),
            quantity: Decimal::from(qty),
            timestamp: 1234567890,
        }
    })
}
```

**C# Comparison:**
- proptest ≈ FsCheck or AutoFixture with theories

## Logging in Tests

**Enable tracing output:**

```rust
use test_log::test;  // Instead of #[test]

#[test(tokio::test)]
async fn test_with_logging() {
    tracing::info!("This log will be visible if test fails");

    let result = process_data().await;

    assert!(result.is_ok());
}
```

**Setup tracing for tests:**
```rust
#[cfg(test)]
mod tests {
    use tracing_subscriber;

    #[ctor::ctor]
    fn init_test_logging() {
        tracing_subscriber::fmt()
            .with_test_writer()
            .init();
    }
}
```

## Coverage Guidelines

**Target coverage:**
- Core domain logic: >80%
- Application filters/sinks: >70%
- Infrastructure adapters: >60%
- API handlers: >50%

**Measure with:**
```bash
cargo llvm-cov --workspace --html
```

**Focus on:**
- Happy path coverage
- Error path coverage
- Edge cases (empty input, null, max values)

**Don't obsess over:**
- 100% coverage (diminishing returns)
- Trivial getters/setters
- Generated code

## Test Traits and Assertions

### Common Assertions

```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, unexpected);

// Boolean
assert!(condition);
assert!(!condition);

// Results
assert!(result.is_ok());
assert!(result.is_err());
result.unwrap();  // Panics with message if Err
result.expect("descriptive message");

// Options
assert!(option.is_some());
assert!(option.is_none());
assert_eq!(option.unwrap(), expected);

// Floating point (use approx crate)
use approx::assert_relative_eq;
assert_relative_eq!(actual, expected, epsilon = 0.0001);
```

### Custom Assertions

```rust
#[cfg(test)]
mod assertions {
    pub fn assert_trade_equals(actual: &Trade, expected: &Trade) {
        assert_eq!(actual.trade_id, expected.trade_id, "trade_id mismatch");
        assert_eq!(actual.symbol, expected.symbol, "symbol mismatch");
        assert_eq!(actual.price, expected.price, "price mismatch");
        assert_eq!(actual.quantity, expected.quantity, "quantity mismatch");
    }
}
```

## Anti-Patterns

**Avoid:**
- ❌ Testing private functions directly (test behavior via public API)
- ❌ Mocking everything (prefer real dependencies when cheap)
- ❌ Tests with no assertions (test should verify something)
- ❌ Tests dependent on execution order
- ❌ Tests with hardcoded timestamps (use relative times)
- ❌ Ignoring test failures ("it works on my machine")

**Use:**
- ✅ Test public interfaces
- ✅ Real database via Testcontainers
- ✅ Clear assertion with failure message
- ✅ Independent tests (parallel execution)
- ✅ Deterministic test data
- ✅ Fix or document flaky tests

## CI Integration

Tests run automatically in GitHub Actions:

```yaml
# .github/workflows/ci.yml
test:
  runs-on: ubuntu-latest
  services:
    postgres:
      image: timescale/timescaledb-ha:pg18.1-ts2.23.0-all
  steps:
    - run: cargo test --workspace --all-features
```

**Locally:**
```bash
make test           # Run all tests
cargo test          # Same
cargo test -- --nocapture  # Show println! output
cargo test test_name  # Run specific test
```

## Summary Checklist

Before committing tests:

- [ ] Tests follow naming convention: `test_{function}_{scenario}_{outcome}`
- [ ] Unit tests for pure functions (no IO)
- [ ] Integration tests for database/WebSocket/channels
- [ ] Async tests use `#[tokio::test]`
- [ ] Database tests use Testcontainers
- [ ] All assertions have clear failure messages
- [ ] Tests are deterministic (no randomness, fixed data)
- [ ] Tests clean up resources (channels closed, tasks joined)
- [ ] Coverage focuses on behavior, not line count
- [ ] Tests pass locally: `cargo test`

**C# Developer Notes:**
- No test project separation (tests in same crate or `tests/` dir)
- `#[test]` ≈ `[Fact]` / `[Test]`
- `#[tokio::test]` ≈ `async Task` test methods
- Testcontainers ≈ Same library, different API
- No test runners (cargo test built-in)
