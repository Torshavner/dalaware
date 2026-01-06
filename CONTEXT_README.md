# Feed-Forward Neural Network Project (Dreadnought-AI)

**Project Status:** Planning/Pre-Implementation Phase
**Last Updated:** 2026-01-06

## Project Overview

A from-scratch Feed-Forward Neural Network framework built in Rust, designed as a learning project for a Senior C# Developer transitioning to Rust. The framework follows Clean Architecture principles and includes real-time visualization capabilities using ICED GUI framework.

## Current State

### What Exists
- **Documentation Structure**: Complete architectural planning and user stories
- **Infrastructure**: Docker-based observability stack (TimescaleDB, Loki, Grafana, Vector)
- **Project Scaffolding**: Directory structure, Makefiles, and build scripts
- **No Rust Code Yet**: The `src/` directory is empty - implementation has not started

### Project Structure

```
feed_forward_nn/
├── src/                          # EMPTY - Future Rust implementation
├── ai/                           # AI Assistant Guidelines & Architecture
│   ├── architecture.md           # Complete Clean Architecture design
│   ├── code-guidelines.md        # Rust coding standards (650 lines)
│   ├── guidelines.md             # Communication & learning approach
│   ├── testing-guidelines.md     # TDD approach
│   ├── review-guidelines.md      # Code review standards
│   ├── mcp-guide.md             # MCP integration guide
│   └── user-story-template.md   # Story format template
├── docs/user-stories/           # Feature Specifications
│   ├── feed-forward neural network foundation.md
│   ├── neural network visualization dashboard.md
│   └── stuctured console observability.md
├── docker/                      # Observability Infrastructure
│   ├── docker-compose.yml       # Full monitoring stack
│   ├── loki-config.yml         # Log aggregation config
│   └── vector.toml             # Log shipping config
├── scripts/                     # Build & CI Scripts
│   ├── check.sh, fmt.sh, clippy.sh, test.sh
│   ├── build.sh, ci.sh, coverage.sh, bench.sh
│   └── docker-up.sh, docker-down.sh, db-reset.sh
└── Makefile                     # Development workflow commands
```

## Architecture Design

### Clean Architecture Layers

The framework is designed with strict layer separation:

1. **Core Layer** (`core/`)
   - Pure mathematical operations
   - Traits: `Module`, `Activation`, `Optimizer`, `Loss`
   - Type alias: `Tensor` (ndarray-based)
   - Forward/backward propagation interfaces

2. **Application Layer** (`application/`)
   - `Sequential`: Network container (`Vec<Box<dyn Module>>`)
   - `Trainer`: Orchestrates epochs, batching, gradient descent
   - Training loop logic

3. **Infrastructure Layer** (`infrastructure/`)
   - `Persistence`: Weight serialization (serde + JSON/MessagePack)
   - `Telemetry`: Integration with tracing → Vector → Loki → Grafana

4. **UI Layer** (`viz_dashboard/`)
   - ICED-based real-time visualization
   - Decision boundary planes
   - Loss curves, accuracy gauges
   - Weight distribution visualizations

### Key Design Principles

- **Trait-Driven Flexibility**: Activation functions, optimizers, and loss functions are traits
- **Generic Tensors**: `ndarray::Array<f32, D>` for N-dimensional operations
- **Compile-Time Safety**: Type system prevents dimension mismatches
- **Dynamic Dispatch**: `Box<dyn Module>` allows heterogeneous layer types in `Sequential`

### Data Flow (Training Pass)

1. **Forward**: Input → Layer → Activation → Layer → Activation → Output
2. **Loss Calculation**: Compare output vs targets (MSE/Cross-Entropy)
3. **Backward**: Reverse gradient calculation via Chain Rule
4. **Optimization**: SGD updates weights/biases using gradients

## User Stories & Features

### Story 1: Feed-Forward Neural Network Foundation (Medium Complexity)
**Status:** Not Started

**Acceptance Criteria:**
- [ ] `DenseLayer` struct with weights/biases (ndarray)
- [ ] Forward propagation with activation functions
- [ ] Backpropagation for gradient calculation
- [ ] Multiple activations: Sigmoid, ReLU, Softmax
- [ ] `Trainer` for epochs/batching/gradient descent
- [ ] Cross-Entropy loss for classification
- [ ] **Test Goals:**
  - [ ] Forward prop on 2×2 identity matrix
  - [ ] Gradient verification via numerical approximation
  - [ ] Solve XOR problem in <1000 epochs

**Dependencies:**
- `ndarray`, `ndarray-rand`, `rand`, `anyhow`, `serde`

### Story 2: Neural Network Visualization Dashboard (High Complexity)
**Status:** Not Started

**Acceptance Criteria:**
- [ ] Real-time loss curve (plotters-iced line chart)
- [ ] 2D decision boundary heatmap for XOR
- [ ] Weight distribution visualization (Sankey/mega-neurons)
- [ ] Live accuracy gauge (train vs validation)
- [ ] Interactive controls: Start/Pause training
- [ ] Hyperparameter sliders (learning rate, batch size)
- [ ] Cross-thread communication: tokio channel → Iced Subscription

