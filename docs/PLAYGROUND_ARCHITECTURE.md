# Neural Network Playground - Architecture Design

## Overview

Interactive visualization tool for understanding neural network training, inspired by TensorFlow Playground.

## Component Structure (Clean Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│                     Presentation Layer                       │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐  │
│  │   Dataset    │  │   Decision   │  │    Network      │  │
│  │ Visualization│  │   Boundary   │  │   Topology      │  │
│  │   Canvas     │  │    Canvas    │  │    Diagram      │  │
│  └──────────────┘  └──────────────┘  └─────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Control Panel UI                          │ │
│  │  [Dataset] [Network] [Features] [Hyperparameters]     │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓ Messages
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  ┌──────────────────┐  ┌──────────────────┐               │
│  │  Playground      │  │   Training       │               │
│  │  Orchestrator    │  │   Service        │               │
│  └──────────────────┘  └──────────────────┘               │
│  ┌──────────────────┐  ┌──────────────────┐               │
│  │  Network         │  │   Dataset        │               │
│  │  Builder         │  │   Generator      │               │
│  └──────────────────┘  └──────────────────┘               │
└─────────────────────────────────────────────────────────────┘
                            ↓ Uses
┌─────────────────────────────────────────────────────────────┐
│                        Core Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │   Module     │  │  Activation  │  │     Loss     │    │
│  │   (Layer)    │  │   Functions  │  │   Functions  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. User Interaction Flow

```
User clicks "Circle Dataset"
    ↓
UI Message: DatasetSelected(Circle)
    ↓
Application: DatasetGenerator::generate_circle()
    ↓
Returns: Vec<DataPoint> with (x, y, label)
    ↓
UI updates: Dataset visualization canvas
```

### 2. Training Flow

```
User clicks "Play"
    ↓
UI Message: StartTraining
    ↓
Application: PlaygroundOrchestrator::start_training()
    ↓
Spawns: tokio::task with TrainingService
    ↓
Loop:
    ┌─ Train one batch
    ├─ Compute metrics
    ├─ Send Message::TrainingProgress
    └─ Yield to UI thread
    ↓
UI updates:
    ├─ Decision boundary (recompute grid)
    ├─ Loss chart (add point)
    └─ Network diagram (update weights)
```

### 3. Decision Boundary Computation

```
Timer tick (every 100ms during training)
    ↓
Generate 50×50 grid points
    ↓
For each grid point:
    ├─ Forward pass through network
    └─ Get prediction (0 or 1)
    ↓
Convert to color gradient (blue → orange)
    ↓
Render to Canvas
```

## Key Components

### 1. Presentation Layer

#### `PlaygroundApp` (Main ICED Application)

```rust
pub struct PlaygroundApp {
    // State
    dataset: PlaygroundDataset,
    network_config: NetworkConfig,
    training_state: TrainingState,
    hyperparameters: Hyperparameters,

    // Visualization Data
    decision_boundary: Option<DecisionBoundary>,  // 50×50 grid of predictions
    loss_history: Vec<(usize, f32)>,              // (epoch, loss)

    // Services
    orchestrator: PlaygroundOrchestrator,

    // Caches for Canvas
    dataset_cache: Cache,
    boundary_cache: Cache,
    network_cache: Cache,
}

pub enum Message {
    // Dataset
    DatasetSelected(DatasetType),
    NoiseChanged(f32),
    TrainTestRatioChanged(f32),

    // Network
    AddLayer,
    RemoveLayer,
    ChangeLayerSize { layer: usize, size: usize },
    ActivationChanged(ActivationType),

    // Features
    FeatureToggled(Feature),

    // Training
    StartTraining,
    PauseTraining,
    ResetTraining,
    TrainingProgress(TrainingUpdate),

    // Hyperparameters
    LearningRateChanged(f32),
    BatchSizeChanged(usize),
    RegularizationChanged(f32),
}
```

#### Canvas Components

**Dataset Visualization Canvas**
```rust
pub struct DatasetCanvas<'a> {
    points: &'a [DataPoint],      // Training points
    test_points: &'a [DataPoint], // Test points
    cache: &'a Cache,
}

impl<'a> Program<Message> for DatasetCanvas<'a> {
    fn draw(...) {
        // Blue circles for class 0
        // Orange circles for class 1
        // Smaller grey circles for test set
    }
}
```

