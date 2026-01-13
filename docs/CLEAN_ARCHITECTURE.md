# Clean Architecture Implementation

## Overview

This project follows Clean Architecture principles to ensure maintainability, testability, and clear separation of concerns. The architecture enforces dependency rules where inner layers know nothing about outer layers.

## Layer Structure

```
┌─────────────────────────────────────────────────┐
│              Presentation Layer                  │
│    (UI, ICED widgets, User interaction)         │
│          presentation/                           │
└──────────────┬──────────────────────────────────┘
               │ depends on ↓
┌──────────────▼──────────────────────────────────┐
│           Application Layer                      │
│  (Use Cases, Services, Business Logic)          │
│          application/                            │
│  - TrainingService, PredictionService            │
│  - MNIST Loading, Training Orchestration        │
└──────────────┬──────────────────────────────────┘
               │ depends on ↓
┌──────────────▼──────────────────────────────────┐
│              Core Layer                          │
│     (Domain Model, Pure Mathematics)             │
│            core/                                 │
│  - Module trait, Sequential, Layers              │
│  - Activation functions, Loss functions          │
└──────────────┬──────────────────────────────────┘
               │
               ├──────────────┐
               │              │
┌──────────────▼──┐   ┌───────▼──────────┐
│  Infrastructure  │   │    (Future)      │
│  (Persistence,   │   │  - TimescaleDB   │
│   Serialization) │   │  - Model I/O     │
│ infrastructure/  │   │  - Telemetry     │
└──────────────────┘   └──────────────────┘
```

## Dependency Rule

**Inner layers MUST NOT depend on outer layers.**

- ✅ Presentation → Application → Core
- ✅ Infrastructure → Application → Core
- ❌ Core → Application (FORBIDDEN)
- ❌ Application → Presentation (FORBIDDEN)

## Layer Responsibilities

### Core (`core/`)

**Pure mathematics and domain model. No I/O, no external dependencies.**

```rust
// core/src/module.rs
pub trait Module: Send {
    fn forward(&mut self, input: &Array2<f32>) -> Array2<f32>;
    fn backward(&mut self, grad_output: &Array2<f32>) -> Array2<f32>;
    fn update_parameters(&mut self, learning_rate: f32);
}
```

**Characteristics:**
- No `tokio`, no `iced`, no `serde` (unless for internal math)
- Pure functions, deterministic
- Unit testable without mocks
- Represents mathematical truth

**C# Analogy:** Domain entities in DDD - POCOs with business rules

---

### Application (`application/`)

**Use cases and business logic orchestration.**

```rust
// application/src/services/prediction_service.rs
pub trait PredictionService: Send {
    fn predict(&mut self, raw_pixels: &[u8; 28 * 28]) -> PredictionResult;
}

// application/src/services/training_service.rs
pub struct MnistTrainingService {
    model: Arc<Mutex<Option<Sequential>>>,
    dataset: MnistDataset,
}

impl MnistTrainingService {
    pub fn train(&self, config: TrainerConfig) -> Result<TrainingResult> {
        // Orchestrates training using core::Sequential
        // Returns metrics, not UI state
    }
}
```

**Characteristics:**
- Defines **traits** (interfaces) for services
- Implements use cases: "Train Model", "Predict Digit"
- Domain logic: normalization (`u8` → `f32`), prediction extraction
- No UI code, no rendering, no user input handling
- Can have infrastructure dependencies (dataset loading)

**C# Analogy:** Application Services in DDD/Clean Architecture

---

### Presentation (`presentation/`)

**UI layer - adapts application services to user interface.**

```rust
// presentation/src/painter/app.rs
pub struct MnistPainter {
    canvas: [u8; 28 * 28],              // UI state
    service: MnistTrainingService,       // Application service
    prediction: Option<PredictionResult>, // Display state
}

impl MnistPainter {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PredictDigit => {
                // Delegates to application service
                let prediction = self.service.predict(&self.canvas);
                self.prediction = Some(prediction);
            }
            Message::TrainModel => {
                // Spawns async task, calls service
                Task::perform(
                    train_model_async(self.service.clone(), config),
                    Message::TrainingComplete
                )
            }
        }
    }
}
```

