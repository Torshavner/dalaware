use iced::widget::canvas::{self, Cache, Event, Geometry, Path, Program};
use iced::widget::{button, Canvas, column, container, pick_list, progress_bar, row, slider, text, text_input, Container, Row};
use iced::{mouse, Alignment, Color, Element, Fill, Length, Point, Rectangle, Renderer, Size, Subscription, Task, Theme};
use ndarray::Array2;
use nn_application::{
    ActivationType, LayerConfig, MnistDataset, NetworkBuilder, NetworkConfig,
    PresetType, TrainingState,
};
use nn_core::loss::CrossEntropy;
use nn_core::sequential::Sequential;
use nn_core::Module;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

mod theme;

// Configuration constants
const MAX_HIDDEN_LAYERS: usize = 6;
const MIN_NEURONS_PER_LAYER: u8 = 1;
const MAX_NEURONS_PER_LAYER: u8 = 128;
const DEFAULT_NEURON_COUNT: usize = 64;

// Spacing constants
const SPACE_SM: f32 = 8.0; // Standard spacing (form controls, column items)
const SPACE_MD: f32 = 10.0; // Medium spacing (button groups)
const SPACE_LG: f32 = 20.0; // Large spacing (major sections)

const PADDING_CONTROL: f32 = 12.0; // Inside cards
const PADDING_PANEL: f32 = 20.0; // Main panels

fn main() -> iced::Result {
    nn_infrastructure::init_tracing().expect("Failed to initialize tracing");

    tracing::info!("===============================================");
    tracing::info!("Neural Network Playground - MNIST Edition");
    tracing::info!("===============================================");

    // Load MNIST dataset
    let dataset = nn_application::load_mnist_subset(1000, 200).expect("Failed to load MNIST");

    iced::application(
        "Neural Network Playground - MNIST",
        PlaygroundApp::update,
        PlaygroundApp::view,
    )
        .subscription(PlaygroundApp::subscription)
        .window_size(iced::Size::new(1300.0, 700.0))
        .run_with(move || PlaygroundApp::new(dataset))
}

pub struct AppFlags {
    pub dataset: MnistDataset,
}

pub enum BrushSize {
    Small,
    Medium,
    Large,
}

struct PlaygroundApp {
    // Network configuration
    layers: Vec<LayerConfig>,
    global_activation: ActivationType,
    selected_preset: Option<PresetType>,

    // Training configuration
    epochs_input: String,
    learning_rate_input: String,
    batch_size_input: String,

    // Model and dataset
    model: Option<Arc<Mutex<Sequential>>>,
    dataset: MnistDataset,
    training_state: TrainingState,

    // Canvas for drawing
    canvas: [u8; 28 * 28],
    canvas_cache: Cache,
    is_drawing: bool,
    brush_size: BrushSize,

    // Prediction
    prediction: Option<PredictionResult>,

    // Training results
    training_result: Option<TrainingResult>,
    current_epoch: usize,
    current_epoch_accuracy: f32,
    current_epoch_loss: f32,
    training_receiver: Option<mpsc::UnboundedReceiver<TrainingProgress>>,

    // Error state
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
enum TrainingProgress {
    EpochComplete { epoch: usize, accuracy: f32, loss: f32 },
    Complete(TrainingResult),
}

#[derive(Clone)]
struct PredictionResult {
    digit: usize,
    confidence: f32,
    all_scores: Vec<f32>,
}

struct TrainingResult {
    final_accuracy: f32,
    final_loss: f32,
    model: Arc<Mutex<Sequential>>,
}

impl Clone for TrainingResult {
    fn clone(&self) -> Self {
        Self {
            final_accuracy: self.final_accuracy,
            final_loss: self.final_loss,
            model: Arc::clone(&self.model),
        }
    }
}

impl std::fmt::Debug for TrainingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrainingResult")
            .field("final_accuracy", &self.final_accuracy)
            .field("final_loss", &self.final_loss)
            .field("model", &"<Sequential>")
            .finish()
    }
}

#[derive(Debug, Clone)]
enum Message {
    // Layer management
    AddLayer,
    RemoveLayer,
    ResetNetwork,
    ChangeLayerSize { layer: usize, size: usize },

    // Activation
    GlobalActivationChanged(ActivationType),

    // Presets
    LoadPreset(PresetType),

    // Training
    TrainModel,
    TrainingEpochComplete,
    EpochsChanged(String),
    LearningRateChanged(String),
    BatchSizeChanged(String),