**Dependencies:**
- `iced`, `plotters`, `plotters-iced`, `tokio`

**Technical Challenges:**
- Orchestrating high-speed training loop with non-blocking GUI updates
- Message passing between background thread and UI thread

### Story 3: Structured Console Observability (Low Complexity)
**Status:** Not Started

**Acceptance Criteria:**
- [ ] Human-readable console logs via `tracing-subscriber`
- [ ] `RUST_LOG` environment variable support
- [ ] Structured logging with context fields (symbol, exchange)
- [ ] Colorized output (ANSI) for level distinction
- [ ] UDP sink to Vector (port 9000) for Loki ingestion

**Dependencies:**
- `tracing`, `tracing-subscriber` (already standard in Rust ecosystem)

## Infrastructure Services

The Docker stack provides full observability (ready to use):

| Service | Port | Purpose | Access |
|---------|------|---------|---------|
| **PostgreSQL (TimescaleDB)** | 5432 | Time-series metrics storage | `postgres:password@localhost:5432/main_db` |
| **pgAdmin** | 8081 | Database admin UI | http://localhost:8081 (admin@example.com / admin) |
| **Loki** | 3100 | Log aggregation | HTTP API endpoint |
| **Vector** | 9000/udp, 8686 | Log shipping (Docker + UDP) | Receives logs from app + containers |
| **Grafana** | 3000 | Visualization dashboard | http://localhost:3000 (admin / admin) |

**Network:** All services on `monitoring-net` bridge network

**Volumes:**
- `pgdata`: PostgreSQL database
- `pgadmindata`: pgAdmin settings
- `grafanadata`: Grafana dashboards/settings
- `loki_data`: Loki indices

## Development Workflow

### Quick Start Commands

```bash
make help         # Show all available commands
make docker-up    # Start infrastructure (Postgres, Loki, Grafana, Vector)
make docker-down  # Stop all containers
make run          # Build and run application (once implemented)
```

### CI Pipeline

```bash
make ci           # Full pipeline: fmt → clippy → test → build
make fmt          # Check formatting (cargo fmt --check)
make fmt-fix      # Auto-fix formatting
make clippy       # Run lints
make test         # Run test suite
make coverage     # Generate coverage report
make bench        # Run benchmarks
```

### Database Management

```bash
make db-reset     # Drop and recreate database (DESTRUCTIVE)
make docker-logs SERVICE=postgres-timescale  # View service logs
```

## Key Technical Decisions

### Rust vs C# Analogies (For Context)

| Rust | C# Equivalent | Notes |
|------|---------------|-------|
| `Box<dyn Module>` | `IModule` interface | Dynamic dispatch for heterogeneous collections |
| `Arc<RwLock<T>>` | `ConcurrentDictionary<K,V>` / `ReaderWriterLockSlim` | Shared mutable state across threads |
| `tokio::spawn` | `Task.Run` | Async task spawning |
| `Result<T, E>` | `try-catch` exceptions | Railway-oriented error handling |
| `ndarray::Array2` | `MathNet.Numerics.Matrix<double>` | Linear algebra operations |
| `serde` derive macros | `[JsonSerializable]` source generator | Serialization |
| `tracing` structured logging | Serilog with sinks | Observability |

### Why Rust for Neural Networks?

1. **Educational Goal**: Understanding ML math from first principles
2. **Performance**: Zero-cost abstractions, no GC pauses
3. **Memory Safety**: Ownership system prevents data races
4. **Explicitness**: Forces understanding of matrix dimensions, memory layout

### Why ICED for Visualization?

- Native Rust GUI (no JS/Electron overhead)
- Elm-inspired architecture (message passing)
- `plotters-iced` integration for scientific visualizations
- Cross-platform (macOS, Linux, Windows)

## Communication Guidelines (AI Assistance)

**Developer Persona:** Senior C# Developer (.NET, ASP.NET Core, TPL)
**AI Persona:** Senior Rust Developer

**Tone Requirements:**
- ✅ **Use:** "That is correct," "Affirmative," "Your understanding is accurate"
- ❌ **Avoid:** "Great question!", "Fantastic!", "Absolutely!", "You're 100% correct!"
- ✅ **Use:** Definitive technical statements
- ❌ **Avoid:** Hedging ("I think," "probably," "might be")

**Code Philosophy:**
- Self-documenting code (no comments except for public APIs, safety invariants)
- Frame all explanations in C# terms where possible
- Focus on **why Rust does things differently**, not just syntax
- Emphasize architecture, trade-offs, ownership model

**Workflow:**
- Read files before proposing changes
- Incremental implementation with `cargo check` validation
- Error-first approach (show compiler errors verbatim)
- TDD cycles with `cargo test`

## Code Standards Highlights

