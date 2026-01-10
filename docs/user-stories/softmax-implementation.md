### User Story: Softmax Activation Implementation

# Implementing Softmax for Multi-Class Classification

## User Story

As a developer implementing the neural network core, I need to understand and implement the Softmax activation function so that the network can output probability distributions for multi-class classification (MNIST digits 0-9).

## What is Softmax?

Softmax converts a vector of arbitrary real numbers into a **probability distribution** where:
- All values are in range [0, 1]
- All values sum to 1
- Larger input values get higher probabilities

### Mathematical Formula

For a vector **z** with elements z₁, z₂, ..., zₙ:

```
σ(z)ᵢ = exp(zᵢ) / Σⱼ exp(zⱼ)
```

### Example (by hand)

Input: `[1.0, 2.0, 3.0]`

**Step 1:** Calculate exp() for each element
- exp(1.0) = 2.718
- exp(2.0) = 7.389
- exp(3.0) = 20.086

**Step 2:** Sum all exp values
- sum = 2.718 + 7.389 + 20.086 = 30.193

**Step 3:** Divide each exp by the sum
- σ(z)₁ = 2.718 / 30.193 = 0.090 (9%)
- σ(z)₂ = 7.389 / 30.193 = 0.245 (24.5%)
- σ(z)₃ = 20.086 / 30.193 = 0.665 (66.5%)

**Output:** `[0.090, 0.245, 0.665]` ✅ Sums to 1.0

Notice that the largest input (3.0) gets the highest probability (66.5%).

## The Numerical Stability Problem

### Why Naive Implementation Fails

```rust
// ❌ NAIVE (WRONG) - Will overflow!
fn softmax_naive(input: &Array1<f32>) -> Array1<f32> {
    let exp_values: Array1<f32> = input.mapv(|x| x.exp());
    let sum: f32 = exp_values.sum();
    exp_values / sum
}
```

**Problem:** exp(1000.0) = INFINITY ⚠️

Try this in your calculator:
- exp(100) ≈ 2.7 × 10⁴³
- exp(1000) = **OVERFLOW** (f32 max is ~3.4 × 10³⁸)

### The Solution: Subtract Max Value

**Mathematical Trick:** Softmax is invariant to constant shifts.

```
σ(z) = σ(z - C)  for any constant C
```

**Choose C = max(z)** to keep values near zero:

```
σ(z)ᵢ = exp(zᵢ - max(z)) / Σⱼ exp(zⱼ - max(z))
```

### Example with Large Values

Input: `[1000.0, 1001.0, 1002.0]`
Max: 1002.0

**Step 1:** Subtract max
- 1000 - 1002 = -2
- 1001 - 1002 = -1
- 1002 - 1002 = 0

**Step 2:** Apply exp (now safe!)
- exp(-2) = 0.135
- exp(-1) = 0.368
- exp(0) = 1.000

**Step 3:** Normalize
- sum = 1.503
- [0.135/1.503, 0.368/1.503, 1.000/1.503]
- **Output:** `[0.090, 0.245, 0.665]`

Same result, no overflow! 🎉

## Implementation Guide

### Part 1: Activation (Forward Pass)

**File:** `core/src/activation.rs` (line 47)

**What you need to do:**

1. **Handle batches** - Input is `Array2<f32>` with shape `[batch_size, num_classes]`
2. **Process each row independently** (each row is one sample)
3. **For each row:**
   - Find max value
   - Subtract max from all elements
   - Compute exp() of each element
   - Sum all exp values
   - Divide each exp by the sum

**Pseudo-code:**

```rust
fn activate(&self, input: &Array2<f32>) -> Array2<f32> {
    let mut result = Array2::zeros(input.dim());

    // Process each row (sample) independently
    for (i, row) in input.axis_iter(Axis(0)).enumerate() {
        // 1. Find max value in this row
        let max_val = /* use .iter().fold() to find max */;

        // 2. Compute exp(x - max) for each element
        let exp_values: Vec<f32> = row.iter()
            .map(|&x| (x - max_val).exp())
            .collect();

        // 3. Sum all exp values
        let sum: f32 = exp_values.iter().sum();

        // 4. Normalize: divide each by sum
        for (j, &exp_val) in exp_values.iter().enumerate() {
            result[[i, j]] = exp_val / sum;
        }
    }

    result
}
```

**Hints:**
- Use `input.axis_iter(ndarray::Axis(0))` to iterate over rows
- Use `row.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))` to find max
- Remember to enumerate rows to get index `i`

