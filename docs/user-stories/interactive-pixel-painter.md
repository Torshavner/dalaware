# User Story: Interactive Pixel Painter for MNIST Input

## Overview
Build a Rust-based pixel painting tool that allows users to draw 28x28 grayscale images (matching MNIST format) and feed them directly to the trained neural network for digit classification.

## User Story
**As a** user of the neural network
**I want** to draw digits on a 28x28 canvas
**So that** I can test the trained model with my own handwritten inputs in real-time

## Technical Requirements

### Canvas Specifications
- 28x28 pixel grid (MNIST standard)
- Grayscale values [0, 255] (normalized to [0.0, 1.0] for network input)
- Drawing with variable brush sizes (1x1, 2x2, 3x3 pixels)
- Clear/reset functionality
- Visual scaling for better UX (render at 280x280 or 560x560 for visibility)

### Input Processing
- Convert painted canvas to `Array2<f32>` with shape (1, 784)
- Apply same normalization as MNIST loader (pixel / 255.0)
- Optional: Apply Gaussian blur to simulate pen thickness (makes drawn digits more realistic)

### Network Integration
- Load pre-trained model weights (requires serialization - future work)
- OR: Train model first, keep in memory, then run painter
- Run forward pass on painted input
- Display predicted digit + confidence scores for all 10 classes

### Library Options for GUI

#### Option 1: **iced** (Elm-Inspired Declarative UI) ⭐ RECOMMENDED
```toml
[dependencies]
iced = "0.12"
```

**Pros:**
- **Clean architecture** - Elm pattern (Model-View-Update) enforces separation of concerns
- **Type-safe state management** - Messages are enums, compiler prevents invalid states
- **Full widget suite** - Buttons, sliders, text, canvas, layouts included
- **Cross-platform native** - Windows, macOS, Linux with native look
- **Scales naturally** - Easy to add training controls, confidence charts, model management
- **Active ecosystem** - Good documentation, growing community
- **Aligns with project architecture** - Similar to your core/application separation

**Cons:**
- Elm architecture learning curve (message-driven, declarative)
- More boilerplate than immediate mode (message enum, update function, view function)
- Heavier than minimal framebuffer approaches

**C# Analogy:**
- Similar to **Avalonia UI** or **MAUI** (cross-platform, declarative XAML-like)
- Message pattern is like **MVVM with commands/events**
- State updates similar to Redux/Flux in React

**Best for:** Structured applications, extensibility, production-quality UI, portfolio projects

---

#### Option 2: **minifb** (Minimal Framebuffer)
```toml
[dependencies]
minifb = "0.25"
```

**Pros:**
- Lightweight, minimal dependencies
- Direct pixel manipulation (perfect for 28x28 grid)
- Cross-platform (Windows, macOS, Linux)
- Simple event handling (mouse, keyboard)
- Maximum control, educational value

**Cons:**
- Low-level (you draw pixels manually)
- No built-in UI widgets (buttons, text)
- Requires manual scaling for display
- Hard to extend with polish

**C# Analogy:**
- Like writing to `Bitmap` buffer in WinForms with manual `SetPixel()`
- Lower-level than WPF, closer to GDI+

**Best for:** Minimal dependency, from-scratch learning, maximum control

---

#### Option 3: **egui** (Immediate Mode GUI)
```toml
[dependencies]
egui = "0.27"
eframe = "0.27"  # Application framework for egui
```

**Pros:**
- High-level UI framework (buttons, sliders, text out of the box)
- Immediate mode (like ImGui) - very intuitive, rapid prototyping
- Can draw custom widgets (28x28 grid as a custom widget)
- Built-in styling, layout, text rendering
- Less boilerplate than Elm pattern

**Cons:**
- Immediate mode = less structured than iced (direct mutation in closures)
- Less aligned with Rust ownership idioms
- Can become spaghetti code in larger apps

**C# Analogy:**
- Similar to **ImGui.NET** (immediate mode)
- Less like WPF (retained mode)

**Best for:** Rapid prototyping, internal tools, less formal structure

---

#### Option 4: **pixels + winit** (Modern Pixel Buffer)
```toml
[dependencies]
pixels = "0.13"
winit = "0.29"
```

**Pros:**
- Modern GPU-accelerated pixel rendering
- Integrates with `winit` for window/event management
- Good for real-time graphics
- Clean API for pixel buffers

**Cons:**
- More complex setup than `minifb`
- GPU dependency (though minimal)
- Manual UI construction

**Best for:** Performance-sensitive applications, modern graphics stack

---

#### Option 5: **nannou** (Creative Coding)
```toml
[dependencies]
nannou = "0.19"
```

**Pros:**
- Built for creative/visual applications
- Easy drawing primitives (rectangles, circles)
- Good for visualizations

**Cons:**
- Heavy framework (brings entire graphics ecosystem)
- Overkill for simple pixel grid

**Best for:** Visual demos, presentations, artistic projects

---

## Recommended Approach