**Characteristics:**
- ICED widgets, canvas rendering, mouse events
- Delegates domain logic to `application` services
- Owns UI state (canvas pixels, input fields)
- Does NOT own domain model (`Sequential`)
- Triggers use cases via service methods

**C# Analogy:** WPF ViewModels, ASP.NET Controllers, Blazor Components

---

### Infrastructure (`infrastructure/`)

**External systems: persistence, telemetry, serialization.**

```rust
// infrastructure/src/persistence/model_repository.rs (future)
pub struct FileModelRepository {
    path: PathBuf,
}

impl ModelRepository for FileModelRepository {
    fn save(&self, model: &Sequential) -> Result<()> {
        // serde serialization to disk
    }

    fn load(&self) -> Result<Sequential> {
        // Load from file
    }
}
```

**Characteristics:**
- Implements traits defined in `application`
- File I/O, database connections, HTTP clients
- Swappable implementations (InMemoryRepo, FileRepo, S3Repo)

**C# Analogy:** EF Core DbContext, Repository implementations

---

## Key Architectural Decisions

### 1. UI Does Not Own the Model

**Before (Infrastructure violation):**
```rust
// ❌ BAD: UI owns Sequential
pub struct MnistPainter {
    model: Option<Sequential>,  // Owned by UI!
}
```

**After (Clean Architecture):**
```rust
// ✅ GOOD: Service owns model, UI delegates
pub struct MnistPainter {
    service: MnistTrainingService,  // Service owns model via Arc<Mutex<>>
}

impl MnistPainter {
    fn predict(&mut self) {
        let result = self.service.predict(&self.canvas);  // Delegate
    }
}
```

**Benefits:**
- Model can be accessed by telemetry without UI
- Can add REST API without changing training logic
- Testing UI doesn't require real models

---

### 2. Domain Logic in Application, Not UI

**Before:**
```rust
// ❌ BAD: Normalization in UI
impl MnistPainter {
    fn canvas_to_network_input(&self) -> Array2<f32> {
        let normalized: Vec<f32> = self.canvas
            .iter()
            .map(|&pixel| pixel as f32 / 255.0)  // Domain rule!
            .collect();
        Array2::from_shape_vec((1, 784), normalized).unwrap()
    }
}
```

**After:**
```rust
// ✅ GOOD: Normalization in application service
// application/src/services/prediction_service.rs
pub fn normalize_mnist_input(raw_pixels: &[u8; 28 * 28]) -> Array2<f32> {
    let normalized: Vec<f32> = raw_pixels
        .iter()
        .map(|&pixel| f32::from(pixel) / 255.0)
        .collect();
    Array2::from_shape_vec((1, 784), normalized).expect("Must be 784")
}

// UI just passes raw bytes
impl MnistPainter {
    fn predict(&mut self) {
        self.service.predict(&self.canvas);  // Service handles normalization
    }
}
```

**Benefits:**
- If normalization changes (standardization), UI unaffected
- Can test normalization without ICED
- Reusable across CLI, REST API, GUI

---

### 3. Async Training Without UI Coupling

**Before:**
```rust
// ❌ BAD: Training logic in UI update
Message::TrainModel => {
    let mut model = Sequential::new();
    model.add(...);  // Building in UI!
    let trainer = Trainer::new(...);
    trainer.train(&mut model, ...)?;  // Blocking UI!
    self.model = Some(model);
}
```

**After:**
```rust
// ✅ GOOD: Service handles training
Message::TrainModel => {
    Task::perform(
        train_model_async(self.service.clone(), config),
        Message::TrainingComplete
    )
}

async fn train_model_async(service: MnistTrainingService, config: TrainerConfig) -> TrainingResult {
    tokio::task::spawn_blocking(move || {
        service.train(config).expect("Training failed")
    }).await.unwrap()
}
```

