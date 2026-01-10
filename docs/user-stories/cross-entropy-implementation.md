### User Story: Cross-Entropy Loss Implementation

# Implementing Cross-Entropy for Multi-Class Classification

## User Story

As a developer implementing the neural network core, I need to understand and implement the Cross-Entropy loss function so that the network can properly train for multi-class classification (MNIST digits 0-9).

## What is Cross-Entropy Loss?

Cross-Entropy measures how different two probability distributions are. In classification:
- **Target (y_true)**: One-hot encoded vector (e.g., `[0, 0, 1, 0, 0]` for class 2)
- **Prediction (y_pred)**: Probability distribution from Softmax (e.g., `[0.1, 0.2, 0.6, 0.05, 0.05]`)

**Lower loss = Better predictions**

### Mathematical Formula

```
L = -Σᵢ (y_true_i * log(y_pred_i))
```

For batches (multiple samples), average the loss:
```
L = -(1/N) * ΣₙΣᵢ (y_true_n,i * log(y_pred_n,i))
```

### Example (by hand)

**Prediction:** `[0.1, 0.2, 0.7]` (Softmax output)
**Target:** `[0, 0, 1]` (True class is 2)

**Step 1:** Element-wise multiply
- 0 * log(0.1) = 0
- 0 * log(0.2) = 0
- 1 * log(0.7) = 1 * (-0.357) = -0.357

**Step 2:** Sum and negate
- L = -(0 + 0 + (-0.357)) = 0.357

**Interpretation:** The model is fairly confident (70%) about the correct class, so loss is low.

### Why It Works

| Prediction | Target | Loss | Meaning |
|------------|--------|------|---------|
| `[0.9, 0.05, 0.05]` | `[1, 0, 0]` | 0.105 | ✅ Correct, confident → Low loss |
| `[0.4, 0.3, 0.3]` | `[1, 0, 0]` | 0.916 | ⚠️ Correct, uncertain → Medium loss |
| `[0.1, 0.2, 0.7]` | `[1, 0, 0]` | 2.303 | ❌ Wrong, confident → High loss |

## The log(0) Problem

**Naive implementation has a bug:**

```rust
// ❌ WRONG - Can cause panic!
let loss = -(targets * predictions.mapv(|x| x.ln())).sum();
```

**Problem:** If `predictions` contains 0, then `ln(0) = -∞` → NaN

**Solution:** Add tiny epsilon before taking log:

```rust
const EPSILON: f32 = 1e-7;
let safe_predictions = predictions.mapv(|x| x.max(EPSILON));
let loss = -(targets * safe_predictions.mapv(|x| x.ln())).sum();
```

## The Magical Gradient Simplification

### Full Math (Complex)

Gradient of Cross-Entropy with respect to Softmax input:

```
∂L/∂z = σ(z) - y_true
```

This involves:
1. Softmax Jacobian (complex matrix)
2. Cross-Entropy gradient
3. Chain rule
4. **Miraculously simplifies to:** `predictions - targets`

### Why This is Amazing

**Instead of:** Computing complex Jacobian matrix multiplication
**We get:** Simple element-wise subtraction!

**Example:**
- Predictions: `[0.1, 0.2, 0.7]`
- Targets: `[0.0, 0.0, 1.0]`
- **Gradient:** `[0.1, 0.2, -0.3]`

The negative gradient for the correct class (-0.3) pushes the network to increase that probability.

## Implementation Guide

### Part 1: Loss Calculation

**File:** `core/src/loss.rs` (line 43)

```rust
fn calculate(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> f32 {
    const EPSILON: f32 = 1e-7;

    // 1. Clip predictions to avoid log(0)
    let safe_predictions = predictions.mapv(|x| x.max(EPSILON));

    // 2. Compute element-wise: y_true * log(y_pred)
    let element_wise = targets * &safe_predictions.mapv(|x| x.ln());

    // 3. Sum all elements and negate
    let total_loss = -element_wise.sum();

    // 4. Average over batch
    #[allow(clippy::cast_precision_loss)]
    let batch_size = predictions.nrows() as f32;

    total_loss / batch_size
}
```

**Key points:**
- Use `mapv()` for element-wise operations
- `max(EPSILON)` ensures no value is below 1e-7
- Divide by `batch_size` (number of rows) to get average loss