    // Canvas interaction
    CanvasMouseDown { x: usize, y: usize },
    CanvasMouseMove { x: usize, y: usize },
    CanvasMouseUp,
    ClearCanvas,

    // Prediction
    PredictDigit,

    // Error handling
    DismissError,
}

// ===== Helper Functions =====

/// Create a titled card with themed styling
fn control_card<'a>(title: &'a str, content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    container(
        column![
            text(title).size(18).style(theme::text_primary()),
            content.into()
        ]
        .spacing(SPACE_SM)
    )
    .padding(PADDING_CONTROL)
    .style(theme::container_card())
}

/// Create a parameter row with label and input
fn parameter_row<'a>(
    label: &'a str,
    input: Element<'a, Message>
) -> Row<'a, Message> {
    row![
        text(label).size(14),
        input
    ]
    .spacing(SPACE_MD)
    .align_y(Alignment::Center)
}

impl PlaygroundApp {
    fn new(dataset: MnistDataset) -> (Self, Task<Message>) {
        let app = Self {
            layers: vec![
                LayerConfig::new(128, ActivationType::ReLU),
                LayerConfig::new(64, ActivationType::ReLU),
            ],
            global_activation: ActivationType::ReLU,
            selected_preset: None,
            epochs_input: String::from("5"),
            learning_rate_input: String::from("0.01"),
            batch_size_input: String::from("64"),
            model: None,
            dataset,
            training_state: TrainingState::Idle,
            canvas: [0; 28 * 28],
            canvas_cache: Cache::default(),
            is_drawing: false,
            brush_size: BrushSize::Medium,
            prediction: None,
            training_result: None,
            current_epoch: 0,
            current_epoch_accuracy: 0.0,
            current_epoch_loss: 0.0,
            training_receiver: None,
            error_message: None,
        };

        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddLayer => {
                if self.layers.len() < MAX_HIDDEN_LAYERS {
                    self.layers
                        .push(LayerConfig::new(DEFAULT_NEURON_COUNT, self.global_activation));
                }
            }

            Message::RemoveLayer => {
                if !self.layers.is_empty() {
                    self.layers.pop();
                }
            }

            Message::ResetNetwork => {
                self.layers = vec![
                    LayerConfig::new(128, ActivationType::ReLU),
                    LayerConfig::new(64, ActivationType::ReLU),
                ];
                self.global_activation = ActivationType::ReLU;
                self.selected_preset = None;
                self.model = None;
                self.training_result = None;
            }

            Message::ChangeLayerSize { layer, size } => {
                if let Some(layer_config) = self.layers.get_mut(layer) {
                    layer_config.neurons = size;
                }
            }

            Message::GlobalActivationChanged(activation) => {
                self.global_activation = activation;
                for layer in &mut self.layers {
                    layer.activation = activation;
                }
            }

            Message::LoadPreset(preset) => {
                self.layers = match preset {
                    PresetType::Shallow => vec![
                        LayerConfig::new(128, self.global_activation)
                    ],
                    PresetType::Deep => vec![
                        LayerConfig::new(64, self.global_activation),
                        LayerConfig::new(64, self.global_activation),
                        LayerConfig::new(64, self.global_activation),
                        LayerConfig::new(64, self.global_activation),
                    ],
                    PresetType::Pyramid => vec![
                        LayerConfig::new(128, self.global_activation),
                        LayerConfig::new(96, self.global_activation),
                        LayerConfig::new(64, self.global_activation),
                        LayerConfig::new(32, self.global_activation),
                    ],
                    PresetType::Wide => vec![
                        LayerConfig::new(256, self.global_activation),
                        LayerConfig::new(256, self.global_activation),
                    ],
                };
                self.selected_preset = Some(preset);
            }

            Message::TrainModel => {
                if self.training_state == TrainingState::Idle {
                    self.training_state = TrainingState::Running;
                    self.training_result = None;
                    self.current_epoch = 0;
                    self.current_epoch_accuracy = 0.0;
                    self.current_epoch_loss = 0.0;
                    self.error_message = None;

                    let epochs = self.epochs_input.parse().unwrap_or(5);
                    let learning_rate = self.learning_rate_input.parse().unwrap_or(0.01);
                    let batch_size = self.batch_size_input.parse().unwrap_or(64);

                    let mut config = NetworkConfig::new(784, self.layers.clone());
                    config.output_size = 10; // MNIST has 10 classes

                    // Create channel for progress updates
                    let (tx, rx) = mpsc::unbounded_channel();
                    self.training_receiver = Some(rx);

                    // Spawn training task
                    tokio::spawn(train_model_with_progress(
                        config,
                        self.dataset.clone(),
                        epochs,
                        learning_rate,
                        batch_size,
                        tx,
                    ));
                }
            }

            Message::TrainingEpochComplete => {
                // Check the channel for updates
                let mut completed = false;
                let mut result_opt = None;

                if let Some(rx) = &mut self.training_receiver {
                    while let Ok(progress) = rx.try_recv() {
                        match progress {
                            TrainingProgress::EpochComplete { epoch, accuracy, loss } => {
                                self.current_epoch = epoch;
                                self.current_epoch_accuracy = accuracy;
                                self.current_epoch_loss = loss;
                                tracing::info!(
                                    epoch = epoch,
                                    accuracy = accuracy,
                                    loss = loss,
                                    "Epoch completed"
                                );
                            }
                            TrainingProgress::Complete(result) => {
                                completed = true;
                                result_opt = Some(result);
                            }
                        }
                    }
                }

                if completed {
                    if let Some(result) = result_opt {
                        self.training_state = TrainingState::Idle;
                        self.model = Some(result.model.clone());
                        self.training_result = Some(result);
                        self.training_receiver = None;
                        self.error_message = None;
                        tracing::info!("Training completed successfully");
                    }
                }
            }

            Message::EpochsChanged(value) => {
                self.epochs_input = value;
            }

            Message::LearningRateChanged(value) => {
                self.learning_rate_input = value;
            }

            Message::BatchSizeChanged(value) => {
                self.batch_size_input = value;
            }

            Message::CanvasMouseDown { x, y } => {
                self.is_drawing = true;
                self.paint_pixel(x, y);
                self.canvas_cache.clear();
            }

            Message::CanvasMouseMove { x, y } => {
                if self.is_drawing {
                    self.paint_pixel(x, y);
                    self.canvas_cache.clear();
                }
            }

            Message::CanvasMouseUp => {
                self.is_drawing = false;
            }

            Message::ClearCanvas => {
                self.canvas.fill(0);
                self.prediction = None;
                self.canvas_cache.clear();
            }

            Message::PredictDigit => {
                let input = self.canvas_to_network_input();
                if let Some(model_arc) = &self.model {
                    if let Ok(mut model) = model_arc.lock() {
                        let output = model.forward(&input);
                        self.prediction = Some(extract_prediction(&output));
                    }
                }
            }

            Message::DismissError => {
                self.error_message = None;
            }
        }

        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // Poll for training progress updates
        if self.training_state == TrainingState::Running {
            iced::time::every(std::time::Duration::from_millis(100))
                .map(|_| Message::TrainingEpochComplete)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut content_items = vec![];

        // Add error banner if there's an error
        if let Some(ref error) = self.error_message {
            let error_banner = container(
                row![
                    text(format!("Error: {}", error)).size(13).style(theme::text_primary()),
                    button(text("✕")).on_press(Message::DismissError).style(theme::button_ghost()),
                ]
                    .spacing(SPACE_MD)
                    .align_y(Alignment::Center),
            )
                .padding(PADDING_CONTROL)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(0.50, 0.50, 0.50, 0.12))),
                    text_color: Some(theme::TEXT_PRIMARY),
                    border: iced::Border {
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                        width: 1.0,
                        radius: theme::BORDER_RADIUS.into(),
                    },
                    ..Default::default()
                });
            content_items.push(error_banner.into());
        }

        let main_content = row![
            self.network_config_panel(),
            self.training_panel(),
            self.canvas_panel(),
        ]
            .spacing(SPACE_LG);

        content_items.push(main_content.into());

        let content = column(content_items).spacing(SPACE_LG).padding(PADDING_PANEL);

        container(content)
            .width(Fill)
            .height(Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(theme::BG_DARK)),
                text_color: Some(theme::TEXT_PRIMARY),
                ..Default::default()
            })
            .into()
    }

    fn network_config_panel(&self) -> Element<'_, Message> {
        let architecture = format!(
            "784 → [{}] → 10",
            self.layers
                .iter()
                .map(|l| l.neurons.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let config = NetworkConfig::new(784, self.layers.clone());
        let total_params = config.total_parameters();

        control_card(
            "Network Architecture",
            column![
                text(architecture).size(14).style(theme::text_secondary()),
                text(format!("Total parameters: {}", total_params)).size(12).style(theme::text_secondary()),
                text("Presets").size(16).style(theme::text_primary()),
                row![
                    button(text("Shallow")).on_press(Message::LoadPreset(PresetType::Shallow)).style(theme::button_secondary()),
                    button(text("Deep")).on_press(Message::LoadPreset(PresetType::Deep)).style(theme::button_secondary()),
                    button(text("Pyramid")).on_press(Message::LoadPreset(PresetType::Pyramid)).style(theme::button_secondary()),
                    button(text("Wide")).on_press(Message::LoadPreset(PresetType::Wide)).style(theme::button_secondary()),
                ]
                .spacing(SPACE_SM),
                text("Layer Management").size(16).style(theme::text_primary()),
                row![
                    button(text("+ Add")).on_press_maybe(
                        if self.layers.len() < MAX_HIDDEN_LAYERS {
                            Some(Message::AddLayer)
                        } else {
                            None
                        }
                    ).style(theme::button_primary()),
                    button(text("- Remove")).on_press_maybe(if !self.layers.is_empty() {
                        Some(Message::RemoveLayer)
                    } else {
                        None
                    }).style(theme::button_danger()),
                    button(text("Reset")).on_press(Message::ResetNetwork).style(theme::button_secondary()),
                ]
                .spacing(SPACE_SM),
                text("Layers").size(16).style(theme::text_primary()),
                column(
                    self.layers
                        .iter()
                        .enumerate()
                        .map(|(idx, layer)| {
                            row![
                                text(format!("L{}", idx + 1)).size(12).width(Length::Fixed(30.0)),
                                slider(
                                    MIN_NEURONS_PER_LAYER..=MAX_NEURONS_PER_LAYER,
                                    layer.neurons as u8,
                                    move |n| {
                                        Message::ChangeLayerSize {
                                            layer: idx,
                                            size: n as usize
                                        }
                                    }
                                ).width(Fill),
                                text(format!("{}", layer.neurons)).size(12).width(Length::Fixed(40.0)),
                            ]
                            .spacing(SPACE_MD)
                            .align_y(Alignment::Center)
                            .into()
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(SPACE_SM),
                text("Activation").size(16).style(theme::text_primary()),
                pick_list(
                    &ActivationType::ALL[..],
                    Some(self.global_activation),
                    Message::GlobalActivationChanged
                ),
            ]
                .spacing(SPACE_LG)
        )
            .width(Length::Fixed(320.0))
            .into()
    }

    fn training_panel(&self) -> Element<'_, Message> {
        let status_element = match self.training_state {
            TrainingState::Idle => {
                if let Some(result) = &self.training_result {
                    text(format!(
                        "Training Complete\nAccuracy: {:.2}%\nLoss: {:.4}",
                        result.final_accuracy * 100.0,
                        result.final_loss
                    )).size(14).style(theme::text_primary())
                } else {
                    text("Configure and train your network").size(14).style(theme::text_secondary())
                }
            }
            TrainingState::Running => {
                if self.current_epoch > 0 {
                    text(format!(
                        "Training in progress\nEpoch: {}\nAccuracy: {:.2}%\nLoss: {:.4}",
                        self.current_epoch,
                        self.current_epoch_accuracy * 100.0,
                        self.current_epoch_loss
                    )).size(14).style(theme::text_primary())
                } else {
                    text("Training in progress...").size(14).style(theme::text_secondary())
                }
            }
            TrainingState::Paused => text("Training paused").size(14).style(theme::text_secondary()),
        };

        control_card(
            "Training Configuration",
            column![
                status_element,
                parameter_row(
                    "Epochs:",
                    text_input("5", &self.epochs_input)
                        .on_input(Message::EpochsChanged)
                        .width(Length::Fixed(100.0))
                        .into()
                ),
                parameter_row(
                    "Learning Rate:",
                    text_input("0.01", &self.learning_rate_input)
                        .on_input(Message::LearningRateChanged)
                        .width(Length::Fixed(100.0))
                        .into()
                ),
                parameter_row(
                    "Batch Size:",
                    text_input("64", &self.batch_size_input)
                        .on_input(Message::BatchSizeChanged)
                        .width(Length::Fixed(100.0))
                        .into()
                ),
                button(text(if self.training_state == TrainingState::Running {
                    "Training..."
                } else if self.model.is_some() {
                    "Retrain"
                } else {
                    "Train Model"
                }))
                .on_press_maybe(if self.training_state == TrainingState::Idle {
                    Some(Message::TrainModel)
                } else {
                    None
                })
                .style(theme::button_success())
                .width(Fill),
            ]
                .spacing(SPACE_LG)
        )
            .width(Length::Fixed(320.0))
            .into()
    }

    fn canvas_panel(&self) -> Element<'_, Message> {
        let canvas_widget = Canvas::new(PixelCanvas {
            pixels: &self.canvas,
            cache: &self.canvas_cache,
        })
            .width(Length::Fixed(360.0))
            .height(Length::Fixed(360.0));

        // Always create histogram bars, showing placeholder when no prediction
        let bars: Vec<Element<Message>> = (0..10)
            .map(|digit| {
                let (score, percentage_text) = match &self.prediction {
                    Some(pred) => (pred.all_scores[digit], format!("{:.1}%", pred.all_scores[digit] * 100.0)),
                    None => (0.0, "—".to_string()),
                };

                row![
                    text(format!("{}", digit)).width(Length::Fixed(20.0)).style(theme::text_secondary()),
                    progress_bar(0.0..=1.0, score)
                        .width(Length::Fixed(150.0))
                        .height(Length::Fixed(18.0)),
                    text(format!("{}", percentage_text)).width(Length::Fixed(50.0)).style(theme::text_secondary()),
                ]
                    .spacing(SPACE_SM)
                    .align_y(Alignment::Center)
                    .into()
            })
            .collect();

        // Prediction header
        let prediction_header = match &self.prediction {
            None => column![
                text("—").size(32).style(theme::text_secondary()),
                text("—").size(14).style(theme::text_tertiary()),
            ],
            Some(pred) => column![
                text(format!("{}", pred.digit)).size(32).style(theme::text_primary()),
                text(format!("{:.1}%", pred.confidence * 100.0)).size(14).style(theme::text_secondary()),
            ],
        };

        // Canvas and controls on the left
        let canvas_section = control_card(
            "Draw a Digit",
            column![
                canvas_widget,
                row![
                    button(text("Clear")).on_press(Message::ClearCanvas).style(theme::button_secondary()).width(Length::Fixed(100.0)),
                    button(text(if self.model.is_some() {
                        "Predict"
                    } else {
                        "Train First"
                    }))
                    .on_press_maybe(if self.model.is_some() {
                        Some(Message::PredictDigit)
                    } else {
                        None
                    })
                    .style(theme::button_primary())
                    .width(Fill),
                ]
                .spacing(SPACE_MD),
            ]
                .spacing(SPACE_LG)
                .align_x(Alignment::Center)
        );

        // Histogram on the right
        let histogram_section = control_card(
            "Prediction",
            column![
                prediction_header,
                text("").size(SPACE_LG), // Spacer
                column(bars).spacing(SPACE_SM),
            ]
                .spacing(SPACE_LG)
        )
            .width(Length::Fixed(280.0));

        // Side by side layout
        row![canvas_section, histogram_section]
            .spacing(SPACE_LG)
            .align_y(Alignment::Start)
            .into()
    }

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
                        let nx = x.saturating_add(dx);
                        let ny = y.saturating_add(dy);
                        if nx < 28 && ny < 28 {
                            self.canvas[ny * 28 + nx] = 255;
                        }
                    }
                }
            }
            BrushSize::Large => {
                for dy in 0..3 {
                    for dx in 0..3 {
                        let nx = x.saturating_sub(1).saturating_add(dx);
                        let ny = y.saturating_sub(1).saturating_add(dy);
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
            .map(|&pixel| f32::from(pixel) / 255.0)
            .collect();

        Array2::from_shape_vec((1, 784), normalized).expect("Canvas shape mismatch")
    }
}

fn extract_prediction(output: &Array2<f32>) -> PredictionResult {
    let scores: Vec<f32> = output.row(0).to_vec();

    let (digit, &confidence) = scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((0, &0.0));

    PredictionResult {
        digit,
        confidence,
        all_scores: scores,
    }
}

async fn train_model_with_progress(
    mut config: NetworkConfig,
    dataset: MnistDataset,
    epochs: usize,
    learning_rate: f32,
    batch_size: usize,
    tx: mpsc::UnboundedSender<TrainingProgress>,
) {
    tokio::task::spawn_blocking(move || {
        config.output_size = 10; // MNIST has 10 classes
        let mut model = NetworkBuilder::build(&config).expect("Failed to build network");

        // Custom training loop to send real-time updates
        use nn_core::Module;
        use nn_core::loss::Loss;
        let loss_fn = CrossEntropy;

        for epoch in 0..epochs {
            // Train one epoch
            let num_samples = dataset.train_images.nrows();
            let mut indices: Vec<usize> = (0..num_samples).collect();

            // Shuffle indices for SGD
            use rand::seq::SliceRandom;
            use rand::thread_rng;
            indices.shuffle(&mut thread_rng());

            // Mini-batch training
            for batch_start in (0..num_samples).step_by(batch_size) {
                let batch_end = (batch_start + batch_size).min(num_samples);
                let batch_indices = &indices[batch_start..batch_end];

                // Get batch data
                let batch_inputs = dataset.train_images.select(ndarray::Axis(0), batch_indices);
                let batch_targets = dataset.train_labels.select(ndarray::Axis(0), batch_indices);

                // Forward pass
                let outputs = model.forward(&batch_inputs);

                // Backward pass
                let grad = loss_fn.gradient(&outputs, &batch_targets);
                model.backward(&grad);

                // Update parameters
                model.update_parameters(learning_rate);
            }

            // Evaluate on test set
            let outputs = model.forward(&dataset.test_images);
            let loss = loss_fn.calculate(&outputs, &dataset.test_labels);

            // Calculate accuracy
            let predictions = outputs.map_axis(ndarray::Axis(1), |row| {
                row.iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(idx, _)| idx)
                    .unwrap()
            });

            let targets = dataset.test_labels.map_axis(ndarray::Axis(1), |row| {
                row.iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(idx, _)| idx)
                    .unwrap()
            });

            let correct = predictions.iter()
                .zip(targets.iter())
                .filter(|(pred, target)| pred == target)
                .count();

            let accuracy = correct as f32 / dataset.test_images.nrows() as f32;

            // Send epoch update immediately
            let _ = tx.send(TrainingProgress::EpochComplete {
                epoch: epoch + 1,
                accuracy,
                loss,
            });

            tracing::info!(
                epoch = epoch + 1,
                total_epochs = epochs,
                loss = loss,
                accuracy = accuracy * 100.0,
                "Epoch completed"
            );
        }

        // Send completion
        let outputs = model.forward(&dataset.test_images);
        let loss = CrossEntropy.calculate(&outputs, &dataset.test_labels);

        let predictions = outputs.map_axis(ndarray::Axis(1), |row| {
            row.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap()
        });

        let targets = dataset.test_labels.map_axis(ndarray::Axis(1), |row| {
            row.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap()
        });

        let correct = predictions.iter()
            .zip(targets.iter())
            .filter(|(pred, target)| pred == target)
            .count();

        let accuracy = correct as f32 / dataset.test_images.nrows() as f32;

        let _ = tx.send(TrainingProgress::Complete(TrainingResult {
            final_accuracy: accuracy,
            final_loss: loss,
            model: Arc::new(Mutex::new(model)),
        }));
    });
}

