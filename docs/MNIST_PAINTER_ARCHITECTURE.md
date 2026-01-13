# MNIST Painter Architecture (iced)

## Overview
Interactive 28x28 pixel painter using iced's Elm architecture to test the trained MNIST neural network with user-drawn digits.

## Architecture Principles

### Alignment with Existing Project
```
dalaware/
├── core/              # Pure math, no I/O (✅ Unchanged)
├── application/       # Training, evaluation (✅ Unchanged)
└── infrastructure/    # NEW: External I/O, GUI, serialization
    ├── src/
    │   ├── lib.rs
    │   ├── painter/           # iced GUI components
    │   │   ├── mod.rs
    │   │   ├── app.rs         # Application trait implementation
    │   │   ├── canvas.rs      # Canvas widget for 28x28 drawing
    │   │   ├── message.rs     # Message enum
    │   │   └── styles.rs      # UI styling
    │   └── model_io/          # Future: Model serialization
    │       └── mod.rs
    └── examples/
        └── mnist_painter.rs   # Main executable
```

### Elm Architecture (Model-View-Update)

```
┌─────────────────────────────────────────────────────────────┐
│                         User Input                           │
│                  (Mouse Click, Button Press)                 │
└────────────────────────────┬────────────────────────────────┘
                             │
                             ▼
                        ┌─────────┐
                        │ Message │  (enum)
                        └────┬────┘
                             │
                             ▼
                      ┌──────────────┐
                      │   update()   │  (Pure function)
                      │              │  Pattern match on message
                      │  Returns:    │  Update state
                      │  - New State │  Return Command (async)
                      │  - Command   │
                      └──────┬───────┘
                             │
                             ▼
                      ┌──────────────┐
                      │    Model     │  (State struct)
                      │              │
                      │  - canvas    │
                      │  - model     │
                      │  - prediction│
                      └──────┬───────┘
                             │
                             ▼
                      ┌──────────────┐
                      │    view()    │  (Declarative)
                      │              │  Build UI tree from state
                      │  Returns:    │  No side effects
                      │  Element<M>  │
                      └──────┬───────┘
                             │
                             ▼
                        ┌─────────┐
                        │ Render  │  (iced runtime)
                        └─────────┘
                             │
                             ▼
                      ┌──────────────┐
                      │   Display    │
                      └──────────────┘
```

## Component Breakdown

### 1. Message Enum (message.rs)

Defines all possible user actions and system events.

```rust
#[derive(Debug, Clone)]
pub enum Message {
    CanvasMouseDown { x: usize, y: usize },
    CanvasMouseMove { x: usize, y: usize },
    CanvasMouseUp,
    ClearCanvas,
    PredictDigit,
    PredictionComplete(PredictionResult),
}

#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub digit: usize,
    pub confidence: f32,
    pub all_scores: Vec<f32>,
}
```

**Design Notes:**
- `Clone` required by iced (messages passed through event system)
- Separate mouse down/move/up for brush-like drawing
- `PredictionComplete` allows async prediction (future: offload to thread)

### 2. Model (State) Struct (app.rs)

Holds all application state.

```rust
pub struct MnistPainter {
    canvas: [u8; 28 * 28],
    is_drawing: bool,
    prediction: Option<PredictionResult>,
    model: Sequential,
    brush_size: BrushSize,
}

pub enum BrushSize {
    Small,   // 1x1
    Medium,  // 2x2
    Large,   // 3x3
}
```

**Ownership Notes:**
- `Sequential` is owned by the app (moved from main)
- `canvas` is stack-allocated array (784 bytes, not heap)
- `is_drawing` tracks mouse state across events

### 3. Application Trait Implementation (app.rs)

Core iced integration.

