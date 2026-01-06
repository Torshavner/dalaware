# Architecture: Neural Network Framework (Dreadnought-AI)

## 1. Design Philosophy

The framework follows **Clean Architecture** principles to ensure that the mathematical core is decoupled from the user interface (ICED) and infrastructure (persistence/logging).

* **Generic Tensors**: Using `ndarray` as the primary data structure for N-dimensional operations.
* **Trait-Driven Flexibility**: Components like `Activation` and `Optimizer` are defined as traits to allow for easy extension (e.g., adding ReLU, Adam, or Momentum later).
* **Compile-Time Safety**: Utilizing Rust's type system to ensure that layers can only be connected if their dimensions align (leveraging `anyhow` for runtime validation where dynamic shapes are required).

## 2. Layered Structure

### Core Layer (`core/`)

Contains the fundamental building blocks and mathematical traits.

* **`Array2<f32>`**: Using concrete ndarray 2D arrays for consistency.
* **`Module` Trait**: The base interface for all network components (Layers).
  * `forward(&mut self, input: &Array2<f32>) -> Array2<f32>` - Computes output and caches activations
  * `backward(&mut self, grad_output: &Array2<f32>) -> Array2<f32>` - Computes input gradients
  * `update_parameters(&mut self, learning_rate: f32)` - Applies gradient descent
* **Rationale**: `&mut self` in forward allows activation caching for backprop; separate `update_parameters` decouples gradient calculation from parameter updates (Strategy Pattern for different optimizers)



### Application Layer (`application/`)

Orchestrates the training loops and network construction.

* **`Sequential`**: A container struct that holds a vector of `Box<dyn Module>` to define the network architecture.
* **`Trainer`**: Manages the training epoch, batching, and evaluation logic.

### Infrastructure Layer (`infrastructure/`)

Handles the "outside world."

* **`Persistence`**: Logic for saving/loading weights using `serde` (JSON/MessagePack).
* **`Telemetry`**: Integration with `tracing` to send loss metrics to Loki/Grafana via the established MCP pipeline.

### UI Layer (`viz_dashboard/`)

The **ICED** application for real-time visualization of decision planes.

---

## 3. Component Diagram

| Component | Responsibility | C# Analogy |
| --- | --- | --- |
| **Module Trait** | Interface for forward/backward passes | `ILayer` or `INetworkModule` |
| **Optimizer Trait** | Updates weights based on gradients | `IUpdateStrategy` |
| **Sequential** | Container for ordered layers | `List<ILayer>` with aggregate execution |
| **Storage** | Serializes model state | `IModelRepository` |

---

## 4. Data Flow (Training Pass)

1. **Forward Pass**: Data flows through `Layer` -> `Activation` -> `Layer` -> `Activation`.
2. **Loss Calculation**: Output is compared against targets using a `Loss` function (e.g., MSE).
3. **Backward Pass**: Gradients are calculated in reverse order using the Chain Rule.
4. **Optimization**: The `Optimizer` (e.g., SGD) modifies the `weights` and `biases` stored in the layers.

---

## 5. Extensibility Path

To ensure this framework can create "Any other network," we implement the following:

* **Convolutional Layers**: By using `ndarray`, we can handle 4D tensors (Batch, Channel, Height, Width) for future Image Recognition tasks.
* **Custom Loss Functions**: Users can implement the `Loss` trait to support Classification (Cross-Entropy) or Regression (MSE).

---

## Technical Reasoning for the C# Developer

* **`Box<dyn Module>` vs Generics**: In C#, you would use an interface `IModule`. In Rust, we use `dyn Module` (Dynamic Dispatch) for the `Sequential` container because it allows us to have a list of *different* layer types (Dense, Conv, Dropout) in a single `Vec`. If we used pure Generics, the entire list would have to be the exact same type.
* **Ownership of Weights**: Weights are owned by the `Layer` struct. During the backward pass, we use **mutable borrows** (`&mut self`) to update them, ensuring no other part of the program can touch the weights while they are being modified.