**Decision Boundary Canvas**
```rust
pub struct DecisionBoundaryCanvas<'a> {
    boundary_grid: &'a Array2<f32>,  // 50×50 predictions
    dataset_points: &'a [DataPoint],
    cache: &'a Cache,
}

impl<'a> Program<Message> for DecisionBoundaryCanvas<'a> {
    fn draw(...) {
        // Background: color gradient based on predictions
        // Overlay: dataset points
        // Performance: use cached background, redraw only when needed
    }
}
```

**Network Topology Canvas**
```rust
pub struct NetworkTopologyCanvas<'a> {
    layers: &'a [LayerInfo],
    weights: &'a [Array2<f32>],
    activations: &'a [Array1<f32>],
    cache: &'a Cache,
}

struct LayerInfo {
    neurons: usize,
    activation: ActivationType,
}

impl<'a> Program<Message> for NetworkTopologyCanvas<'a> {
    fn draw(...) {
        // Neurons: circles sized by activation value
        // Connections: lines with thickness = abs(weight)
        // Colors: positive weights (orange), negative (blue)
    }
}
```

### 2. Application Layer

#### `PlaygroundOrchestrator`

```rust
pub struct PlaygroundOrchestrator {
    model: Option<Sequential>,
    dataset: Option<GeneratedDataset>,
    config: PlaygroundConfig,
    training_handle: Option<tokio::task::JoinHandle<()>>,
}

impl PlaygroundOrchestrator {
    pub fn build_network(&mut self, config: &NetworkConfig) -> anyhow::Result<()> {
        // Dynamically construct network based on UI config
        let mut model = Sequential::new();

        // Input layer: depends on selected features (2-7 inputs)
        let input_size = self.get_active_feature_count();

        // Hidden layers: user-configured
        for layer_size in &config.hidden_layers {
            model.add(Box::new(DenseLayer::new(prev_size, *layer_size)));
            model.add(Box::new(ActivationLayer::new(config.activation)));
        }

        // Output layer: binary classification
        model.add(Box::new(DenseLayer::new(prev_size, 2)));
        model.add(Box::new(ActivationLayer::new(Softmax)));

        self.model = Some(model);
        Ok(())
    }

    pub async fn start_training(&mut self, sender: tokio::sync::mpsc::Sender<Message>) {
        let model = self.model.clone();
        let dataset = self.dataset.clone();

        let handle = tokio::spawn(async move {
            let mut trainer = PlaygroundTrainer::new(config);
            trainer.train_with_callbacks(model, dataset, |update| {
                sender.send(Message::TrainingProgress(update)).await.ok();
            }).await;
        });

        self.training_handle = Some(handle);
    }

    pub fn compute_decision_boundary(&mut self) -> Array2<f32> {
        // Generate 50×50 grid
        let grid_size = 50;
        let mut boundary = Array2::zeros((grid_size, grid_size));

        for i in 0..grid_size {
            for j in 0..grid_size {
                let x = -1.0 + 2.0 * (i as f32 / grid_size as f32);
                let y = -1.0 + 2.0 * (j as f32 / grid_size as f32);

                // Apply feature engineering
                let input = self.apply_features(x, y);

                // Forward pass
                let output = self.model.forward(&input);

                // Store prediction (class 1 probability)
                boundary[[i, j]] = output[[0, 1]];
            }
        }

        boundary
    }

    fn apply_features(&self, x: f32, y: f32) -> Array2<f32> {
        let mut features = Vec::new();

        if self.config.features.x { features.push(x); }
        if self.config.features.y { features.push(y); }
        if self.config.features.x_squared { features.push(x * x); }
        if self.config.features.y_squared { features.push(y * y); }
        if self.config.features.x_times_y { features.push(x * y); }
        if self.config.features.sin_x { features.push(x.sin()); }
        if self.config.features.sin_y { features.push(y.sin()); }

        Array2::from_shape_vec((1, features.len()), features).unwrap()
    }
}
```

#### `DatasetGenerator`