### Part 2: Gradient Calculation

**File:** `core/src/loss.rs` (line 51)

This is the easiest part thanks to the mathematical simplification!

```rust
fn gradient(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> Array2<f32> {
    // The magical simplification: ∂L/∂z = y_pred - y_true
    predictions - targets
}
```

That's it! No division by batch size needed here (it's absorbed into the learning rate).

## Testing Your Implementation

**File:** `core/src/loss.rs` (lines 107-180)

Remove `#[ignore]` from each test as you implement:

1. **Test 1:** Perfect predictions → Near zero loss
   - Pred: `[0, 0, 1]`, Target: `[0, 0, 1]`
   - Expected: loss < 0.01

2. **Test 2:** Wrong prediction → High loss
   - Pred: `[0.9, 0.05, 0.05]`, Target: `[0, 0, 1]`
   - Expected: loss > 2.0

3. **Test 3:** Softmax output → Correct cross-entropy
   - Pred: `[0.1, 0.2, 0.7]`, Target: `[0, 0, 1]`
   - Expected: -ln(0.7) ≈ 0.357

4. **Test 4:** Batch processing → Averaged loss
   - 2 samples
   - Check: loss is average of both

5. **Test 5:** Gradient → Simple subtraction
   - Pred: `[0.1, 0.2, 0.7]`, Target: `[0, 0, 1]`
   - Expected: `[0.1, 0.2, -0.3]`

**Run tests:**
```bash
cargo test cross_entropy -- --nocapture
```

## C# Developer Notes

| Rust | C# Equivalent |
|------|---------------|
| `predictions.mapv(\|x\| x.max(EPSILON))` | `predictions.Select(x => Math.Max(x, EPSILON))` |
| `targets * &safe_predictions.mapv(\|x\| x.ln())` | Element-wise multiply + log |
| `element_wise.sum()` | `element_wise.Sum()` (LINQ) |
| `predictions.nrows()` | `predictions.RowCount` |

## Common Pitfalls

### ❌ Forgetting epsilon
```rust
// BAD - Will panic on log(0)
let loss = -(targets * predictions.mapv(|x| x.ln())).sum();
```

### ✅ Safe implementation
```rust
// GOOD - Clips to avoid log(0)
const EPSILON: f32 = 1e-7;
let safe_predictions = predictions.mapv(|x| x.max(EPSILON));
```

### ❌ Not averaging over batch
```rust
// BAD - Loss grows with batch size
return -element_wise.sum();
```

### ✅ Correct averaging
```rust
// GOOD - Consistent loss regardless of batch size
let batch_size = predictions.nrows() as f32;
return total_loss / batch_size;
```

## Why Softmax + Cross-Entropy Pair?

They're mathematically designed to work together:

1. **Softmax** converts logits → probabilities (sums to 1)
2. **Cross-Entropy** measures distance between probability distributions
3. **Gradient** simplifies beautifully (no Jacobian needed!)

**Alternative pairs that DON'T work as well:**
- Sigmoid + MSE → Slower convergence
- Softmax + MSE → Gradient doesn't flow properly

## Acceptance Criteria

- [ ] Cross-Entropy implementation passes all 5 tests
- [ ] No compiler warnings
- [ ] Handles edge cases (predictions near 0) with epsilon
- [ ] Batch averaging works correctly
- [ ] Gradient is simple subtraction (predictions - targets)

## Next Steps

After implementing Cross-Entropy, you'll create:
1. **ActivationLayer wrapper** - Integrate Softmax into Module trait
2. **Sequential container** - Compose layers
3. **Trainer** - Use these components together

## Resources

- **Why this gradient?** [https://deepnotes.io/softmax-crossentropy](https://deepnotes.io/softmax-crossentropy)
- **Full derivation** (optional): [https://www.ics.uci.edu/~pjsadows/notes.pdf](https://www.ics.uci.edu/~pjsadows/notes.pdf)

---

**Time Estimate:** 20-30 minutes (with testing)

**Delete these when implementing:**
- `#[allow(unused_variables)]` annotations (lines 42, 50)
- `todo!()` macros (lines 47, 55)
- `#[ignore]` attributes from tests (lines 109, 122, 135, 149, 168)
