# User Story Template

Use this template when creating new user stories. Stories use Gherkin-style acceptance criteria for testability.

## File Naming Convention

```
docs/user_stories/{number}_{feature_name_snake_case}.md
```

**Examples:**
- `1_database_integration_testing.md`
- `12_timescaledb_hypertable_optimization.md`
- `21_websocket_reconnection_logic.md`

## Template Structure

```markdown
# {Feature Name in Title Case}

## User Story
As a {role}, I need {capability} so that {business value}.

## Problem Statement (Optional - for complex features)
Brief explanation of the problem being solved, including:
- Current pain points
- Technical challenges
- Cost/performance implications
- Why this matters for a learning project

## Acceptance Criteria

### {Logical Grouping 1} (Optional)
- [ ] Given {precondition}, when {action}, then {expected outcome}
- [ ] Given {precondition}, when {action}, then {expected outcome}

### {Logical Grouping 2} (Optional)
- [ ] {Technical requirement}
- [ ] {Technical requirement with specific metric}

## Technical Context
Brief explanation of:
- Technology/patterns being used
- How it fits into existing architecture
- References to existing infrastructure/config files

**C# Comparison:** (Optional - if relevant concept exists)
- {Rust concept} ≈ {C# equivalent}

## Dependencies
- {User Story N} (or None)
- {Prerequisite system/service}

## Estimated Complexity
{Low | Medium | High}

**Reasoning:** (Optional - for High complexity)
Brief explanation of complexity drivers.
```

## Real Example: Following Template

```markdown
# API Key Authentication Middleware

## User Story
As an operator, I need to restrict API access to authenticated clients so that unauthorized users cannot consume system resources.

## Acceptance Criteria

### Core Functionality
- [ ] Given a request with valid X-API-Key header, when endpoint is accessed, then response is 200 OK
- [ ] Given a request with invalid X-API-Key header, when endpoint is accessed, then response is 401 Unauthorized
- [ ] Given a request with missing X-API-Key header, when endpoint is accessed, then response is 401 Unauthorized
- [ ] Given a public endpoint (health check), when accessed without key, then response is 200 OK

### Configuration
- [ ] API key stored in environment variable
- [ ] Public endpoints configurable via whitelist

### Testing (Test-First)
- [ ] Write failing tests first (from acceptance criteria)
- [ ] Integration test verifies authentication success path
- [ ] Integration test verifies authentication failure path

## Technical Context
Middleware pattern using Tower layers. Integrates with Axum request pipeline.

**C# Comparison:**
- Tower middleware ≈ ASP.NET Core middleware pipeline
- ApiKeyAuthLayer ≈ Custom AuthenticationHandler

## Dependencies
None

## Estimated Complexity
Low
```

## Acceptance Criteria Guidelines

### Use Gherkin Format for Behavior

**Format:** Given-When-Then
```
Given {initial context/state}
When {action/event occurs}
Then {expected outcome/verification}
```

**Good Examples:**
```markdown
- [ ] Given database contains 1000 trades, when pagination request for page 2 with size 50, then response contains trades 51-100
- [ ] Given consumer is lagging by 500 messages, when new message arrives, then warning is logged with lag count
- [ ] Given circuit breaker is open, when request is made, then request fails immediately without calling exchange
```

**Also Acceptable:** Technical requirements (not behavior)
```markdown
- [ ] Compression policy configured for data older than 7 days
- [ ] Metrics exported to Prometheus format at /metrics endpoint
- [ ] Response time p99 < 50ms under 1000 req/s load
```

### Grouping Acceptance Criteria

Use logical groupings when story has multiple concerns:

```markdown
### Core Implementation
- [ ] {Core behavior 1}
- [ ] {Core behavior 2}

### Error Handling
- [ ] {Error scenario 1}
- [ ] {Error scenario 2}

### Observability
- [ ] {Logging requirement}
- [ ] {Metrics requirement}

### Configuration
- [ ] {Config requirement 1}
```

### Make Criteria Testable

**Bad (untestable):**
```markdown
- [ ] System should be fast
- [ ] Error handling should be good
- [ ] Logging should be comprehensive
```

**Good (testable):**
```markdown
- [ ] API response time p99 < 100ms for market summary endpoint
- [ ] Database connection failures logged with error level and retry count
- [ ] All WebSocket messages include structured fields: exchange, symbol, timestamp
```

### Avoid Implementation Details in Criteria

**Bad (too specific):**
```markdown
- [ ] Use Arc<RwLock<HashMap<Exchange, Price>>> for cache
- [ ] Spawn tokio task with name "price-updater"
```

**Good (behavior-focused):**
```markdown
- [ ] Price cache supports concurrent reads without blocking
- [ ] Price updater task runs in background and logs lifecycle events
```

Implementation details belong in "Technical Context" section.

## Optional Sections

### Problem Statement
Use when feature needs context:
- Data volume/cost analysis (see Story 12)
- Performance justification
- Architectural decision rationale

### C# Comparison
Include when:
- Rust concept has clear C# equivalent
- Helps bridge understanding for C# developers
- Pattern exists in ASP.NET Core / .NET ecosystem