### Phase 1: Console-Based (No GUI Yet)
Start without GUI to validate the pipeline:
```rust
// Draw a hardcoded digit pattern
let canvas = array![
    [0.0, 0.0, 1.0, 1.0, ...],  // 28x28 grid
    // ... (hardcode a "7" or similar)
];
let input = canvas.into_shape((1, 784)).unwrap();
let prediction = model.forward(&input);
println!("Predicted: {}", argmax(&prediction));
```

### Phase 2: Production GUI with **iced** ⭐ RECOMMENDED
Reasoning:
- **Structured architecture** - Aligns with your core/application separation philosophy
- **Type-safe** - Message-driven state management prevents invalid UI states
- **Extensible** - Easy to add controls, charts, model management later
- **Professional** - Suitable for portfolio/demo purposes
- **Declarative** - Similar to C# WPF/XAML, Avalonia patterns you know

```rust
use iced::{
    widget::{button, canvas, column, container, row, text, Canvas},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

struct MnistPainter {
    canvas: [u8; 28 * 28],
    prediction: Option<PredictionResult>,
    model: Sequential,
}

struct PredictionResult {
    digit: usize,
    confidence: f32,
    all_scores: Vec<f32>,
}

#[derive(Debug, Clone)]
enum Message {
    CanvasClicked { x: usize, y: usize },
    CanvasDragged { x: usize, y: usize },
    Clear,
    Predict,
}

impl Application for MnistPainter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Sequential;

    fn new(model: Sequential) -> (Self, Command<Message>) {
        (
            Self {
                canvas: [0; 28 * 28],
                prediction: None,
                model,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("MNIST Digit Painter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CanvasClicked { x, y } => {
                self.paint_pixel(x, y);
            }
            Message::CanvasDragged { x, y } => {
                self.paint_pixel(x, y);
            }
            Message::Clear => {
                self.canvas.fill(0);
                self.prediction = None;
            }
            Message::Predict => {
                let input = self.canvas_to_network_input();
                let output = self.model.forward(&input);
                self.prediction = Some(self.extract_prediction(&output));
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(CanvasDrawer { pixels: &self.canvas })
            .width(Length::Fixed(280.0))
            .height(Length::Fixed(280.0));

        let controls = row![
            button("Clear").on_press(Message::Clear),
            button("Predict").on_press(Message::Predict),
        ]
        .spacing(10);

        let prediction_text = match &self.prediction {
            None => text("Draw a digit and click Predict"),
            Some(pred) => text(format!(
                "Prediction: {} ({:.1}% confidence)",
                pred.digit,
                pred.confidence * 100.0
            )),
        };

        column![canvas, controls, prediction_text]
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }
}

impl MnistPainter {
    fn paint_pixel(&mut self, x: usize, y: usize) {
        if x < 28 && y < 28 {
            self.canvas[y * 28 + x] = 255;
        }
    }

    fn canvas_to_network_input(&self) -> Array2<f32> {
        let normalized: Vec<f32> = self.canvas.iter()
            .map(|&p| p as f32 / 255.0)
            .collect();
        Array2::from_shape_vec((1, 784), normalized).unwrap()
    }

    fn extract_prediction(&self, output: &Array2<f32>) -> PredictionResult {
        let scores: Vec<f32> = output.row(0).to_vec();
        let (digit, &confidence) = scores.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        PredictionResult {
            digit,
            confidence,
            all_scores: scores,
        }
    }
}

fn main() -> iced::Result {
    // Train model first (or load from disk)
    let model = train_mnist_model();

    MnistPainter::run(Settings::with_flags(model))
}
```

### Phase 3: Enhanced Features (After Basic UI Works)
Once the basic iced UI is working, easily extend:
- **Confidence bar chart** for all 10 digits
- **Brush size control** (slider widget)
- **Model training controls** (epochs, learning rate sliders)
- **Save/Load model** buttons
- **Real-time loss/accuracy visualization** during training

## Comparison Table

| Feature | iced | minifb | egui | pixels+winit | nannou |
|---------|------|--------|------|--------------|--------|
| **Architecture** | Elm (MVU) | Manual loop | Immediate | Manual loop | Framework |
| **Complexity** | Medium | Low | Medium | Medium | High |
| **Dependencies** | Moderate | Minimal | Moderate | Moderate | Heavy |
| **UI Widgets** | Full suite | None | Full suite | None | Some |
| **State Management** | Built-in (type-safe) | Manual | Manual | Manual | Manual |
| **Extensibility** | ✅ Excellent | ❌ Hard | ✅ Good | ❌ Hard | ⚠️ Medium |
| **Learning Curve** | Moderate (Elm) | Easy | Moderate | Moderate | Steep |
| **Production Ready** | ✅ Yes | ⚠️ Basic | ✅ Yes | ⚠️ Basic | ❌ Niche |
| **Project Fit** | ✅ **Best** | ✅ Good | ✅ Good | ⚠️ Technical | ❌ Too heavy |

### Detailed Comparison: iced vs. egui vs. minifb

