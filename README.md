# Dalaware - Neural Network Framework from Scratch

A complete feed-forward neural network implementation built from first principles in Rust, using only `ndarray` for linear algebra. No PyTorch, no TensorFlow - just pure math and Rust.

[![Tests](https://img.shields.io/badge/tests-47%20passing-brightgreen)]()
[![Coverage](https://img.shields.io/badge/coverage-100%25%20core-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)]()

## 🎯 Features

- ✅ **Full Backpropagation** - Manual implementation of chain rule
- ✅ **Numerically Stable** - Softmax handles extreme values
- ✅ **Mini-Batch SGD** - Efficient training with batching
- ✅ **MNIST Ready** - Achieves >90% test accuracy
- ✅ **Clean Architecture** - Modular, extensible design
- ✅ **Comprehensive Tests** - 47 tests, 100% core coverage

## 🚀 Quick Start

### Run Tests
```bash
# All unit tests (fast)
cargo test --workspace

# XOR integration test
cargo test --package nn-application --test xor_test -- --nocapture

# MNIST tests (requires dataset download)
cargo test --package nn-application --test mnist_test -- --ignored --nocapture
```

### Train on MNIST
```rust
use nn_application::{Trainer, TrainerConfig, load_mnist};
use nn_core::{
    sequential::Sequential,
    layer::DenseLayer,
    activation_layer::ActivationLayer,
    activation::{ReLU, Softmax},
    loss::CrossEntropy,
};

// Load MNIST
let dataset = load_mnist()?;

// Build network: 784 → 128 → 64 → 10
let mut model = Sequential::new();
model.add(Box::new(DenseLayer::new(784, 128)));
model.add(Box::new(ActivationLayer::new(ReLU)));
model.add(Box::new(DenseLayer::new(128, 64)));
model.add(Box::new(ActivationLayer::new(ReLU)));
model.add(Box::new(DenseLayer::new(64, 10)));
model.add(Box::new(ActivationLayer::new(Softmax)));

// Train
let config = TrainerConfig {
    epochs: 15,
    learning_rate: 0.01,
    batch_size: 64,
};
let trainer = Trainer::new(config, CrossEntropy);
let metrics = trainer.train(&mut model, &dataset.train_images, &dataset.train_labels)?;

// Evaluate
let (loss, accuracy) = trainer.evaluate(&mut model, &dataset.test_images, &dataset.test_labels)?;
println!("Test Accuracy: {:.2}%", accuracy * 100.0);
```

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                       │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Trainer (Mini-Batch SGD)                          │    │
│  │  - Epoch management                                │    │
│  │  - Loss tracking                                   │    │
│  │  - Accuracy calculation                            │    │
│  └─────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  MNIST Loader                                      │    │
│  │  - Normalization                                   │    │
│  │  - One-hot encoding                                │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        Core Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ DenseLayer   │  │ Activations  │  │ Loss         │     │
│  │ - Forward    │  │ - Sigmoid    │  │ - MSE        │     │
│  │ - Backward   │  │ - ReLU       │  │ - CrossEntropy│    │
│  │ - Update     │  │ - Softmax    │  └──────────────┘     │
│  └──────────────┘  └──────────────┘                        │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │ Sequential   │  │ ActivationLayer                       │
│  │ - Compose    │  │ - Wrapper    │                        │
│  │ - Chain      │  └──────────────┘                        │
│  └──────────────┘                                           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                         ndarray (linear algebra)
```

## 📁 Project Structure

```
dalaware/
├── core/                      # Mathematical foundation
│   ├── activation.rs          # Sigmoid, ReLU, Softmax
│   ├── activation_layer.rs    # Activation → Module wrapper
│   ├── layer.rs               # DenseLayer implementation
│   ├── loss.rs                # MSE, Cross-Entropy
│   ├── module.rs              # Base trait
│   └── sequential.rs          # Layer composition
├── application/               # Training infrastructure
│   ├── trainer.rs             # Mini-batch SGD
│   ├── mnist_loader.rs        # Dataset preprocessing
│   └── tests/
│       ├── xor_test.rs        # XOR integration test
│       └── mnist_test.rs      # MNIST integration tests
├── presentation/              # UI layer (ICED)
│   ├── examples/
│   │   └── mnist_painter_fast.rs  # Interactive MNIST digit painter
│   └── src/
│       └── painter/           # Canvas widgets
├── infrastructure/            # External integrations
│   └── observability/         # Structured logging (tracing)
└── docs/                      # Documentation
    ├── user-stories/          # Implementation guides
    │   └── neural_network_playground.md  # Playground feature spec
    ├── PLAYGROUND_ARCHITECTURE.md         # Playground design
    ├── PERFORMANCE_OPTIMIZATION.md        # Training optimization guide
    ├── CLEAN_ARCHITECTURE.md              # Architecture principles
    ├── IMPLEMENTATION_SUMMARY.md
    └── FINAL_STATUS.md
```

## 🎓 What's Implemented

### Core Components
- **DenseLayer**: Fully-connected layer with He initialization
- **Activations**: Sigmoid, ReLU, Softmax (numerically stable)
- **Loss Functions**: MSE, Cross-Entropy (with gradient simplification)
- **Sequential**: Chain multiple layers together
- **ActivationLayer**: Wrapper to integrate activations

### Training
- **Trainer**: Mini-batch SGD with epoch management
- **Metrics**: Loss and accuracy tracking per epoch
- **Evaluation**: Forward-only pass for testing

### Data
- **MNIST Loader**: Automatic download, normalization, one-hot encoding
- **Batching**: Efficient mini-batch processing

## 📈 Performance

### MNIST Benchmark
- **Network**: 784 → 128 → 64 → 10
- **Parameters**: ~109,000
- **Training Time**: 2-5 minutes (15 epochs, CPU)
- **Test Accuracy**: >90% ✅

### XOR Benchmark
- **Network**: 2 → 4 → 1
- **Training Time**: <1 second (5000 epochs)
- **Convergence**: Loss < 0.15

## 🔬 Technical Details

### Softmax Numerical Stability
```rust
// Subtract row max to prevent overflow
let max_val = row.fold(f32::NEG_INFINITY, |a, b| a.max(b));
let exp_values: Vec<f32> = row.iter()
    .map(|&x| (x - max_val).exp())
    .collect();
```

### Cross-Entropy Gradient Simplification
```rust
// Softmax + CrossEntropy gradient = predictions - targets
// This is a mathematical simplification that avoids
// computing the full Jacobian matrix
fn gradient(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> Array2<f32> {
    predictions - targets
}
```

## 📚 Documentation

### Core Framework
- **[Implementation Summary](docs/IMPLEMENTATION_SUMMARY.md)** - Detailed breakdown
- **[Final Status](FINAL_STATUS.md)** - Complete overview with examples
- **[Softmax Guide](docs/user-stories/softmax-implementation.md)** - How to implement numerically stable Softmax
- **[Cross-Entropy Guide](docs/user-stories/cross-entropy-implementation.md)** - Why it pairs with Softmax

### Architecture & Design
- **[Clean Architecture](docs/CLEAN_ARCHITECTURE.md)** - Layered design principles
- **[Performance Optimization](docs/PERFORMANCE_OPTIMIZATION.md)** - Training speed improvements

### Planned Features
- **[Neural Network Playground](docs/PLAYGROUND_ARCHITECTURE.md)** - Interactive visualization design
- **[Playground User Story](docs/user-stories/neural_network_playground.md)** - Feature specifications

## 🧪 Testing

```bash
# Unit tests (47 tests)
cargo test --workspace

# Integration tests
cargo test --package nn-application --test xor_test
cargo test --package nn-application --test mnist_test -- --ignored

# With output
cargo test -- --nocapture
```

## 🎯 Goals Achieved

- [x] Build neural network from scratch
- [x] Implement backpropagation manually
- [x] Solve XOR problem (non-linear classification)
- [x] Train on MNIST (>90% accuracy)
- [x] Clean architecture (modular, testable)
- [x] Comprehensive tests (100% core coverage)
- [x] No high-level ML libraries

## 🎨 Interactive Visualization (Planned)

**Neural Network Playground** - An interactive GUI for understanding neural networks visually:
- Real-time decision boundary visualization
- Multiple datasets (Circle, XOR, Gaussian, Spiral)
- Interactive network configuration
- Feature engineering controls
- Live training metrics

See [Playground Architecture](docs/PLAYGROUND_ARCHITECTURE.md) and [User Story](docs/user-stories/neural_network_playground.md) for details.

## 🚀 Future Enhancements

- [ ] Adam optimizer
- [ ] Dropout regularization
- [ ] Batch normalization
- [ ] Convolutional layers
- [ ] Model serialization (save/load)
- [ ] Parallel batch processing (rayon)
- [ ] GPU acceleration (wgpu)

## 🤝 Contributing

This is an educational project demonstrating neural network fundamentals. Feel free to:
- Add new layer types
- Implement additional optimizers
- Improve performance
- Add more examples

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

Built with:
- `ndarray` - Rust's NumPy equivalent
- `mnist` - MNIST dataset loader
- Test-driven development
- Clean architecture principles

---

**Ready to train on MNIST?**
```bash
cargo test --package nn-application --test mnist_test test_mnist_full_training -- --ignored --nocapture
```
