# Code Review Guidelines

This document defines the standards, metrics, and processes for code review in the Dreadnought project.

## Review Philosophy

Code reviews serve three purposes:
1. **Quality Gate**: Ensure code meets architectural and quality standards
2. **Knowledge Transfer**: Share patterns and best practices across the team
3. **Continuous Improvement**: Identify areas for enhancement

Reviews should be **objective, constructive, and actionable**. Use metrics to quantify quality and track improvements over time.

---

## Review Scoring System

Each change receives a score from 0-100 based on weighted categories. Target: **≥ 80 for approval**.

### Scoring Breakdown

| Category | Weight | Points | Description |
|----------|--------|--------|-------------|
| Architecture Compliance | 30% | 30 | Clean Architecture boundaries, layer dependencies, trait usage |
| Code Quality | 25% | 25 | Naming, error handling, type safety, self-documenting code |
| Testing | 20% | 20 | Test-first compliance, coverage, test quality |
| Async Patterns | 10% | 10 | Tokio usage, channel selection, graceful shutdown |
| Performance | 10% | 10 | Allocations, database efficiency, logging overhead |
| Security | 5% | 5 | Input validation, SQL injection prevention, secrets management |
| **TOTAL** | **100%** | **100** | |

### Score Interpretation

- **90-100**: Excellent - Exemplary code, minimal improvements
- **80-89**: Good - Approved with minor suggestions
- **70-79**: Acceptable - Approved with recommended improvements
- **60-69**: Needs Work - Conditional approval, improvements required
- **< 60**: Reject - Significant issues, rework needed

---

## Detailed Scoring Criteria

### 1. Architecture Compliance (30 points)

**Clean Architecture Boundaries (15 points)**
- ✅ **15 pts**: Perfect layer separation
  - `core` has zero dependencies
  - `application` depends only on `core`
  - `infrastructure` depends only on `core`
  - `api_server` orchestrates all layers
- ⚠️ **10 pts**: Minor violations (e.g., infrastructure imports from application)
- ❌ **5 pts**: Major violations (e.g., core depends on infrastructure)
- ❌ **0 pts**: Architecture completely broken

**Pipe & Filters Pattern (8 points)**
- ✅ **8 pts**: Correct channel usage for data flow
  - `broadcast::channel` for fan-out (1 producer → N consumers)
  - `mpsc::channel` for point-to-point (1 producer → 1 consumer)
- ⚠️ **5 pts**: Works but inefficient (e.g., mpsc where broadcast needed)
- ❌ **2 pts**: Incorrect pattern (e.g., polling instead of channels)
- ❌ **0 pts**: No async communication or blocking patterns

**Trait Usage (7 points)**
- ✅ **7 pts**: Abstractions in core, implementations in infrastructure
- ⚠️ **4 pts**: Some traits in wrong layer but functional
- ❌ **2 pts**: Concrete types where traits should be used
- ❌ **0 pts**: No abstraction, tight coupling

---

### 2. Code Quality (25 points)

**Self-Documenting Code (8 points)**
- ✅ **8 pts**: Code explains itself, comments only for complex algorithms
- ⚠️ **5 pts**: Some unnecessary comments (e.g., `// Create client`)
- ❌ **2 pts**: Heavy commenting masking unclear code
- ❌ **0 pts**: No clarity, requires extensive comments

**Naming Conventions (6 points)**
- ✅ **6 pts**: All conventions followed
  - Traits: `PastTense` (e.g., `ExchangeConnected`)
  - Structs: `PascalCase` (e.g., `BinanceClient`)
  - Functions: `snake_case` (e.g., `calculate_vwap`)
  - Constants: `SCREAMING_SNAKE_CASE`
- ⚠️ **4 pts**: Minor inconsistencies (e.g., `get_` prefix on non-getter)
- ❌ **2 pts**: Multiple violations
- ❌ **0 pts**: No convention adherence

**Error Handling (6 points)**
- ✅ **6 pts**: Perfect error handling
  - Libraries use `thiserror`
  - Applications use `anyhow`
  - All public APIs return `Result<T, E>`
  - Errors propagated with `?`, not swallowed
- ⚠️ **4 pts**: Minor issues (e.g., using `String` instead of custom error type)
- ❌ **2 pts**: `.unwrap()` or `.expect()` in production code
- ❌ **0 pts**: Panics or ignored errors

**Type Safety (5 points)**
- ✅ **5 pts**: Newtype pattern for domain concepts
  - Example: `CoinPair` struct, not `String`
- ⚠️ **3 pts**: Some primitive obsession (e.g., `String` for symbols)
- ❌ **1 pt**: Heavy primitive obsession
- ❌ **0 pts**: Stringly-typed code