// Canvas implementation for drawing pixels
pub struct PixelCanvas<'a> {
    pub pixels: &'a [u8; 28 * 28],
    pub cache: &'a Cache,
}

impl<'a> Program<Message> for PixelCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let pixel_size = bounds.width / 28.0;

            for y in 0..28 {
                for x in 0..28 {
                    let pixel_value = self.pixels[y * 28 + x];

                    let gray = f32::from(pixel_value) / 255.0;
                    let color = Color::from_rgb(gray, gray, gray);

                    let rect = Path::rectangle(
                        Point::new(x as f32 * pixel_size, y as f32 * pixel_size),
                        Size::new(pixel_size, pixel_size),
                    );

                    frame.fill(&rect, color);
                }
            }

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, bounds.size()),
                canvas::Stroke::default()
                    .with_color(Color::from_rgb(0.5, 0.5, 0.5))
                    .with_width(2.0),
            );
        });

        vec![geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        if let Some(position) = cursor.position_in(bounds) {
            let pixel_size = bounds.width / 28.0;
            let x = (position.x / pixel_size) as usize;
            let y = (position.y / pixel_size) as usize;

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    return (
                        canvas::event::Status::Captured,
                        Some(Message::CanvasMouseDown { x, y }),
                    );
                }
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    return (
                        canvas::event::Status::Captured,
                        Some(Message::CanvasMouseMove { x, y }),
                    );
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    return (
                        canvas::event::Status::Captured,
                        Some(Message::CanvasMouseUp),
                    );
                }
                _ => {}
            }
        }

        (canvas::event::Status::Ignored, None)
    }
}