### Critical Rules
- ❌ **No `unwrap()` in production paths** (use `?` operator or `expect()` in startup only)
- ✅ **Structured logging**: `tracing::info!(field = value)` not string interpolation
- ✅ **Prefer borrowing over cloning**: `&[Trade]` not `Vec<Trade>`
- ✅ **Handle channel errors explicitly**: `RecvError::Lagged`, `RecvError::Closed`
- ✅ **Use `tokio::time::sleep`, never `std::thread::sleep` in async**

### Naming Conventions
- Types: `PascalCase` (DenseLayer, VwapResult)
- Functions/variables: `snake_case` (calculate_vwap, trade_buffer)
- Constants: `SCREAMING_SNAKE_CASE` (DEFAULT_LEARNING_RATE)
- Lifetimes: `'a` or descriptive (`'data`, `'static`)

### Module Organization
```rust
// Good: Flat hierarchy
use application::pipeline::filters::VWapAggregator;

// Avoid: Deep nesting
use application::pipeline::filters::aggregators::vwap::VWapAggregator;
```

## Implementation Roadmap (Not Started)

### Phase 1: Core Foundation
1. Create Cargo workspace with `core`, `application`, `infrastructure`, `viz_dashboard` crates
2. Implement `Module` trait and `DenseLayer` struct
3. Implement activation functions (Sigmoid, ReLU, Softmax as traits)
4. Write forward propagation logic with unit tests

### Phase 2: Backpropagation & Training
5. Implement backward pass (gradient calculation)
6. Implement `Trainer` struct (epochs, batching, SGD)
7. Implement loss functions (MSE, Cross-Entropy)
8. Test: Solve XOR problem convergence

### Phase 3: Observability Integration
9. Integrate `tracing` with structured fields
10. Configure Vector UDP sink (port 9000)
11. Verify logs appear in Grafana/Loki

### Phase 4: Visualization Dashboard
12. ICED application scaffold
13. Plotters integration for loss curves
14. Decision boundary heatmap rendering
15. Cross-thread communication (tokio channel → Subscription)

## Next Steps (When Implementation Begins)

1. **Initialize Cargo Workspace**
   ```bash
   cargo init --lib core
   cargo init --lib application
   cargo init --lib infrastructure
   cargo init --bin viz_dashboard
   ```

2. **Add Dependencies to Workspace `Cargo.toml`**
   ```toml
   [workspace]
   members = ["core", "application", "infrastructure", "viz_dashboard"]

   [workspace.dependencies]
   ndarray = "0.15"
   ndarray-rand = "0.14"
   serde = { version = "1", features = ["derive"] }
   tokio = { version = "1", features = ["full"] }
   iced = "0.12"
   plotters = "0.3"
   tracing = "0.1"
   tracing-subscriber = "0.3"
   anyhow = "1"
   ```

3. **Define Core Traits** ([architecture.md:18-20](ai/architecture.md#L18-L20))
   ```rust
   // core/src/lib.rs
   pub trait Module {
       fn forward(&self, input: Tensor) -> Tensor;
       fn backward(&mut self, grad: Tensor) -> Tensor;
   }
   ```

4. **Test-First Approach**
   - Write failing test for 2×2 identity matrix forward pass
   - Implement `DenseLayer` to pass test
   - Repeat for backprop gradient verification

## Resources

### Official Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [ndarray Documentation](https://docs.rs/ndarray/latest/ndarray/)
- [ICED Book](https://book.iced.rs/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Project Files
- Architecture: [ai/architecture.md](ai/architecture.md)
- Code Guidelines: [ai/code-guidelines.md](ai/code-guidelines.md) (650 lines)
- Testing Guidelines: [ai/testing-guidelines.md](ai/testing-guidelines.md)
- User Stories: [docs/user-stories/](docs/user-stories/)

## Troubleshooting

### Docker Stack Issues
```bash
# Check service health
docker ps

# View logs for specific service
make docker-logs SERVICE=loki

# Reset everything
make docker-down
make docker-up
```

### Future Rust Compilation Issues
- Always run `cargo check` before committing
- Use `cargo clippy` to catch common mistakes
- Read compiler errors carefully (Rust errors are descriptive)

## Notes for Future Context Recovery

**If you lose context and return to this project:**

1. **Read this file first** to understand current state
2. **No code exists yet** - `src/` is empty, this is pre-implementation
3. **Architecture is fully planned** - see [ai/architecture.md](ai/architecture.md)
4. **Three user stories define scope** - see [docs/user-stories/](docs/user-stories/)
5. **Developer is C# → Rust learner** - frame all explanations in .NET terms
6. **Docker stack is ready** - `make docker-up` starts observability infrastructure
7. **Follow TDD approach** - write tests before implementation (see testing-guidelines.md)
8. **Communication tone is crucial** - avoid flattery, use engineering language (see guidelines.md)

**Implementation Priority:**
1. Core → Application → Infrastructure → UI (dependency order)
2. Test XOR problem convergence before moving to visualization
3. Integrate observability early (tracing setup in first implementation phase)

---

**Last Known State:** Project scaffolded, documentation complete, awaiting first Rust code implementation.