| Aspect | iced | egui | minifb |
|--------|------|------|--------|
| **Paradigm** | Retained (Elm/React) | Immediate (ImGui) | Manual pixel buffer |
| **State** | Explicit struct + messages | Direct mutation in closure | Manual struct |
| **Rendering** | Declarative `view()` | Imperative `ui.add(...)` | Raw pixel writes |
| **Best For** | Structured apps, scaling | Rapid prototyping, tools | Educational, minimal |
| **Rust Idioms** | Strong ownership, messages | Flexible, less strict | Full control |
| **C# Analogy** | WPF/Avalonia/MAUI | ImGui.NET | GDI+ Bitmap |

## Acceptance Criteria
- [ ] User can draw on a 28x28 canvas using mouse/trackpad
- [ ] Canvas visually scales up (e.g., 10x) for usability
- [ ] Clear button resets canvas
- [ ] Predict button feeds canvas to trained model
- [ ] Displays predicted digit and confidence scores
- [ ] Input normalization matches MNIST preprocessing

## Architecture Considerations

### C# Developer Perspective

**iced** is conceptually similar to:
- **Avalonia UI** or **MAUI** (cross-platform declarative UI)
- **WPF with MVVM** (data binding via messages, separation of view/logic)
- **React/Redux** in web (unidirectional data flow, message-driven)
- State updates like `ICommand` execution in MVVM

**egui** is conceptually similar to:
- **ImGui.NET** (immediate mode GUI)
- Less like WPF (retained mode), more like direct drawing in game loop

**minifb** is conceptually similar to:
- Writing directly to a `Bitmap` pixel buffer in C#
- WinForms `PictureBox` with manual `Bitmap.SetPixel()` calls
- Lower-level than WPF/Avalonia, closer to GDI+

### Rust-Specific Notes (iced)
- **Ownership**: State is owned by the app struct, messages are `Clone`
- **Event Loop**: Managed by iced runtime (like WPF dispatcher)
- **Elm Architecture**:
  - `Model` = Your state struct
  - `update()` = Handle messages, return new state
  - `view()` = Render UI declaratively based on current state
- **No Runtime**: Compiles to native binary (like Avalonia Native)

### Elm Architecture Pattern (iced)
```
User Action → Message → update() → New State → view() → UI Update
     ↑                                                      ↓
     └──────────────────── Event Loop ─────────────────────┘
```

This is identical to:
- **MVVM**: `User Action → Command → ViewModel Update → PropertyChanged → View Update`
- **Redux**: `Action → Reducer → New State → Component Re-render`

## Questions for Discussion

1. **Model Persistence**: Do you want to save/load trained weights, or train fresh each time?
   - If save/load: Need serialization (serde + bincode/JSON for weights)
   - If fresh: Train → hold in memory → open painter

2. **UX Complexity**: How polished?
   - Minimal: Just canvas + console output for prediction
   - Moderate: Canvas + on-screen prediction text
   - Full: Canvas + confidence bars + training controls

3. **Deployment**: Who's the audience?
   - Just you: Console + simple GUI is fine
   - Demo/portfolio: Spend time on egui polish
   - Educational (others learning): Keep it minimal (minifb) for clarity

4. **Integration Point**:
   - Separate binary (`nn-painter` package)?
   - Example in `nn-application/examples/painter.rs`?
   - Add to tests as visual validation tool?

## Recommended Next Steps

1. **Create `infrastructure` or `examples` package** for UI code (keep `core`/`application` pure)
2. **Start with Phase 1** (hardcoded canvas, validate pipeline)
3. **Implement Phase 2 with minifb** (aligns with project goals)
4. **Optionally migrate to egui** if you want to polish it for portfolio/demo

## File Structure Proposal
```
dalaware/
├── core/                   # Pure math (no changes)
├── application/            # Training, MNIST loader (no changes)
├── infrastructure/         # NEW: External I/O, GUI
│   ├── src/
│   │   ├── lib.rs
│   │   ├── painter.rs      # minifb canvas implementation
│   │   └── model_serde.rs  # Save/load model weights (optional)
│   └── Cargo.toml
└── examples/
    └── mnist_painter.rs    # Main executable
```

---

## Final Recommendation: **iced** ⭐

### Why iced is the Best Fit

1. **Aligns with Project Architecture**
   - You've already structured the project with clean separation (core/application)
   - Elm architecture extends this pattern to the UI layer
   - Type-safe message passing mirrors your trait-based design

2. **Professional Quality**
   - Suitable for portfolio/demo
   - Native cross-platform support
   - Production-ready framework

3. **Extensibility**
   - Easy to add training controls later
   - Natural fit for confidence visualization
   - Can integrate model save/load UI

4. **Rust Learning**
   - Teaches Elm architecture (valuable pattern)
   - Reinforces ownership/message-passing concepts
   - More idiomatic Rust than immediate mode

5. **C# Transition**
   - If you know WPF/Avalonia MVVM, this will feel familiar
   - Message pattern = `ICommand` pattern
   - Declarative view = XAML bindings

### When to Use Alternatives

- **Use minifb if**: You want absolute minimal dependencies and full pixel control (educational focus)
- **Use egui if**: You need to prototype very quickly and don't care about structure
- **Use pixels+winit if**: You're building a performance-critical graphics application

For this project (neural network with potential for training UI, confidence visualization, model management), **iced provides the best foundation**.
