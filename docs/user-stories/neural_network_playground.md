# Neural Network Playground (Interactive Visualization)

## User Story

As a learner, I need an interactive neural network playground so that I can visualize how different network configurations, hyperparameters, and datasets affect learning in real-time.

## Problem Statement

Understanding how neural networks learn is difficult without visual feedback. The TensorFlow Playground (https://playground.tensorflow.org) demonstrates that interactive visualization makes concepts like:
- Hidden layer effects
- Activation function impact
- Learning rate tuning
- Dataset complexity
- Decision boundaries

...much more intuitive. Building a Rust/ICED equivalent provides:
- Deeper understanding through implementation
- Native performance for real-time updates
- Clean architecture demonstration
- Foundation for more complex visualizations

## Acceptance Criteria

### Core Visualization
- [ ] Given a 2D dataset, when the network trains, then the decision boundary is visualized in real-time
- [ ] Given different network architectures, when selected, then the network topology diagram updates
- [ ] Given training progress, when epochs complete, then loss and accuracy metrics update on charts
- [ ] Given neuron activations, when hovering over neurons, then activation values are displayed

### Network Configuration Controls

#### Layer Management
- [ ] **Add Layer Button**: Add a new hidden layer to the network (max 6 layers)
- [ ] **Remove Layer Button**: Remove the last hidden layer from the network (min 0 layers)
- [ ] **Layer Counter Display**: Shows current number of hidden layers (e.g., "3 hidden layers")
- [ ] **Reset to Default**: One-click reset to default architecture (e.g., [4, 2])

#### Per-Layer Configuration
Each hidden layer should have:
- [ ] **Neuron Count Slider**: Adjust neurons from 1 to 8 for each layer independently
- [ ] **Neuron Count Display**: Shows current neuron count (e.g., "Layer 1: 4 neurons")
- [ ] **Visual Indicator**: Current layer configuration displayed as badge (e.g., "[4, 2, 3]")

#### Activation Function Selection
- [ ] **Global Activation Dropdown**: Apply activation to all hidden layers
  - Options: ReLU, Tanh, Sigmoid, Linear
  - Default: ReLU
- [ ] **Per-Layer Activation** (Advanced): Different activation per layer
  - Toggle "Advanced mode" to enable
  - Each layer gets its own activation selector
- [ ] **Output Activation**: Fixed to Softmax (non-configurable for classification)

#### Network Architecture Presets
- [ ] **Preset Selector**: Quick-start architectures
  - "Shallow" - [4] single hidden layer
  - "Deep" - [4, 4, 4] three equal layers
  - "Pyramid" - [6, 4, 2] decreasing layer sizes
  - "Wide" - [8, 8] two wide layers
  - "Custom" - user-defined configuration

#### Network Topology Visualization
- [ ] **Dynamic Diagram**: Shows all layers (input → hidden → output)
- [ ] **Layer Labels**: Each layer labeled with size and activation
- [ ] **Connection Lines**: Lines between neurons colored by weight (blue=negative, orange=positive)
- [ ] **Line Thickness**: Proportional to absolute weight value
- [ ] **Neuron Size**: Proportional to activation value during training
- [ ] **Hover Tooltips**: Show exact neuron values and weights on hover

#### Configuration Validation
- [ ] **Input Layer Auto-Update**: Automatically adjusts based on selected features (2-7 inputs)
- [ ] **Output Layer Fixed**: Always 2 neurons for binary classification
- [ ] **Parameter Count Display**: Shows total trainable parameters
- [ ] **Memory Estimate**: Shows approximate memory usage
- [ ] **Warning Indicators**:
  - Too many parameters (>100k) - "May train slowly"
  - Too few neurons - "May underfit complex patterns"
  - Very deep (>4 layers) - "May be hard to train"

### Training Controls
- [ ] User can start/pause/reset training
- [ ] User can adjust learning rate (0.0001-10) with slider
- [ ] User can adjust batch size (1, 10, 30)
- [ ] User can adjust regularization rate (0-10)
- [ ] Training speed control (slow/medium/fast)

### Feature Engineering (Input Features)
- [ ] User can enable/disable input features: X, Y, X², Y², X×Y, sin(X), sin(Y)
- [ ] Feature selection updates network input layer
- [ ] Feature visualization shows which features are active

### Real-time Metrics
- [ ] Training loss displayed and updated per epoch
- [ ] Test loss displayed and updated per epoch
- [ ] Epoch counter increments during training
- [ ] Training time elapsed displayed

### Visualization Quality
- [ ] Decision boundary uses color gradient (blue to orange)
- [ ] Grid resolution is 50×50 for smooth visualization
- [ ] Neuron weights visualized as line thickness
- [ ] Neuron activations visualized as color intensity

## Technical Context

### Architecture Components

**Data Generation** (`application/src/datasets/playground.rs`):
- Generate 2D synthetic datasets (circle, XOR, Gaussian, spiral)
- Add configurable noise
- Split train/test sets

**Network Builder** (`application/src/playground/builder.rs`):
- Dynamically construct networks based on UI config
- Handle variable input features (2-7 dimensions)
- Support variable hidden layers (1-6 layers with 1-8 neurons each)

**Visualization Engine** (`presentation/src/playground/viz.rs`):
- Decision boundary rendering using Canvas
- Network topology diagram
- Real-time metric charts

**State Management** (`presentation/src/playground/state.rs`):
- Training state (running/paused/stopped)
- Network configuration
- Dataset configuration
- UI state (selected features, hyperparameters)

**Training Loop** (`application/src/playground/trainer.rs`):
- Async training with yield points for UI updates
- Per-batch or per-epoch callbacks
- Non-blocking training (tokio::task)

### ICED Components Needed

1. **Canvas Widgets** (3x):
   - Dataset visualization (left panel)
   - Decision boundary (center panel)
   - Network topology (right panel)

2. **Control Panels**:
   - Dataset selector and noise slider
   - Network configuration (add/remove layers)
   - Feature toggles (checkboxes)
   - Hyperparameter sliders
   - Play/pause/reset buttons

3. **Chart Widgets**:
   - Line chart for loss over time
   - Epoch counter display

### Key Technical Challenges

1. **Real-time Updates**: Training must yield to UI thread frequently
2. **Decision Boundary Computation**: Requires 2500 forward passes (50×50 grid)
3. **Performance**: Must maintain 30+ FPS during training
4. **Dynamic Network Construction**: Build networks with arbitrary architecture

### Rust Advantages Over JavaScript

- **Type Safety**: Network topology validated at compile time where possible
- **Performance**: Native rendering, faster training loops
- **Concurrency**: tokio for non-blocking training
- **Memory**: Deterministic cleanup, no GC pauses

**C# Comparison:**
- ICED GUI ≈ WPF/WinUI XAML
- Canvas rendering ≈ WriteableBitmap manipulation
- Async training ≈ Task-based async/await
- Dataset generation ≈ LINQ + Random

## Dependencies

- None (standalone learning tool)

## Estimated Complexity

**High**

**Reasoning:**
- Multiple complex visualizations (decision boundary, network diagram, charts)
- Real-time training with UI updates requires careful async design
- Dynamic network construction based on UI config
- Decision boundary computation is expensive (2500 forward passes)
- Requires 3-4 different synthetic dataset generators
- Extensive UI with many interactive controls

## Implementation Plan

### Phase 1: Dataset Generation (Low complexity)
1. Implement Circle, XOR, Gaussian, Spiral generators
2. Add noise injection
3. Train/test split

### Phase 2: Dynamic Network Builder (Medium complexity)
1. Feature engineering layer (input transformation)
2. Dynamic hidden layer construction
3. Support for variable neurons per layer

### Phase 3: Basic Training Visualization (Medium complexity)
1. Simple dataset visualization (scatter plot)
2. Training loop with callbacks
3. Loss/accuracy metrics display

### Phase 4: Decision Boundary Visualization (High complexity)
1. Grid point generation (50×50)
2. Batch prediction on grid
3. Color gradient rendering
4. Real-time updates during training

### Phase 5: Network Topology Diagram (Medium complexity)
1. Layer positioning
2. Connection line rendering
3. Weight visualization (line thickness)
4. Neuron activation colors

### Phase 6: Interactive Controls (Medium complexity)
1. All sliders and buttons
2. State management
3. Training control (play/pause/reset)

## Success Criteria

User can:
1. Select a dataset (e.g., Circle)
2. Configure a network (e.g., 2 hidden layers: [4, 4] neurons, ReLU)
3. Adjust learning rate to 0.03
4. Click "Play" and watch:
   - Decision boundary evolve in real-time
   - Loss decrease on chart
   - Network weights update (line thickness)
5. Pause training mid-way
6. Adjust network config and resume
7. Understand why certain architectures fail/succeed

## Future Enhancements

- Custom datasets (user draws points)
- Save/load network configurations
- Export trained model
- Confusion matrix visualization
- Gradient visualization (backprop flow)
- Support for 3D datasets
- Compare multiple networks side-by-side

## References

- Original TensorFlow Playground: https://playground.tensorflow.org
- Distill.pub visualization articles
- 3Blue1Brown neural network series