**Benefits:**
- Non-blocking UI during training
- Service can log to TimescaleDB during training
- Can trigger training from CLI without duplicating logic

---

## Testing Strategy

### Core Layer (Pure Unit Tests)
```rust
#[test]
fn given__relu_negative_input__when__activate__then__returns_zero() {
    let relu = ReLU;
    let input = array![[-5.0]];
    let result = relu.activate(&input);
    assert_eq!(result[[0, 0]], 0.0);
}
```

### Application Layer (Service Tests)
```rust
#[test]
fn given__trained_service__when__predict__then__returns_correct_digit() {
    let dataset = load_mnist_subset(100, 10).unwrap();
    let mut service = MnistTrainingService::new(dataset);
    service.train(TrainerConfig::default()).unwrap();

    let mnist_seven = [/* actual MNIST image bytes */];
    let result = service.predict(&mnist_seven);

    assert_eq!(result.digit, 7);
}
```

### Presentation Layer (Mock Services)
```rust
struct MockPredictionService {
    predictions: Vec<PredictionResult>,
}

impl PredictionService for MockPredictionService {
    fn predict(&mut self, _pixels: &[u8; 28 * 28]) -> PredictionResult {
        self.predictions.pop().unwrap()
    }
}

#[test]
fn given__canvas_painted__when__predict_message__then__displays_result() {
    let mock = MockPredictionService { /* ... */ };
    let mut painter = MnistPainter::with_service(mock);

    painter.update(Message::PredictDigit);

    assert!(painter.prediction.is_some());
}
```

---

## Running the Application

```bash
# Run from presentation layer
cargo run --package nn-presentation --example mnist_painter

# What happens (Clean Architecture flow):
# 1. Presentation loads dataset via application::load_mnist()
# 2. User clicks "Train" → Presentation calls service.train()
# 3. Application orchestrates core::Sequential training
# 4. User draws digit → Presentation calls service.predict()
# 5. Application normalizes input, calls core::Module::forward()
```

---

## Future Extensibility

### Adding REST API (No Changes to Core/Application)
```rust
// api/src/main.rs
#[post("/predict")]
async fn predict(
    image: Json<Vec<u8>>,
    service: Data<MnistTrainingService>
) -> Json<PredictionResult> {
    let pixels: [u8; 784] = image.try_into().unwrap();
    Json(service.predict(&pixels))
}
```

### Adding Telemetry (Infrastructure Layer)
```rust
// infrastructure/src/telemetry/training_observer.rs
pub struct TimescaleObserver {
    db: PgPool,
}

impl TrainingObserver for TimescaleObserver {
    fn on_epoch_complete(&self, epoch: usize, metrics: &EpochMetrics) {
        sqlx::query!("INSERT INTO training_metrics ...")
            .execute(&self.db)
            .await
            .unwrap();
    }
}
```

---

## Violation Detection

**How to spot violations:**

1. **Import check**: If `core` imports `iced` or `tokio` → VIOLATION
2. **Trait location**: If `PredictionService` is in `presentation` → VIOLATION
3. **Ownership**: If UI struct owns `Sequential` directly → VIOLATION
4. **Business logic**: If normalization is in `painter/app.rs` → VIOLATION

---

## C# Developer Comparison

| Rust Layer | C# Equivalent |
|------------|---------------|
| `core/` | Domain Layer (DDD entities) |
| `application/` | Application Services, Use Cases |
| `presentation/` | ASP.NET Controllers, WPF ViewModels |
| `infrastructure/` | EF Core, Repositories, External APIs |

**Key Insight:** This is identical to Onion Architecture or Hexagonal Architecture in .NET.

---

## Summary

This architecture ensures:
- ✅ **Testability**: Each layer tested independently
- ✅ **Maintainability**: Changes localized to one layer
- ✅ **Flexibility**: Swap UI (CLI, Web, Desktop) without changing core
- ✅ **Scalability**: Add features (telemetry, persistence) without modifying existing layers

The model NEVER leaks into the presentation layer. The UI is a thin adapter over application services.
