# Testing Strategy Mode

You are now in **Testing Strategy Mode** for the Dreadnought project.

## Context Loaded
- Testing guidelines from [ai/testing-guidelines.md](../ai/testing-guidelines.md)
- Architecture patterns from [ai/architecture.md](../ai/architecture.md)
- Code guidelines from [ai/code-guidelines.md](../ai/code-guidelines.md)

## Test Pyramid Target
- **60%** Unit tests (pure functions, business logic)
- **35%** Integration tests (database, external services)
- **5%** End-to-end tests (full system flows)

## Test Strategy by Layer

### Core (Domain)
**Focus**: Pure functions, domain logic
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_valid_coin_pair() {
        let pair = CoinPair::new("BTC/USD").unwrap();
        assert_eq!(pair.base(), "BTC");
        assert_eq!(pair.quote(), "USD");
    }
}
```
**Guidelines**:
- No external dependencies
- Fast execution (<1ms per test)
- Cover edge cases, validation logic

### Application (Filters, Sinks)
**Focus**: Business logic, channel interactions
```rust
#[tokio::test]
async fn vwap_aggregator_calculates_correctly() {
    let (tx, rx) = broadcast::channel(100);
    let aggregator = VwapAggregator::new(rx);

    // Send test trades
    tx.send(Trade { price: 100.0, quantity: 1.0, .. }).unwrap();
    tx.send(Trade { price: 200.0, quantity: 2.0, .. }).unwrap();

    // Assert VWAP = (100*1 + 200*2) / (1+2) = 166.67
}
```
**Guidelines**:
- Mock channel interactions
- Test aggregation logic
- Verify error propagation

### Infrastructure (Exchange Clients, Database)
**Focus**: External service contracts
```rust
#[tokio::test]
async fn binance_client_handles_reconnection() {
    let client = BinanceClient::new(mock_config());

    // Simulate disconnect
    // Assert reconnection attempts
    // Verify message recovery
}
```
**Database tests** (use Testcontainers):
```rust
#[sqlx::test]
async fn trade_writer_batches_inserts(pool: PgPool) {
    let writer = TradeWriter::new(pool.clone());

    // Send 1000 trades
    // Assert batched into chunks of 100
    // Verify all written to database
}
```
**Guidelines**:
- Use Testcontainers for real databases
- Test connection failures, retries
- Verify data serialization

### API Server (HTTP Handlers)
**Focus**: Request/response contracts
```rust
#[tokio::test]
async fn health_check_returns_200() {
    let app = create_test_app();

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```
**Guidelines**:
- Use Axum's test utilities
- Test middleware (auth, timing)
- Verify error responses

## Test Data Builders
For complex structs:
```rust
struct TradeBuilder {
    price: f64,
    quantity: f64,
    // ... other fields with defaults
}

impl TradeBuilder {
    fn new() -> Self {
        Self {
            price: 100.0,
            quantity: 1.0,
            // ... sensible defaults
        }
    }

    fn price(mut self, price: f64) -> Self {
        self.price = price;
        self
    }

    fn build(self) -> Trade {
        Trade { /* construct from self */ }
    }
}

// Usage in tests
let trade = TradeBuilder::new().price(150.0).build();
```

## Testing Async Code
**Pattern 1**: `tokio::test` macro
```rust
#[tokio::test]
async fn async_function_completes() {
    let result = some_async_fn().await;
    assert!(result.is_ok());
}
```

**Pattern 2**: Testing with timeout
```rust
#[tokio::test]
async fn websocket_connects_within_5s() {
    timeout(Duration::from_secs(5), client.connect())
        .await
        .expect("Connection timeout");
}
```

## Integration Test Setup
```rust
// tests/integration_test.rs
use testcontainers::{clients::Cli, images::postgres::Postgres, Container};

#[tokio::test]
async fn test_with_database() {
    let docker = Cli::default();
    let postgres = docker.run(Postgres::default());
    let port = postgres.get_host_port_ipv4(5432);

    let database_url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Run migrations
    sqlx::migrate!().run(&pool).await.unwrap();

    // Test database operations
}
```

## C# Comparison
- `#[tokio::test]` ≈ `[Fact]` with async support
- `testcontainers` ≈ `Testcontainers.DotNet`
- `mockall` crate ≈ `Moq`
- `assert_eq!` ≈ `Assert.Equal`

## Test Checklist
For each feature:
- [ ] Unit tests for pure functions
- [ ] Integration tests for external dependencies
- [ ] Error path coverage (not just happy path)
- [ ] Edge cases (empty input, max values, null/None)
- [ ] Test data builders for complex structs
- [ ] Async tests use `tokio::test` with proper runtime

## Commands
```bash
make test           # Run all tests
make coverage       # Generate coverage report
cargo test          # Run tests directly
cargo test -- --nocapture  # Show println! output
cargo test test_name --exact  # Run specific test
```

Ready to design test strategy. Describe the feature or code you want to test.