### Part 2: Derivative (Backward Pass)

**File:** `core/src/activation.rs` (line 55)

**Important:** Softmax derivative is complex, but we have a shortcut!

**Full Jacobian (complex):**
```
∂σᵢ/∂zⱼ = σᵢ(δᵢⱼ - σⱼ)
```
where δᵢⱼ = 1 if i==j, else 0

**Our Shortcut:** When Softmax is paired with Cross-Entropy loss (which it always is in classification), the gradient simplifies to:

```
gradient = y_pred - y_true
```

This simplification happens in the loss function, not here!

**For now, implement element-wise derivative:**

```rust
fn derivative(&self, input: &Array2<f32>) -> Array2<f32> {
    let softmax_output = self.activate(input);
    // Element-wise: σ(x) * (1 - σ(x))
    &softmax_output * &(1.0 - &softmax_output)
}
```

**Note:** This is an approximation. The full Jacobian requires matrix multiplication. But when combined with Cross-Entropy (later), the math simplifies beautifully.

## Testing Your Implementation

**File:** `core/src/activation.rs` (lines 158-241)

Remove `#[ignore]` from each test as you implement:

1. **Test 1:** Uniform input → Uniform probabilities
   - Input: `[1.0, 1.0, 1.0]`
   - Expected: `[0.333, 0.333, 0.333]`

2. **Test 2:** Simple input → Sum to 1
   - Input: `[1.0, 2.0, 3.0]`
   - Check: `sum ≈ 1.0`

3. **Test 3:** Large values → No overflow
   - Input: `[1000.0, 1001.0, 1002.0]`
   - Check: All values in [0, 1], sum = 1.0

4. **Test 4:** Batch processing
   - Input: 2 rows
   - Check: Each row sums to 1

**Run tests:**
```bash
cargo test softmax_activation -- --nocapture
```

## C# Developer Notes

### Comparison

| Concept | Rust | C# Equivalent |
|---------|------|---------------|
| `axis_iter(Axis(0))` | Iterate over rows | `for (int i = 0; i < matrix.RowCount; i++)` |
| `fold(NEG_INFINITY, ...)` | Find max | `row.Max()` (LINQ) |
| `mapv(|x| x.exp())` | Element-wise exp | `row.Select(x => Math.Exp(x))` |
| `Array2::zeros(dim)` | Create empty matrix | `Matrix<double>.Build.Dense(rows, cols)` |

### Key Rust Patterns

1. **Iterators are lazy** - `.map()` doesn't execute until `.collect()`
2. **Borrowing** - `&row` borrows, doesn't copy
3. **Closures** - `|&x|` pattern matches to dereference

## Common Pitfalls

### ❌ Forgetting to subtract max
```rust
// BAD - Will overflow on large inputs
let exp_values: Vec<f32> = row.iter().map(|&x| x.exp()).collect();
```

### ✅ Correct approach
```rust
// GOOD - Numerically stable
let max_val = row.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
let exp_values: Vec<f32> = row.iter().map(|&x| (x - max_val).exp()).collect();
```

### ❌ Processing entire matrix at once
```rust
// BAD - Doesn't handle batches correctly
let max_val = input.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
```

### ✅ Process each row independently
```rust
// GOOD - Each sample gets its own normalization
for (i, row) in input.axis_iter(Axis(0)).enumerate() {
    let max_val = row.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    // ...
}
```

## Acceptance Criteria

- [ ] Softmax implementation passes all 4 tests
- [ ] No compiler warnings
- [ ] Handles large values (1000+) without overflow
- [ ] Each row in batch sums to 1.0 (within 1e-6 tolerance)
- [ ] All output values in range [0, 1]

## Next Steps

After implementing Softmax, you'll implement:
1. **Cross-Entropy Loss** - Where the gradient simplification happens
2. **Combined Softmax+CrossEntropy Layer** - For numerical stability in backprop

## Resources

- **Why subtract max?** [https://cs231n.github.io/linear-classify/#softmax](https://cs231n.github.io/linear-classify/#softmax)
- **Full Jacobian derivation** (optional): [https://eli.thegreenplace.net/2016/the-softmax-function-and-its-derivative/](https://eli.thegreenplace.net/2016/the-softmax-function-and-its-derivative/)

---

**Time Estimate:** 30-45 minutes (with testing)

**Delete these when implementing:**
- `#[allow(unused_variables)]` annotations (lines 46, 54)
- `todo!()` macros (lines 51, 59)
- `#[ignore]` attributes from tests (lines 160, 175, 188, 202)
