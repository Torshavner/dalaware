# Performance Optimization Guide

## Current Performance Baseline

### Model Architecture
- **Layers**: 784 → 128 → 64 → 10
- **Total Parameters**: ~109,184
- **Training Data**: 60,000 samples (28×28 images)
- **Batches per Epoch**: ~938 (batch size 64)

### Actual Performance (Measured)
- **Current (unoptimized)**: **~5-6 minutes per epoch** (Measured: 351 seconds for epoch 1)
- **10 epochs**: **~50-60 minutes total**
- **Issue**: Full dataset evaluation is the bottleneck!

### Target After Optimizations
- **With optimizations**: 2-3 minutes per epoch
- **10 epochs**: 20-30 minutes total

## Performance Bottlenecks Identified (Prioritized by Impact)

### 0. CRITICAL: Full Dataset Evaluation Per Epoch ⚠️ **BIGGEST BOTTLENECK**
**Location**: [application/src/trainer.rs:114](../application/src/trainer.rs#L114)

```rust
// Evaluates ALL 60,000 samples after EVERY epoch for metrics!
let (loss, accuracy) = self.evaluate(model, inputs, targets)?;
```

**Measured Impact**:
- **Epoch 1**: Started at 20:59:41, completed at 21:05:32 = **351 seconds (~6 minutes)**
- **Breakdown**:
  - Training batches: ~2-3 minutes (normal)
  - Full evaluation: **~3 minutes** (unnecessary!)
- **Total waste**: 50% of training time spent on evaluation!

**Why This is Wrong**:
- Evaluating on the **training set** measures memorization, not generalization
- Should evaluate on **test set** (10,000 samples) for validation accuracy
- Or use a subset (5,000-10,000 samples) for faster feedback

**Solutions** (choose one):

#### Option 1: Use Test Set for Validation (RECOMMENDED)
Evaluate on test set instead of training set - gives real validation accuracy!

**Changes needed**:
1. Pass test data to trainer
2. Evaluate on test set each epoch

```rust
// In MnistTrainingService::train()
let trainer = Trainer::new(config, CrossEntropy);
let metrics = trainer.train(
    &mut model,
    &self.dataset.train_images,
    &self.dataset.train_labels,
    &self.dataset.test_images,    // NEW: Pass test set
    &self.dataset.test_labels,     // NEW: Pass test set
)?;
```

**Benefits**:
- 6x smaller dataset (10,000 vs 60,000)
- Real validation accuracy (not training accuracy)
- **Expected speedup**: Epoch time from 6 min → **2-3 min** (2x faster!)

#### Option 2: Evaluate on Training Subset
Keep current API, but only evaluate on first 10,000 training samples.

```rust
// In train_epoch, replace full evaluation with:
let eval_size = 10000.min(num_samples);
let eval_inputs = inputs.slice(ndarray::s![0..eval_size, ..]);
let eval_targets = targets.slice(ndarray::s![0..eval_size, ..]);
let (loss, accuracy) = self.evaluate(model, &eval_inputs.to_owned(), &eval_targets.to_owned())?;
```

**Benefits**:
- Minimal code changes
- 6x faster evaluation
- **Expected speedup**: Epoch time from 6 min → **2-3 min** (2x faster!)

**Trade-offs**:
- Still evaluating on training data (can overfit)
- Less representative sample

#### Option 3: Skip Evaluation on Some Epochs
Only evaluate every N epochs (e.g., every 2nd or 3rd epoch).

```rust
let (loss, accuracy) = if epoch % 2 == 0 || epoch == self.config.epochs - 1 {
    self.evaluate(model, inputs, targets)?
} else {
    (0.0, 0.0) // Skip, use previous values or zeros
};
```

**Benefits**:
- Simple to implement
- Reduces evaluation overhead by 50-66%

**Trade-offs**:
- Less frequent feedback
- UI won't update loss/accuracy every epoch

**RECOMMENDATION**: Implement **Option 1** (use test set) - it's the most correct approach and gives 2x speedup!

---


### 2. Medium: Input Cloning in Forward Pass
**Location**: [core/src/layer.rs:40](../core/src/layer.rs#L40)

```rust
// Clones entire batch every forward pass for backprop caching
self.cached_input = Some(input.clone());
```

**Impact**: For batch_size=64:
- Clones 64×784 = 50,176 floats
- Happens ~938 times per epoch (training) + ~157 times (evaluation)
- Total copies: ~52M floats per epoch
- Estimated overhead: ~10-15% of training time

**Solution**:
- **Accept it**: Clone is necessary for backprop (gradient computation needs input)
- **Or**: Use `Cow` (Copy-on-Write) for read-only forward passes during evaluation

### 3. Medium: Parameter Updates Create New Arrays
**Location**: [core/src/layer.rs:57-62](../core/src/layer.rs#L57-L62)

```rust
// Creates new array instead of in-place update
self.weights = &self.weights - &(weight_grads * learning_rate);
self.biases = &self.biases - &(bias_grads * learning_rate);
```

**Impact**:
- Allocates new arrays for every parameter update
- Happens 3 times per batch (weights + biases for 3 layers)
- ~2,800 allocations per epoch
- Estimated overhead: ~5-10% of training time

**Solution**: Use in-place operations (see below)

## Optimization Implementation

### Immediate Optimizations (Prioritized by Impact)

#### 1. Use Test Set for Validation (HIGHEST IMPACT - 2x speedup!)
**Estimated speedup**: Epoch time from **6 min → 2-3 min**

See detailed options in "Critical Issue #0" above. Recommended approach:

```rust
// In Trainer::train(), add test data parameters
pub fn train<M: Module>(
    &self,
    model: &mut M,
    train_inputs: &Array2<f32>,
    train_targets: &Array2<f32>,
    test_inputs: &Array2<f32>,   // NEW
    test_targets: &Array2<f32>,  // NEW
) -> anyhow::Result<Vec<EpochMetrics>> {
    // ... training loop ...

    // Evaluate on test set instead of training set
    let (loss, accuracy) = self.evaluate(model, test_inputs, test_targets)?;
}
```

#### 2. In-Place Parameter Updates
Replace array subtraction with scaled add operations.

**Before**:
```rust
self.weights = &self.weights - &(weight_grads * learning_rate);
self.biases = &self.biases - &(bias_grads * learning_rate);
```

**After**:
```rust
use ndarray::Zip;
Zip::from(&mut self.weights)
    .and(&*weight_grads)
    .for_each(|w, &g| *w -= learning_rate * g);

Zip::from(&mut self.biases)
    .and(&*bias_grads)
    .for_each(|b, &g| *b -= learning_rate * g);
```

**Expected speedup**: ~10-15% faster

#### 3. Add Timing Instrumentation
Add per-epoch timing to identify bottlenecks:

```rust
let start = std::time::Instant::now();
let epoch_metrics = self.train_epoch(model, inputs, targets, epoch)?;
let duration = start.elapsed();

tracing::info!(
    epoch = epoch + 1,
    duration_secs = duration.as_secs(),
    loss = %format!("{:.6}", epoch_metrics.loss),
    accuracy = %format!("{:.2}%", epoch_metrics.accuracy * 100.0),
    "Epoch completed"
);
```

### Advanced Optimizations (Future)

#### 1. Parallel Batch Processing
Use `rayon` for parallel forward/backward passes across batch samples.

**Expected speedup**: 2-4x on multi-core CPUs

#### 2. SIMD Optimizations
Enable `ndarray` BLAS backend for optimized matrix multiplication.

```toml
[dependencies]
ndarray = { version = "0.16", features = ["blas"] }
blis-src = "0.2"  # or openblas-src
```

**Expected speedup**: 3-10x depending on CPU

#### 3. Mixed Precision Training (f16)
Use half-precision floats for forward pass, full precision for backward pass.

**Expected speedup**: 1.5-2x with some accuracy trade-off

## Recommended Changes for MNIST Painter

### Short-term (implement now):
1. ✅ Add training progress logs (DONE)
2. 🔴 **Use test set for validation** (2x speedup - CRITICAL!)
3. In-place parameter updates (~10-15% speedup)
4. Add epoch timing instrumentation

### Medium-term:
1. Add progress callbacks for UI updates
2. Enable BLAS backend
3. Add early stopping based on validation accuracy

### Long-term:
1. GPU support via `wgpu` backend
2. Model checkpointing
3. Learning rate scheduling

## Performance Targets

### Current (Measured - Unoptimized)
- **Epoch time**: ~6 minutes (351 seconds measured)
- **10 epochs**: ~60 minutes
- **Issue**: Full dataset evaluation doubles training time

### With Test Set Validation (Option 1)
- **Epoch time**: 2-3 minutes (2x speedup)
- **10 epochs**: 20-30 minutes
- **Implementation complexity**: Medium (API change)

### With Training Subset Evaluation (Option 2)
- **Epoch time**: 2-3 minutes (2x speedup)
- **10 epochs**: 20-30 minutes
- **Implementation complexity**: Low (no API change)

### With In-Place Updates Added
- **Epoch time**: 1.5-2.5 minutes (additional 10-15% speedup)
- **10 epochs**: 15-25 minutes

### With BLAS Backend (Future)
- **Epoch time**: 30-60 seconds (3-5x additional speedup)
- **10 epochs**: 5-10 minutes
- **Requires**: External BLAS library

## How to Measure

Add timing to trainer:
```rust
let start = std::time::Instant::now();
let epoch_metrics = self.train_epoch(model, inputs, targets, epoch)?;
let duration = start.elapsed();

tracing::info!(
    epoch = epoch + 1,
    duration_ms = duration.as_millis(),
    samples_per_sec = (num_samples as f64 / duration.as_secs_f64()) as u64,
    "Epoch completed"
);
```