---

### 3. Testing (20 points)

**Test-First Compliance (8 points)**
- ✅ **8 pts**: Test-first followed (red-green-refactor)
  - Tests written before implementation
  - Commit history shows test commits before code
- ⚠️ **5 pts**: Tests written alongside code
- ❌ **2 pts**: Tests written after implementation
- ❌ **0 pts**: No tests

**Test Coverage (6 points)**
- ✅ **6 pts**: Comprehensive coverage
  - Happy path + edge cases + error conditions
  - Public API fully tested
- ⚠️ **4 pts**: Happy path covered, some edge cases missing
- ❌ **2 pts**: Only happy path
- ❌ **0 pts**: Minimal or no coverage

**Test Quality (6 points)**
- ✅ **6 pts**: High-quality tests
  - Given-When-Then naming (`given__X__when__Y__then__Z`)
  - Focused assertions (test one thing)
  - No test interdependencies
  - Integration tests use Testcontainers
- ⚠️ **4 pts**: Tests work but naming/structure could improve
- ❌ **2 pts**: Brittle tests (flaky, order-dependent)
- ❌ **0 pts**: Tests don't run or always pass

---

### 4. Async Patterns (10 points)

**Tokio Usage (5 points)**
- ✅ **5 pts**: Idiomatic async
  - `tokio::select!` for cancellation
  - Proper `async`/`await` usage
  - No blocking calls in async context
- ⚠️ **3 pts**: Works but non-idiomatic (e.g., polling loops)
- ❌ **1 pt**: Mix of blocking and async
- ❌ **0 pts**: Blocking operations in async context

**Graceful Shutdown (5 points)**
- ✅ **5 pts**: Tasks handle cancellation via `select!` on shutdown signal
- ⚠️ **3 pts**: Shutdown works but not graceful (e.g., abrupt termination)
- ❌ **1 pt**: No shutdown mechanism
- ❌ **0 pts**: Resources leaked on shutdown

---

### 5. Performance (10 points)

**Allocations (5 points)**
- ✅ **5 pts**: Minimal allocations
  - Borrows preferred over clones
  - String allocations avoided in hot paths
  - `Cow<str>` used where appropriate
- ⚠️ **3 pts**: Some unnecessary allocations
- ❌ **1 pt**: Heavy allocation in hot paths
- ❌ **0 pts**: Excessive cloning, performance impact

**Database Efficiency (3 points)**
- ✅ **3 pts**: Efficient queries
  - Batch inserts used
  - Prepared statements via sqlx
  - Transactions for multi-statement operations
- ⚠️ **2 pts**: Works but could be optimized (e.g., N+1 queries)
- ❌ **1 pt**: Inefficient patterns (e.g., row-by-row inserts)
- ❌ **0 pts**: Major inefficiencies
- **N/A**: No database operations (award full points)

**Logging Overhead (2 points)**
- ✅ **2 pts**: Structured logging, no allocations in hot paths
  - `tracing::debug!(value = %x)` instead of `format!`
- ⚠️ **1 pt**: Some string allocation in logs
- ❌ **0 pts**: Heavy logging overhead
- **N/A**: No logging (award full points)

---

### 6. Security (5 points)

**Input Validation (2 points)**
- ✅ **2 pts**: All external input validated (WebSocket, API, user input)
- ⚠️ **1 pt**: Most input validated
- ❌ **0 pts**: No validation

**SQL Injection Prevention (2 points)**
- ✅ **2 pts**: Parameterized queries via sqlx
- ❌ **0 pts**: String concatenation for queries
- **N/A**: No database writes (award full points)

**Secrets Management (1 point)**
- ✅ **1 pt**: Environment variables, no hardcoded secrets
- ❌ **0 pt**: Hardcoded credentials/API keys

---

## Review Process

### 1. Pre-Review Checklist

Before requesting review:
- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Self-review completed using this rubric

### 2. Review Steps

1. **Quick Scan (2 min)**
   - Check layer boundaries
   - Verify tests exist
   - Look for obvious issues

2. **Deep Review (10-20 min)**
   - Score each category
   - Note specific issues with line references
   - Identify patterns (good and bad)

3. **Feedback (5 min)**
   - Calculate total score
   - Provide constructive feedback
   - Suggest concrete improvements

### 3. Review Output Format

