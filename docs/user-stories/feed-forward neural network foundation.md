### User Story: Feed-Forward Neural Network Foundation

# Feed-Forward Neural Network Core Implementation

## User Story

As a developer, I need a modular Feed-Forward Neural Network engine so that I can train models to solve regression and classification problems without relying on high-level ML libraries.

## Problem Statement

Developing neural networks in Python often hides the underlying math and introduces performance bottlenecks. This project aims to bridge that gap by implementing these concepts from first principles in a high-performance, memory-safe language like Rust.

## Acceptance Criteria

### Core Functionality

* [ ] Implement a `DenseLayer` struct that stores weights and biases using `ndarray`
* [ ] Implement forward propagation that computes pre-activations and applies activation functions
* [ ] Implement backpropagation to calculate gradients for each layer
* [ ] Support multiple activation functions (Sigmoid, ReLU, Softmax)

### Training and Optimization

* [ ] Implement a `Trainer` that orchestrates epochs, batching, and gradient descent
* [ ] Calculate loss using Cross-Entropy for classification tasks
* [ ] Update parameters using a configurable learning rate

### Testing (Test-First)

* [ ] Write tests verifying that forward propagation on a 2x2 identity matrix returns expected values
* [ ] Verify gradients calculated during backprop match numerical approximations for a single-layer perceptron
* [ ] Successfully train the network to solve the XOR problem (convergence within <1000 epochs)

## Technical Context

* **Matrix Operations**: Use `ndarray` for all tensor math and `ndarray-rand` for parameter initialization.
* **Parallelism**: Use `Rayon` for parallelizing batch processing if performance gains are needed for large datasets.
* **Persistence**: Use `serde` to serialize/deserialize trained model weights to JSON or MessagePack.

**C# Comparison:**

* `ndarray::Array2` ≈ `MathNet.Numerics.Matrix<double>`
* Backpropagation ≈ Manual implementation of the chain rule through nested data structures.

## Dependencies

* `ndarray` (Linear algebra)
* `ndarray-rand` (Random weight initialization)
* `rand` (RNG backend)
* `anyhow` (Error propagation)

## Estimated Complexity

Medium

**Reasoning:**
While the math is standard, implementing backpropagation correctly requires careful handling of matrix transpositions and dimension matching, which is a common source of subtle bugs in custom engines.