```rust
impl Application for MnistPainter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Sequential;  // Model passed during initialization

    fn new(model: Sequential) -> (Self, Command<Message>) {
        (
            Self {
                canvas: [0; 28 * 28],
                is_drawing: false,
                prediction: None,
                model,
                brush_size: BrushSize::Medium,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("MNIST Digit Painter - Neural Network from Scratch")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CanvasMouseDown { x, y } => {
                self.is_drawing = true;
                self.paint_pixel(x, y);
                Command::none()
            }
            Message::CanvasMouseMove { x, y } => {
                if self.is_drawing {
                    self.paint_pixel(x, y);
                }
                Command::none()
            }
            Message::CanvasMouseUp => {
                self.is_drawing = false;
                Command::none()
            }
            Message::ClearCanvas => {
                self.canvas.fill(0);
                self.prediction = None;
                Command::none()
            }
            Message::PredictDigit => {
                let input = self.canvas_to_network_input();
                let output = self.model.forward(&input);
                self.prediction = Some(extract_prediction(&output));
                Command::none()
            }
            Message::PredictionComplete(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let canvas_widget = Canvas::new(PixelCanvas {
            pixels: &self.canvas,
        })
        .width(Length::Fixed(280.0))
        .height(Length::Fixed(280.0));

        let controls = row![
            button("Clear").on_press(Message::ClearCanvas),
            button("Predict").on_press(Message::PredictDigit),
        ]
        .spacing(10);

        let prediction_display = match &self.prediction {
            None => column![text("Draw a digit (0-9) and click Predict")],
            Some(pred) => column![
                text(format!("Predicted Digit: {}", pred.digit))
                    .size(32),
                text(format!("Confidence: {:.1}%", pred.confidence * 100.0))
                    .size(20),
                confidence_bars(&pred.all_scores),
            ],
        };

        column![
            canvas_widget,
            controls,
            prediction_display,
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
```

### 4. Canvas Widget (canvas.rs)

Custom iced `Canvas` implementation for drawing.

```rust
use iced::widget::canvas::{Cache, Cursor, Event, Frame, Geometry, Path, Program};
use iced::{Color, Point, Rectangle, Size, mouse};

pub struct PixelCanvas<'a> {
    pub pixels: &'a [u8; 28 * 28],
}

impl<'a> Program<Message> for PixelCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let pixel_size = bounds.width / 28.0;

        for y in 0..28 {
            for x in 0..28 {
                let pixel_value = self.pixels[y * 28 + x];

                let gray = pixel_value as f32 / 255.0;
                let color = Color::from_rgb(gray, gray, gray);

                let rect = Path::rectangle(
                    Point::new(x as f32 * pixel_size, y as f32 * pixel_size),
                    Size::new(pixel_size, pixel_size),
                );

                frame.fill(&rect, color);
            }
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if let Some(position) = cursor.position_in(&bounds) {
            let pixel_size = bounds.width / 28.0;
            let x = (position.x / pixel_size) as usize;
            let y = (position.y / pixel_size) as usize;

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    return (
                        event::Status::Captured,
                        Some(Message::CanvasMouseDown { x, y }),
                    );
                }
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    return (
                        event::Status::Captured,
                        Some(Message::CanvasMouseMove { x, y }),
                    );
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    return (
                        event::Status::Captured,
                        Some(Message::CanvasMouseUp),
                    );
                }
                _ => {}
            }
        }

        (event::Status::Ignored, None)
    }
}
```

**Key Design Decisions:**
- Scale factor: 28px → 280px (10x zoom for usability)
- Grayscale rendering: `pixel_value / 255.0` → RGB(gray, gray, gray)
- Event → Message conversion happens in canvas, not main app

### 5. Helper Functions (app.rs)