```rust
pub struct DatasetGenerator;

pub struct GeneratedDataset {
    pub train_points: Vec<DataPoint>,
    pub test_points: Vec<DataPoint>,
}

pub struct DataPoint {
    pub x: f32,
    pub y: f32,
    pub label: usize,  // 0 or 1
}

impl DatasetGenerator {
    pub fn generate(
        dataset_type: DatasetType,
        num_points: usize,
        noise: f32,
        train_ratio: f32,
    ) -> GeneratedDataset {
        let points = match dataset_type {
            DatasetType::Circle => Self::generate_circle(num_points, noise),
            DatasetType::XOR => Self::generate_xor(num_points, noise),
            DatasetType::Gaussian => Self::generate_gaussian(num_points, noise),
            DatasetType::Spiral => Self::generate_spiral(num_points, noise),
        };

        // Split train/test
        let split_idx = (num_points as f32 * train_ratio) as usize;
        GeneratedDataset {
            train_points: points[..split_idx].to_vec(),
            test_points: points[split_idx..].to_vec(),
        }
    }

    fn generate_circle(num_points: usize, noise: f32) -> Vec<DataPoint> {
        let mut rng = rand::thread_rng();
        let mut points = Vec::new();

        for _ in 0..num_points {
            let radius: f32 = rng.gen_range(0.0..1.0);
            let angle: f32 = rng.gen_range(0.0..2.0 * PI);

            let x = radius * angle.cos() + rng.gen_range(-noise..noise);
            let y = radius * angle.sin() + rng.gen_range(-noise..noise);

            // Label: inside circle (< 0.5 radius) = 0, outside = 1
            let label = if radius < 0.5 { 0 } else { 1 };

            points.push(DataPoint { x, y, label });
        }

        points
    }

    // Similar implementations for XOR, Gaussian, Spiral...
}
```

#### `PlaygroundTrainer`

```rust
pub struct PlaygroundTrainer {
    config: TrainerConfig,
}

pub struct TrainingUpdate {
    pub epoch: usize,
    pub train_loss: f32,
    pub test_loss: f32,
    pub train_accuracy: f32,
    pub test_accuracy: f32,
}

impl PlaygroundTrainer {
    pub async fn train_with_callbacks<F>(
        &mut self,
        model: &mut Sequential,
        dataset: &GeneratedDataset,
        mut callback: F,
    ) where
        F: FnMut(TrainingUpdate) + Send,
    {
        for epoch in 0..self.config.epochs {
            // Train one epoch
            self.train_epoch(model, &dataset.train_points)?;

            // Evaluate
            let train_loss = self.evaluate(model, &dataset.train_points);
            let test_loss = self.evaluate(model, &dataset.test_points);
            let train_acc = self.accuracy(model, &dataset.train_points);
            let test_acc = self.accuracy(model, &dataset.test_points);

            // Send update
            callback(TrainingUpdate {
                epoch,
                train_loss,
                test_loss,
                train_accuracy: train_acc,
                test_accuracy: test_acc,
            });

            // Yield to UI
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}
```

## Configuration Structures

```rust
#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub hidden_layers: Vec<usize>,  // e.g., [4, 2] = 2 layers with 4 and 2 neurons
    pub activation: ActivationType,
}

#[derive(Clone, Debug)]
pub struct Hyperparameters {
    pub learning_rate: f32,
    pub batch_size: usize,
    pub regularization: f32,
    pub regularization_type: RegularizationType,
}

#[derive(Clone, Debug)]
pub struct FeatureConfig {
    pub x: bool,
    pub y: bool,
    pub x_squared: bool,
    pub y_squared: bool,
    pub x_times_y: bool,
    pub sin_x: bool,
    pub sin_y: bool,
}

#[derive(Clone, Debug)]
pub enum DatasetType {
    Circle,
    XOR,
    Gaussian,
    Spiral,
}

#[derive(Clone, Debug)]
pub enum ActivationType {
    ReLU,
    Tanh,
    Sigmoid,
}

#[derive(Clone, Debug)]
pub enum RegularizationType {
    None,
    L1,
    L2,
}
```

## Performance Considerations

### Decision Boundary Computation

**Challenge**: 2500 forward passes (50×50 grid) per update

**Solutions**:
1. **Caching**: Only recompute when network weights change significantly
2. **Throttling**: Update at most 10 times per second
3. **Batching**: Process grid in batches of 100 points
4. **Async**: Compute in background task, don't block UI

