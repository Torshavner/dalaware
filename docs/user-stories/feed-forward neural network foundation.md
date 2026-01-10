### User Story: Feed-Forward Neural Network Foundation

# Feed-Forward Neural Network Core Implementation

## User Story

As a developer, I need a modular Feed-Forward Neural Network engine so that I can train models to classify handwritten digits (MNIST dataset) without relying on high-level ML libraries.

## Problem Statement

Developing neural networks in Python often hides the underlying math and introduces performance bottlenecks. This project aims to bridge that gap by implementing these concepts from first principles in a high-performance, memory-safe language like Rust. The MNIST handwritten digit classification task serves as a practical benchmark to validate the implementation while being tractable for a from-scratch implementation.

## Acceptance Criteria

### Core Functionality

* [x] Implement a `DenseLayer` struct that stores weights and biases using `ndarray`
* [x] Implement forward propagation that computes pre-activations and applies activation functions
* [x] Implement backpropagation to calculate gradients for each layer
* [x] Support multiple activation functions (Sigmoid, ReLU)
* [ ] Implement Softmax activation function (required for multi-class classification)
* [ ] Implement `Sequential` container to compose multiple layers
* [ ] Create `ActivationLayer` wrapper to integrate activations into the Module trait

### Training and Optimization

* [ ] Implement a `Trainer` that orchestrates epochs, mini-batch processing, and gradient descent
* [ ] Calculate loss using Cross-Entropy for multi-class classification (MNIST: 10 classes)
* [ ] Support configurable learning rate and batch size
* [ ] Implement mini-batch SGD (Stochastic Gradient Descent)
* [ ] Add training metrics tracking (loss per epoch, accuracy)

### MNIST-Specific Requirements

* [ ] Load MNIST dataset (60,000 training images, 10,000 test images)
* [ ] Preprocess images: normalize pixel values to [0, 1] range
* [ ] Flatten 28x28 images into 784-dimensional vectors
* [ ] One-hot encode labels (10 classes: digits 0-9)
* [ ] Implement data batching for mini-batch SGD
* [ ] Achieve >90% test accuracy within reasonable training time (target: 10-20 epochs)

### Testing (Test-First Approach)

* [x] Write tests verifying that forward propagation on a 2x2 identity matrix returns expected values
* [x] Verify gradients calculated during backprop match numerical approximations
* [ ] Test XOR problem as sanity check (simple non-linear problem)
* [ ] Test Sequential container with multi-layer networks
* [ ] Test Softmax activation with known inputs/outputs
* [ ] Test Cross-Entropy loss calculation
* [ ] Integration test: Train on small MNIST subset (100 samples) and verify loss decreases
* [ ] Full MNIST test: Achieve >90% accuracy on test set

## Technical Context

### Architecture
* **Matrix Operations**: Use `ndarray` for all tensor math and `ndarray-rand` for parameter initialization
* **Network Architecture for MNIST**:
  - Input layer: 784 neurons (28x28 flattened image)
  - Hidden layer 1: 128 neurons with ReLU activation
  - Hidden layer 2: 64 neurons with ReLU activation
  - Output layer: 10 neurons with Softmax activation
* **Parallelism**: Use `Rayon` for parallelizing batch processing across CPU cores
* **Persistence**: Use `serde` to serialize/deserialize trained model weights to JSON or MessagePack
* **Dataset Loading**: Use `mnist` crate or implement custom IDX file parser

### Performance Considerations
* **Mini-batch SGD**: Process batches of 32-128 samples instead of full dataset (faster convergence)
* **Memory Efficiency**: Use borrowing (`&Array2<f32>`) to avoid unnecessary clones during forward/backward passes
* **Vectorization**: Leverage `ndarray`'s BLAS integration for optimized matrix operations
* **Expected Training Time**: ~2-5 minutes on modern CPU for 10-20 epochs

### Mathematical Implementation Details
* **Softmax**: `σ(z)_i = exp(z_i) / Σ(exp(z_j))` - numerically stable implementation required
* **Cross-Entropy Loss**: `L = -Σ(y_true * log(y_pred))` where y_true is one-hot encoded
* **Gradient for Softmax+CrossEntropy**: Simplified to `y_pred - y_true` (mathematical shortcut)
* **Weight Initialization**: He initialization for ReLU layers, Xavier for Sigmoid/Softmax

**C# Comparison:**

* `ndarray::Array2` ≈ `MathNet.Numerics.Matrix<double>`
* `Sequential` container ≈ `List<ILayer>` with composite forward/backward
* Mini-batch processing ≈ LINQ `Chunk()` over training data
* Backpropagation ≈ Manual implementation of the chain rule through nested data structures

## Dependencies

**Core:**
* `ndarray = "0.16"` (Linear algebra, matrix operations)
* `ndarray-rand = "0.15"` (Random weight initialization)
* `rand = "0.8"` (RNG backend)
* `thiserror = "2.0"` (Error types)

**MNIST Dataset:**
* `mnist = "0.5"` (MNIST dataset loader - evaluates to using this vs manual IDX parser)

**Optional Performance:**
* `rayon = "1.8"` (Parallel batch processing)
* `blas-src = "0.8"` (BLAS backend for faster matrix operations)

## Network Architecture Diagram

```
Input (784)  →  Dense(784→128) → ReLU  →  Dense(128→64) → ReLU  →  Dense(64→10) → Softmax  →  Output (10)
  [28x28]         [weights]      [act]      [weights]     [act]       [weights]     [act]      [0-9 probs]
```

**Trainable Parameters:**
- Layer 1: 784 × 128 + 128 = 100,480 params
- Layer 2: 128 × 64 + 64 = 8,256 params
- Layer 3: 64 × 10 + 10 = 650 params
- **Total: ~109,000 parameters**

## Estimated Complexity

**Medium → Medium-High**

**Reasoning:**
- ✅ Core backpropagation logic is already implemented and tested
- ⚠️ Softmax activation requires numerical stability (avoid overflow from exp)
- ⚠️ Cross-Entropy loss with Softmax has a mathematical simplification for gradients
- ⚠️ Mini-batch SGD requires careful batching and shuffling logic
- ⚠️ MNIST data loading/preprocessing adds infrastructure work
- ⚠️ Debugging convergence issues (learning rate tuning, gradient checking) can be time-consuming
- ✅ The mathematical foundation is well-established (no novel research required)

**Key Challenges:**
1. Numerical stability in Softmax (subtract max value before exp)
2. Proper gradient flow through Softmax+CrossEntropy combined layer
3. Mini-batch dimension handling (batch_size × features)
4. Hyperparameter tuning (learning rate, batch size, epochs)
5. Debugging why the network isn't learning (gradient vanishing/exploding)

**Success Metric:**
- 90%+ test accuracy on MNIST is achievable with this architecture in 10-20 epochs
- State-of-the-art (CNNs) achieve 99%+, but 90%+ validates our foundation is working correctly