```markdown
# Code Review: [Feature/PR Name]

## Score: [X]/100 - [Rating]

### Category Breakdown
- Architecture Compliance: [X]/30
- Code Quality: [X]/25
- Testing: [X]/20
- Async Patterns: [X]/10
- Performance: [X]/10
- Security: [X]/5

## Detailed Findings

### Architecture Compliance ([X]/30)
- **[✅/⚠️/❌] Clean Architecture**: [details]
- **[✅/⚠️/❌] Pipe & Filters**: [details]
- **[✅/⚠️/❌] Trait Usage**: [details]

### Code Quality ([X]/25)
[File references with line numbers]

### Testing ([X]/20)
[Test coverage analysis]

### Async Patterns ([X]/10)
[Async usage review]

### Performance ([X]/10)
[Performance considerations]

### Security ([X]/5)
[Security assessment]

## Recommendations

### Critical (must fix before merge)
1. [Issue with file:line reference]

### Important (should fix)
1. [Issue with file:line reference]

### Optional (nice to have)
1. [Suggestion]

## Positives
- [What was done well]

## Decision
[Approve/Approve with Changes/Reject]
```

---

## Common Issues & Patterns

### Anti-Patterns to Flag

**Architecture Violations**
```rust
// ❌ BAD: core depending on infrastructure
use infrastructure::binance::BinanceClient;

// ✅ GOOD: core defines trait, infrastructure implements
use app_core::exchange_client::ExchangeClient;
```

**Error Handling**
```rust
// ❌ BAD: unwrap in production
let value = result.unwrap();

// ✅ GOOD: propagate errors
let value = result?;

// ❌ BAD: String errors in libraries
fn parse(s: &str) -> Result<Trade, String>

// ✅ GOOD: Custom error types
fn parse(s: &str) -> Result<Trade, ParseError>
```

**Async Patterns**
```rust
// ❌ BAD: blocking in async
async fn process() {
    std::thread::sleep(Duration::from_secs(1));
}

// ✅ GOOD: async sleep
async fn process() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

**Testing**
```rust
// ❌ BAD: generic test name
#[test]
fn test_parser() { }

// ✅ GOOD: Given-When-Then naming
#[test]
#[allow(non_snake_case)]
async fn given__valid_message__when__parsed__then__returns_trade() { }
```

### Excellent Patterns to Highlight

**Symbol Normalization at Boundaries**
```rust
// ✅ Normalize external format → domain format
pub fn into_trade(self, symbol: String) -> Trade {
    let normalized_symbol = symbol.replace("XBT", "BTC");
    Trade { symbol: normalized_symbol, ... }
}
```

**Testable Extraction**
```rust
// ✅ Extract parsing logic for unit testing
pub fn parse_trade_message(&self, text: &str) -> Result<Trade, String> {
    // Parsing logic here
}

async fn handle_message(&self, msg: Message) -> Result<(), Error> {
    match msg {
        Message::Text(text) => {
            match self.parse_trade_message(&text) { ... }
        }
    }
}
```

**Error Resilience**
```rust
// ✅ Log parse errors, propagate fatal errors
match self.parse_trade_message(&text) {
    Ok(trade) => { /* send trade */ }
    Err(e) => {
        tracing::error!(error = ?e, "Parse failed, continuing");
        // Don't terminate WebSocket on parse errors
    }
}
```

---

## Metrics Tracking

Track these metrics over time to measure code quality trends:

### Per-PR Metrics
- Review score (0-100)
- Category scores
- Number of critical/important/optional issues
- Review time (minutes)

### Project-Wide Metrics
- Average review score (rolling 10 PRs)
- Test coverage percentage
- Clippy warnings count
- Build time

### Quality Trends
- Score improvement over time
- Reduction in critical issues
- Test-first compliance rate

---

## Calibration Examples

### Example 1: Kraken Integration (Score: 92/100)

**Category Breakdown**
- Architecture: 30/30 ✅
- Code Quality: 23/25 ⚠️ (could use ExchangeClientError instead of String)
- Testing: 20/20 ✅
- Async: 10/10 ✅
- Performance: 8/10 ⚠️ (minor allocation opportunity)
- Security: 1/1 ✅ (N/A database: 4/4)

**Rating**: Excellent - Approved

### Example 2: Hypothetical Poor PR (Score: 55/100)

**Category Breakdown**
- Architecture: 15/30 ❌ (infrastructure imports from application)
- Code Quality: 12/25 ❌ (unwrap usage, no error handling)
- Testing: 8/20 ❌ (only happy path tested)
- Async: 10/10 ✅
- Performance: 5/10 ⚠️ (excessive cloning)
- Security: 5/5 ✅

**Rating**: Reject - Significant rework needed

---

## References

- [Architecture Guidelines](./architecture.md)
- [Code Guidelines](./code-guidelines.md)
- [Testing Guidelines](./testing-guidelines.md)
- [Communication Guidelines](./guidelines.md)
- [User Story Template](./user-story-template.md)