```rust
// Efficient boundary computation
pub async fn compute_boundary_async(&self) -> Array2<f32> {
    const GRID_SIZE: usize = 50;
    const BATCH_SIZE: usize = 100;

    let grid_points = generate_grid_points(GRID_SIZE);
    let mut predictions = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

    for batch in grid_points.chunks(BATCH_SIZE) {
        // Batch forward pass
        let inputs = self.batch_apply_features(batch);
        let outputs = self.model.forward(&inputs);
        predictions.extend(outputs.column(1).iter());

        // Yield occasionally
        if predictions.len() % 500 == 0 {
            tokio::task::yield_now().await;
        }
    }

    Array2::from_shape_vec((GRID_SIZE, GRID_SIZE), predictions).unwrap()
}
```

### UI Update Strategy

**Problem**: Training produces updates faster than UI can render

**Solution**: Throttle updates

```rust
let mut last_ui_update = Instant::now();
const UI_UPDATE_INTERVAL: Duration = Duration::from_millis(100); // 10 FPS

loop {
    // Train batch
    train_batch(&mut model, batch);

    // Only update UI every 100ms
    if last_ui_update.elapsed() > UI_UPDATE_INTERVAL {
        send_ui_update(compute_metrics(&model));
        last_ui_update = Instant::now();
    }
}
```

## UI Control Panels

### Network Configuration Panel

The network panel provides comprehensive controls for building custom neural network architectures:

```rust
pub struct NetworkPanel {
    layers: Vec<LayerConfig>,
    advanced_mode: bool,
    global_activation: ActivationType,
    selected_preset: Option<PresetType>,
}

pub struct LayerConfig {
    neurons: usize,
    activation: ActivationType,  // Used only in advanced mode
}

impl NetworkPanel {
    pub fn view(&self) -> Element<Message> {
        column![
            // Header with layer count
            self.layer_count_header(),

            // Architecture presets
            self.preset_selector(),

            // Layer management buttons
            row![
                button("+ Add Layer")
                    .on_press(Message::AddLayer)
                    .style(if self.layers.len() < 6 { ButtonStyle::Primary } else { ButtonStyle::Disabled }),
                button("- Remove Layer")
                    .on_press(Message::RemoveLayer)
                    .style(if self.layers.len() > 0 { ButtonStyle::Secondary } else { ButtonStyle::Disabled }),
                button("Reset")
                    .on_press(Message::ResetNetwork)
                    .style(ButtonStyle::Danger),
            ].spacing(8),

            // Layer configuration list
            self.layer_list(),

            // Activation function controls
            self.activation_controls(),

            // Network statistics
            self.network_stats(),
        ]
        .spacing(16)
        .padding(16)
        .into()
    }

    fn layer_count_header(&self) -> Element<Message> {
        let layer_text = if self.layers.len() == 1 {
            "1 hidden layer"
        } else {
            format!("{} hidden layers", self.layers.len())
        };

        let architecture = format!(
            "[{}]",
            self.layers.iter()
                .map(|l| l.neurons.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        row![
            text(layer_text).size(16),
            badge(&architecture).style(BadgeStyle::Info),
        ]
        .spacing(8)
        .into()
    }

    fn preset_selector(&self) -> Element<Message> {
        column![
            text("Architecture Presets").size(14),
            row![
                button("Shallow [4]")
                    .on_press(Message::LoadPreset(PresetType::Shallow))
                    .style(self.preset_style(PresetType::Shallow)),
                button("Deep [4,4,4]")
                    .on_press(Message::LoadPreset(PresetType::Deep))
                    .style(self.preset_style(PresetType::Deep)),
                button("Pyramid [6,4,2]")       
                    .on_press(Message::LoadPreset(PresetType::Pyramid))
                    .style(self.preset_style(PresetType::Pyramid)),
                button("Wide [8,8]")
                    .on_press(Message::LoadPreset(PresetType::Wide))
                    .style(self.preset_style(PresetType::Wide)),
            ].spacing(4),
        ]
        .spacing(8)
        .into()
    }

    fn layer_list(&self) -> Element<Message> {
        let layers = self.layers.iter().enumerate().map(|(idx, layer)| {
            self.layer_row(idx, layer)
        }).collect();

        column(layers).spacing(8).into()
    }

    fn layer_row(&self, idx: usize, layer: &LayerConfig) -> Element<Message> {
        let mut controls = vec![
            text(format!("Layer {}", idx + 1)).size(14).into(),
            slider(1..=8, layer.neurons, move |n| {
                Message::ChangeLayerSize { layer: idx, size: n }
            }).into(),
            text(format!("{} neurons", layer.neurons)).size(12).into(),
        ];

        // Add per-layer activation in advanced mode
        if self.advanced_mode {
            controls.push(
                pick_list(
                    &ActivationType::ALL[..],
                    Some(layer.activation),
                    move |act| Message::ChangeLayerActivation { layer: idx, activation: act }
                ).into()
            );
        }

        row(controls).spacing(8).into()
    }

    fn activation_controls(&self) -> Element<Message> {
        column![
            row![
                text("Activation Function").size(14),
                checkbox("Advanced mode", self.advanced_mode)
                    .on_toggle(Message::ToggleAdvancedMode),
            ].spacing(8),

            // Global activation selector (only if not in advanced mode)
            if !self.advanced_mode {
                pick_list(
                    &ActivationType::ALL[..],
                    Some(self.global_activation),
                    Message::GlobalActivationChanged
                )
            } else {
                text("Configure per-layer above").size(12).style(Color::from_rgb8(128, 128, 128))
            },

            // Output layer info (non-editable)
            text("Output: Softmax (fixed for classification)")
                .size(12)
                .style(Color::from_rgb8(128, 128, 128)),
        ]
        .spacing(8)
        .into()
    }

    fn network_stats(&self) -> Element<Message> {
        let input_size = self.get_input_size();  // Based on selected features
        let total_params = self.calculate_total_parameters(input_size);
        let memory_kb = (total_params * 4) / 1024;  // f32 = 4 bytes

        let mut warnings = Vec::new();
        if total_params > 100_000 {
            warnings.push("⚠️ May train slowly");
        }
        if self.layers.len() > 4 {
            warnings.push("⚠️ Very deep - may be hard to train");
        }
        if self.layers.iter().any(|l| l.neurons < 3) {
            warnings.push("⚠️ Small layers may underfit");
        }

        column![
            text("Network Statistics").size(14),
            text(format!("Input size: {} (based on features)", input_size)).size(12),
            text(format!("Output size: 2 (binary classification)")).size(12),
            text(format!("Total parameters: {}", total_params)).size(12),
            text(format!("Memory: ~{} KB", memory_kb)).size(12),

            // Warnings
            if !warnings.is_empty() {
                column(
                    warnings.iter().map(|w| {
                        text(*w).size(12).style(Color::from_rgb8(255, 140, 0))
                    }).collect()
                ).spacing(4)
            } else {
                text("✓ Configuration looks good").size(12).style(Color::from_rgb8(0, 200, 0))
            }
        ]
        .spacing(4)
        .padding(8)
        .style(ContainerStyle::Box)
        .into()
    }

    fn calculate_total_parameters(&self, input_size: usize) -> usize {
        let mut params = 0;
        let mut prev_size = input_size;

        // Hidden layers
        for layer in &self.layers {
            params += prev_size * layer.neurons + layer.neurons;  // weights + biases
            prev_size = layer.neurons;
        }

        // Output layer
        params += prev_size * 2 + 2;

        params
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresetType {
    Shallow,    // [4]
    Deep,       // [4, 4, 4]
    Pyramid,    // [6, 4, 2]
    Wide,       // [8, 8]
}

impl PresetType {
    pub fn layers(&self) -> Vec<usize> {
        match self {
            PresetType::Shallow => vec![4],
            PresetType::Deep => vec![4, 4, 4],
            PresetType::Pyramid => vec![6, 4, 2],
            PresetType::Wide => vec![8, 8],
        }
    }
}
```

### Message Handling for Network Configuration

