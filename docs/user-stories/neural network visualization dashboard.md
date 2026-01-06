# User Story: Neural Network Visualization Dashboard

## User Story

As a researcher, I need a real-time visualization dashboard so that I can monitor training metrics (loss, accuracy) and inspect the network's decision boundaries during the learning process.

## Problem Statement

Neural networks are often "black boxes". Without visualization, it is difficult to know if a model is overfitting, if gradients are vanishing, or why a specific classification decision was made. For a learning project, seeing the decision plane evolve over epochs provides immediate feedback on hyperparameter choices like learning rate and architecture depth.

## Acceptance Criteria

### Visualization Features

* [ ] **Real-time Loss Curve**: Display a line chart using `plotters-iced` that updates every epoch.
* [ ] **Decision Boundary Plane**: Generate a 2D heatmap showing the network's classification regions for the XOR problem.
* [ ] **Weight Distribution**: Visualize weight magnitudes as a Sankey diagram or clustered mega-neurons to identify high-contribution features.
* [ ] **Accuracy Gauge**: A live percentage display of training vs. validation accuracy.

### Interactive Controls

* [ ] **Training Toggle**: Start/Pause buttons to control the `Trainer` loop from the UI.
* [ ] **Hyperparameter Sliders**: Adjust learning rate and batch size dynamically using Iced widgets.

### Technical Implementation

* [ ] **Message Passing**: Use Iced's `update` method to handle training data sent via a `tokio` channel.
* [ ] **Canvas Rendering**: Use `ChartWidget` from `plotters-iced` for high-performance chart drawing.
* [ ] **Responsive Layout**: Ensure charts resize correctly within the Iced application window.

## Technical Context

The dashboard acts as an observer of the **Application Layer**. While the `Trainer` runs in a background thread, it sends snapshots of the model's state (current loss, accuracy, and weights) to the Iced `Subscription` loop.

**C# Comparison:**

* `Iced` + `Plotters` ≈ **WPF/WinForms** + **LiveCharts** or **OxyPlot**.
* `Subscription` loop ≈ **MVVM PropertyChanged** notification system for async data streams.

## Dependencies

* `iced` (GUI framework)
* `plotters` and `plotters-iced` (Visualization backend)
* `tokio` (Async runtime for training thread)

## Estimated Complexity

High

**Reasoning:**
Requires orchestrating cross-thread communication between the high-speed training loop and the GUI thread without blocking the UI or incurring significant overhead.

---

**Would you like me to generate the `Cargo.toml` workspace configuration and the initial `core` module file structure to begin implementing these stories?**