### Reasoning (under Complexity)
Explain High complexity ratings:
- Multiple system integrations
- Complex state management
- Requires new architectural patterns

## Common User Story Patterns

### Infrastructure Integration
```markdown
## User Story
As a developer, I need {service} integration tests so that I can verify {behavior} against real infrastructure.

### Acceptance Criteria
- [ ] Testcontainers spins up {service} in test environment
- [ ] Tests run in isolation without port conflicts
- [ ] Container lifecycle managed via RAII
```

### Data Pipeline Component
```markdown
## User Story
As a system, I need a {filter/sink} that {transforms/persists} data so that {downstream benefit}.

### Acceptance Criteria
- [ ] Given {input data}, when processed, then {output/side-effect}
- [ ] Given channel closed, when detected, then graceful shutdown with cleanup
- [ ] Given consumer lagging, when detected, then warning logged
```

### API Endpoint
```markdown
## User Story
As a {client type}, I need {endpoint} that returns {data} so that {use case}.

### Acceptance Criteria
### Core Functionality
- [ ] Given {data exists}, when GET {endpoint}, then response contains {expected data}
- [ ] Given {data missing}, when GET {endpoint}, then response is 404 Not Found

### Performance
- [ ] Response time p99 < {threshold}ms

### Error Handling
- [ ] Given {error condition}, when {endpoint accessed}, then {error response}
```

### Observability Feature
```markdown
## User Story
As an operator, I need {metrics/logs/traces} so that I can {monitor/debug/alert} on {condition}.

### Acceptance Criteria
### Data Collection
- [ ] {Component} emits structured logs with fields: {list}
- [ ] Metrics include: {counter/gauge/histogram names}

### Integration
- [ ] Logs flow through Vector → Loki → Grafana
- [ ] Dashboard query: {LogQL/PromQL example}
```

## Anti-Patterns

**Avoid:**
- ❌ Vague criteria: "Should work well"
- ❌ Missing role in user story: "I need feature X"
- ❌ Implementation-first: "Use library X to do Y"
- ❌ No business value: "so that we have feature X"
- ❌ Untestable criteria: "System should be reliable"

**Use:**
- ✅ Specific outcomes: "Response time p99 < 50ms"
- ✅ Clear roles: "As an operator" / "As a trader" / "As a system"
- ✅ Behavior-focused: "When consumer lags, then warning logged"
- ✅ Clear value: "so that I can identify arbitrage opportunities"
- ✅ Testable: "Given X, when Y, then Z"

## Test-First Implementation (MANDATORY)

When implementing user stories, follow test-first development:

**Process:**
1. **Read acceptance criteria** → Identify Given-When-Then scenarios
2. **Write failing tests** → Translate criteria to test functions (use `#[allow(non_snake_case)]` with Given-When-Then naming)
3. **Run tests** → Verify they fail (`cargo test`)
4. **Implement code** → Make tests pass
5. **Refactor** → Improve while keeping tests green
6. **Verify** → Final `cargo test` and `cargo check`

**Example mapping:**
```markdown
Acceptance Criteria:
- [ ] Given valid API key, when endpoint accessed, then response is 200 OK

Test function:
#[test]
#[allow(non_snake_case)]
async fn given__valid_api_key__when__endpoint_accessed__then__returns_200_ok()
```

**See:** [ai/testing-guidelines.md](testing-guidelines.md) lines 5-57 for complete test-first workflow.

## Checklist Before Creating Story

- [ ] User story follows "As a {role}, I need {capability} so that {value}" format
- [ ] Acceptance criteria use Gherkin (Given-When-Then) for behaviors
- [ ] All criteria are testable (can verify pass/fail)
- [ ] Acceptance criteria can be directly mapped to test function names
- [ ] Dependencies identified (or marked "None")
- [ ] Complexity estimated with reasoning if High
- [ ] Technical context explains architecture fit
- [ ] File named following convention: `{N}_{snake_case}.md`

## Integration with INDEX.md

After creating user story, propose INDEX.md update:

```markdown
N. **{Story Title}** → [docs/user_stories/N_story.md](...)
   - Keywords: {keyword1}, {keyword2}, {keyword3}
   - Dependencies: Story X, Story Y (or None)
   - Complexity: {Low/Medium/High}
```

See [ai/META_GUIDELINES.md](META_GUIDELINES.md) for INDEX update protocol.

## Examples by Category

**Foundation Stories:**
- [1_database_integration_testing.md](../docs/user_stories/1_database_integration_testing.md)
- [12_timescaledb_hypertable_optimization.md](../docs/user_stories/12_timescaledb_hypertable_optimization.md)

**API Stories:**
- [4_hot_path_market_summary_endpoint.md](../docs/user_stories/4_hot_path_market_summary_endpoint.md)
- [5_keyset_pagination_trades.md](../docs/user_stories/5_keyset_pagination_trades.md)

**Pipeline Stories:**
- [3_lead_lag_analytics.md](../docs/user_stories/3_lead_lag_analytics.md)

**Resilience Stories:**
- [9_circuit_breaker_exchange_failures.md](../docs/user_stories/9_circuit_breaker_exchange_failures.md)
- [17_graceful_shutdown.md](../docs/user_stories/17_graceful_shutdown.md)