```rust
impl MnistPainter {
    fn paint_pixel(&mut self, x: usize, y: usize) {
        if x >= 28 || y >= 28 {
            return;
        }

        match self.brush_size {
            BrushSize::Small => {
                self.canvas[y * 28 + x] = 255;
            }
            BrushSize::Medium => {
                for dy in 0..2 {
                    for dx in 0..2 {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx < 28 && ny < 28 {
                            self.canvas[ny * 28 + nx] = 255;
                        }
                    }
                }
            }
            BrushSize::Large => {
                for dy in 0..3 {
                    for dx in 0..3 {
                        let nx = x.saturating_sub(1) + dx;
                        let ny = y.saturating_sub(1) + dy;
                        if nx < 28 && ny < 28 {
                            self.canvas[ny * 28 + nx] = 255;
                        }
                    }
                }
            }
        }
    }

    fn canvas_to_network_input(&self) -> Array2<f32> {
        let normalized: Vec<f32> = self
            .canvas
            .iter()
            .map(|&pixel| pixel as f32 / 255.0)
            .collect();

        Array2::from_shape_vec((1, 784), normalized)
            .expect("Canvas shape mismatch")
    }
}

fn extract_prediction(output: &Array2<f32>) -> PredictionResult {
    let scores: Vec<f32> = output.row(0).to_vec();

    let (digit, &confidence) = scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    PredictionResult {
        digit,
        confidence,
        all_scores: scores,
    }
}

fn confidence_bars(scores: &[f32]) -> Element<'static, Message> {
    let bars: Vec<Element<Message>> = scores
        .iter()
        .enumerate()
        .map(|(digit, &score)| {
            row![
                text(format!("{}: ", digit)).width(Length::Fixed(30.0)),
                progress_bar(0.0..=1.0, score).width(Length::Fixed(200.0)),
                text(format!(" {:.1}%", score * 100.0)),
            ]
            .into()
        })
        .collect();

    column(bars).spacing(5).into()
}
```

### 6. Main Entry Point (examples/mnist_painter.rs)

```rust
use nn_application::{load_mnist, Trainer, TrainerConfig};
use nn_core::{
    activation::{ReLU, Softmax},
    activation_layer::ActivationLayer,
    layer::DenseLayer,
    loss::CrossEntropy,
    sequential::Sequential,
};
use nn_infrastructure::painter::MnistPainter;

fn main() -> iced::Result {
    println!("Loading MNIST dataset...");
    let dataset = load_mnist().expect("Failed to load MNIST dataset");

    println!("Building network: 784 → 128 → 64 → 10");
    let mut model = Sequential::new();
    model.add(Box::new(DenseLayer::new(784, 128)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(128, 64)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(64, 10)));
    model.add(Box::new(ActivationLayer::new(Softmax)));

    println!("Training network (15 epochs)...");
    let config = TrainerConfig {
        epochs: 15,
        learning_rate: 0.01,
        batch_size: 64,
    };
    let trainer = Trainer::new(config, CrossEntropy);
    let metrics = trainer
        .train(&mut model, &dataset.train_images, &dataset.train_labels)
        .expect("Training failed");

    let final_accuracy = metrics.last().unwrap().accuracy;
    println!("Training complete. Final accuracy: {:.2}%", final_accuracy * 100.0);

    println!("Launching painter...");
    MnistPainter::run(iced::Settings::with_flags(model))
}
```

## Cargo.toml (infrastructure/Cargo.toml)

```toml
[package]
name = "nn-infrastructure"
version = "0.1.0"
edition = "2021"

[dependencies]
nn-core = { path = "../core" }
nn-application = { path = "../application" }
ndarray = { workspace = true }
iced = { version = "0.12", features = ["canvas", "tokio"] }
anyhow = { workspace = true }

[[example]]
name = "mnist_painter"
path = "examples/mnist_painter.rs"
```

## C# Developer Mental Model

### iced Elm Architecture vs. WPF MVVM

| Elm (iced) | WPF MVVM | Explanation |
|------------|----------|-------------|
| `Model` struct | ViewModel | Holds all UI state |
| `Message` enum | `ICommand` + Events | User actions |
| `update()` | Property setters + `PropertyChanged` | State transitions |
| `view()` | XAML bindings | Declarative UI |
| `Command` | `async Task` | Asynchronous operations |