```rust
pub enum Message {
    // Layer management
    AddLayer,
    RemoveLayer,
    ResetNetwork,
    ChangeLayerSize { layer: usize, size: usize },

    // Activation
    GlobalActivationChanged(ActivationType),
    ChangeLayerActivation { layer: usize, activation: ActivationType },
    ToggleAdvancedMode(bool),

    // Presets
    LoadPreset(PresetType),

    // ... other messages
}

impl PlaygroundApp {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddLayer => {
                if self.network_panel.layers.len() < 6 {
                    self.network_panel.layers.push(LayerConfig {
                        neurons: 4,
                        activation: self.network_panel.global_activation,
                    });
                    self.rebuild_network();
                }
            }

            Message::RemoveLayer => {
                if !self.network_panel.layers.is_empty() {
                    self.network_panel.layers.pop();
                    self.rebuild_network();
                }
            }

            Message::ResetNetwork => {
                self.network_panel.layers = vec![
                    LayerConfig { neurons: 4, activation: ActivationType::ReLU },
                    LayerConfig { neurons: 2, activation: ActivationType::ReLU },
                ];
                self.network_panel.global_activation = ActivationType::ReLU;
                self.network_panel.advanced_mode = false;
                self.rebuild_network();
            }

            Message::ChangeLayerSize { layer, size } => {
                if let Some(layer_config) = self.network_panel.layers.get_mut(layer) {
                    layer_config.neurons = size;
                    self.rebuild_network();
                }
            }

            Message::GlobalActivationChanged(activation) => {
                self.network_panel.global_activation = activation;
                if !self.network_panel.advanced_mode {
                    // Apply to all layers
                    for layer in &mut self.network_panel.layers {
                        layer.activation = activation;
                    }
                    self.rebuild_network();
                }
            }

            Message::ChangeLayerActivation { layer, activation } => {
                if let Some(layer_config) = self.network_panel.layers.get_mut(layer) {
                    layer_config.activation = activation;
                    self.rebuild_network();
                }
            }

            Message::ToggleAdvancedMode(enabled) => {
                self.network_panel.advanced_mode = enabled;
                if !enabled {
                    // Sync all layers to global activation
                    let global = self.network_panel.global_activation;
                    for layer in &mut self.network_panel.layers {
                        layer.activation = global;
                    }
                }
            }

            Message::LoadPreset(preset) => {
                let layer_sizes = preset.layers();
                self.network_panel.layers = layer_sizes.iter().map(|&size| {
                    LayerConfig {
                        neurons: size,
                        activation: self.network_panel.global_activation,
                    }
                }).collect();
                self.network_panel.selected_preset = Some(preset);
                self.network_panel.advanced_mode = false;
                self.rebuild_network();
            }

            // ... other message handlers
        }

        Command::none()
    }

    fn rebuild_network(&mut self) {
        // Stop current training
        self.stop_training();

        // Build new network based on configuration
        let input_size = self.get_active_feature_count();
        let network_config = NetworkConfig {
            hidden_layers: self.network_panel.layers.clone(),
        };

        match self.orchestrator.build_network(input_size, &network_config) {
            Ok(()) => {
                tracing::info!(
                    architecture = ?network_config.hidden_layers,
                    "Network rebuilt successfully"
                );

                // Clear decision boundary cache
                self.decision_boundary = None;
                self.boundary_cache.clear();

                // Clear network topology cache
                self.network_cache.clear();
            }
            Err(e) => {
                tracing::error!(error = ?e, "Failed to rebuild network");
            }
        }
    }
}
```

## File Structure

```
application/src/
  playground/
    mod.rs
    orchestrator.rs       # Main coordination
    trainer.rs            # Training loop with callbacks
    builder.rs            # Dynamic network construction
    datasets/
      mod.rs
      circle.rs
      xor.rs
      gaussian.rs
      spiral.rs

presentation/src/
  playground/
    mod.rs
    app.rs                # Main ICED app
    message.rs            # Message enum
    canvas/
      dataset.rs          # Dataset visualization
      boundary.rs         # Decision boundary
      network.rs          # Network topology
    controls/
      dataset_panel.rs    # Dataset selection UI
      network_panel.rs    # Layer configuration UI (detailed above)
      feature_panel.rs    # Feature toggles
      hyperparameter_panel.rs  # Sliders for LR, etc.
```

## Next Steps

1. Implement dataset generators (Circle, XOR first)
2. Build dynamic network builder
3. Create basic training loop with callbacks
4. Implement dataset visualization canvas
5. Implement decision boundary canvas
6. Add control panels
7. Implement network topology diagram
8. Add charts for loss/accuracy

## References

- TensorFlow Playground source: https://github.com/tensorflow/playground
- ICED Canvas examples
- Distill.pub articles on visualization