### Key Differences from C#

**1. No PropertyChanged Events**
```csharp
// C# MVVM
public class ViewModel : INotifyPropertyChanged {
    private int _count;
    public int Count {
        get => _count;
        set {
            _count = value;
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(nameof(Count)));
        }
    }
}
```

```rust
// Rust iced - No events, just rebuild view
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::Increment => {
            self.count += 1;  // Direct mutation
            Command::none()    // view() will be called automatically
        }
    }
}
```

**2. Immutable Message Pattern**
```csharp
// C# - Mutable event args
button.Click += (s, e) => { viewModel.Count++; };
```

```rust
// Rust - Immutable message
button("Increment").on_press(Message::Increment)
// Message is Clone, passed by value, no shared mutation
```

**3. Ownership of Model**
```csharp
// C# - Model lives in trainer, referenced by UI
var trainer = new Trainer(model);
var window = new Window { DataContext = trainer };
```

```rust
// Rust - Model is moved into UI, owned by app
let model = train_model();
MnistPainter::run(Settings::with_flags(model))
// model is now owned by MnistPainter, trainer can't access it
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given__empty_canvas__when__paint_pixel__then__sets_pixel_to_white() {
        let mut app = create_test_app();
        app.paint_pixel(5, 5);
        assert_eq!(app.canvas[5 * 28 + 5], 255);
    }

    #[test]
    fn given__painted_canvas__when__clear__then__resets_to_zero() {
        let mut app = create_test_app();
        app.canvas[0] = 255;
        app.update(Message::ClearCanvas);
        assert_eq!(app.canvas[0], 0);
    }

    #[test]
    fn given__canvas__when__convert_to_network_input__then__normalizes_correctly() {
        let mut app = create_test_app();
        app.canvas[0] = 255;
        app.canvas[1] = 128;

        let input = app.canvas_to_network_input();

        assert!((input[[0, 0]] - 1.0).abs() < 1e-6);
        assert!((input[[0, 1]] - 0.5019).abs() < 0.01);
    }
}
```

### Integration Test
No iced UI testing (too complex), but validate pipeline:
```rust
#[test]
fn given__trained_model_and_mnist_digit__when__predict__then__returns_correct_digit() {
    let model = load_trained_model();
    let mnist_seven = load_mnist_sample(7); // Actual MNIST image

    let prediction = predict(&model, &mnist_seven);

    assert_eq!(prediction.digit, 7);
    assert!(prediction.confidence > 0.8);
}
```

## Future Enhancements

### Phase 2 Features
- [ ] Brush size selector (Small/Medium/Large radio buttons)
- [ ] Confidence bar chart for all 10 digits
- [ ] Save/Load canvas as PNG
- [ ] Undo/Redo (stack of canvas states)

### Phase 3 Features
- [ ] Model serialization (save/load weights to disk)
- [ ] Training UI (epochs slider, real-time loss graph)
- [ ] Export model to ONNX for production deployment
- [ ] Batch predict mode (load multiple images)

## Build and Run

```bash
# Build infrastructure package
cargo build --package nn-infrastructure

# Run the painter (trains model first, ~2-5 min)
cargo run --package nn-infrastructure --example mnist_painter

# For faster iteration (skip full training, use subset)
# TODO: Add example that loads pre-trained weights
```

## Key Takeaways for C# Developers

1. **Elm is MVVM without events** - `update()` is called automatically when messages arrive
2. **Messages are immutable** - No shared mutable state between UI and logic
3. **View is pure** - `view()` has no side effects, just builds UI tree from current state
4. **Ownership matters** - Model is moved into app, not shared with trainer
5. **No async by default** - Use `Command` for async operations (network, file I/O)

This architecture maintains clean separation like your core/application split, extends it to the UI layer with type-safe messages, and provides a foundation for future features.
