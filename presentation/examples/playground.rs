use iced::widget::{button, column, container, pick_list, row, slider, text};
use iced::{Element, Fill, Task};
use nn_application::{ActivationType, FeatureConfig, LayerConfig, NetworkBuilder, NetworkConfig, PresetType, TrainingState};
use nn_core::sequential::Sequential;

// Configuration constants
const MAX_HIDDEN_LAYERS: usize = 6;
const MIN_NEURONS_PER_LAYER: u8 = 1;
const MAX_NEURONS_PER_LAYER: u8 = 8;
const DEFAULT_NEURON_COUNT: usize = 4;
const DEFAULT_FIRST_LAYER_NEURONS: usize = 4;
const DEFAULT_SECOND_LAYER_NEURONS: usize = 2;

fn main() -> iced::Result {
    nn_infrastructure::init_tracing().expect("Failed to initialize tracing");

    tracing::info!("===============================================");
    tracing::info!("Neural Network Playground");
    tracing::info!("===============================================");

    iced::application("Neural Network Playground", PlaygroundApp::update, PlaygroundApp::view)
        .window_size(iced::Size::new(1200.0, 800.0))
        .run_with(PlaygroundApp::new)
}

struct PlaygroundApp {
    // Network configuration
    layers: Vec<LayerConfig>,
    global_activation: ActivationType,
    advanced_mode: bool,
    selected_preset: Option<PresetType>,

    // Features
    features: FeatureConfig,

    // Network
    model: Option<Sequential>,

    // Training state
    training_state: TrainingState,

    // Error state
    error_message: Option<String>,
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
    ToggleAdvancedMode(bool),

    // Presets
    LoadPreset(PresetType),

    // Error handling
    DismissError,
}

impl PlaygroundApp {
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            layers: vec![
                LayerConfig::new(DEFAULT_FIRST_LAYER_NEURONS, ActivationType::ReLU),
                LayerConfig::new(DEFAULT_SECOND_LAYER_NEURONS, ActivationType::ReLU),
            ],
            global_activation: ActivationType::ReLU,
            advanced_mode: false,
            selected_preset: None,
            features: FeatureConfig::default(),
            model: None,
            training_state: TrainingState::Idle,
            error_message: None,
        };

        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddLayer => {
                if self.layers.len() < MAX_HIDDEN_LAYERS {
                    self.layers.push(LayerConfig::new(DEFAULT_NEURON_COUNT, self.global_activation));
                    self.rebuild_network();
                }
            }

            Message::RemoveLayer => {
                if !self.layers.is_empty() {
                    self.layers.pop();
                    self.rebuild_network();
                }
            }

            Message::ResetNetwork => {
                self.layers = vec![
                    LayerConfig::new(DEFAULT_FIRST_LAYER_NEURONS, ActivationType::ReLU),
                    LayerConfig::new(DEFAULT_SECOND_LAYER_NEURONS, ActivationType::ReLU),
                ];
                self.global_activation = ActivationType::ReLU;
                self.advanced_mode = false;
                self.selected_preset = None;
                self.rebuild_network();
            }

            Message::ChangeLayerSize { layer, size } => {
                if let Some(layer_config) = self.layers.get_mut(layer) {
                    layer_config.neurons = size;
                    self.rebuild_network();
                }
            }

            Message::GlobalActivationChanged(activation) => {
                self.global_activation = activation;
                if !self.advanced_mode {
                    for layer in &mut self.layers {
                        layer.activation = activation;
                    }
                    self.rebuild_network();
                }
            }

            Message::ToggleAdvancedMode(enabled) => {
                self.advanced_mode = enabled;
                if !enabled {
                    let global = self.global_activation;
                    for layer in &mut self.layers {
                        layer.activation = global;
                    }
                }
            }

            Message::LoadPreset(preset) => {
                self.layers = preset.layers(self.global_activation);
                self.selected_preset = Some(preset);
                self.advanced_mode = false;
                self.rebuild_network();
            }

            Message::DismissError => {
                self.error_message = None;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let layer_count_text = if self.layers.len() == 1 {
            "1 hidden layer".to_string()
        } else {
            format!("{} hidden layers", self.layers.len())
        };

        let architecture = format!(
            "[{}]",
            self.layers.iter()
                .map(|l| l.neurons.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut content_items = vec![
            text("Neural Network Playground").size(32).into(),
            text("Interactive visualization for understanding neural networks").size(16).into(),
        ];

        // Add error banner if there's an error
        if let Some(ref error) = self.error_message {
            let error_banner = container(
                row![
                    text(format!("⚠️ Error: {}", error)).size(14),
                    button(text("✕")).on_press(Message::DismissError),
                ]
                .spacing(10)
                .padding(10)
            )
            .style(|_theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))),
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                }
            });
            content_items.push(error_banner.into());
        }

        let network_config = column![
            row![
                text(layer_count_text).size(20),
                text(architecture).size(16),
            ].spacing(10),

                text("Architecture Presets").size(18),
                row![
                    button(text(PresetType::Shallow.to_string()))
                        .on_press(Message::LoadPreset(PresetType::Shallow)),
                    button(text(PresetType::Deep.to_string()))
                        .on_press(Message::LoadPreset(PresetType::Deep)),
                    button(text(PresetType::Pyramid.to_string()))
                        .on_press(Message::LoadPreset(PresetType::Pyramid)),
                    button(text(PresetType::Wide.to_string()))
                        .on_press(Message::LoadPreset(PresetType::Wide)),
                ].spacing(8),

                text("Layer Management").size(18),
                row![
                    button(text("+ Add Layer"))
                        .on_press_maybe(if self.layers.len() < MAX_HIDDEN_LAYERS {
                            Some(Message::AddLayer)
                        } else {
                            None
                        }),
                    button(text("- Remove Layer"))
                        .on_press_maybe(if !self.layers.is_empty() {
                            Some(Message::RemoveLayer)
                        } else {
                            None
                        }),
                    button(text("Reset"))
                        .on_press(Message::ResetNetwork),
                ].spacing(8),

                text("Layer Configuration").size(18),
                column(
                    self.layers.iter().enumerate().map(|(idx, layer)| {
                        row![
                            text(format!("Layer {}", idx + 1)).size(14),
                            slider(MIN_NEURONS_PER_LAYER..=MAX_NEURONS_PER_LAYER, layer.neurons as u8, move |n| {
                                Message::ChangeLayerSize { layer: idx, size: n as usize }
                            }),
                            text(format!("{} neurons", layer.neurons)).size(12),
                        ]
                        .spacing(8)
                        .into()
                    }).collect::<Vec<_>>()
                ).spacing(8),

                text("Activation Function").size(18),
                pick_list(
                    &ActivationType::ALL[..],
                    Some(self.global_activation),
                    Message::GlobalActivationChanged
                ),
                text("Output: Softmax (fixed for classification)").size(12),

            text("Network Statistics").size(18),
            self.network_stats_view(),
        ]
        .spacing(16);

        content_items.push(container(network_config).padding(20).into());

        let content = column(content_items)
            .spacing(20)
            .padding(40);

        container(content)
            .width(Fill)
            .height(Fill)
            .into()
    }
    fn rebuild_network(&mut self) {
        let input_size = self.features.count();
        let config = NetworkConfig::new(input_size, self.layers.clone());

        match NetworkBuilder::build(&config) {
            Ok(model) => {
                self.model = Some(model);
                self.error_message = None; // Clear any previous errors
                tracing::info!(
                    architecture = ?self.layers,
                    input_size = input_size,
                    total_params = config.total_parameters(),
                    "Network rebuilt successfully"
                );
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to build network: {}", e));
                tracing::error!(error = ?e, "Failed to rebuild network");
            }
        }
    }

    fn network_stats_view(&self) -> Element<Message> {
        let input_size = self.features.count();
        let config = NetworkConfig::new(input_size, self.layers.clone());
        let total_params = config.total_parameters();
        let memory_kb = (total_params * 4) / 1024;

        let mut warnings = Vec::new();
        if total_params > 100_000 {
            warnings.push("⚠️ May train slowly");
        }
        if self.layers.len() > 4 {
            warnings.push("⚠️ Very deep - may be hard to train");
        }
        if self.layers.iter().any(|l| l.neurons < 3) {
            warnings.push("⚠️ Small layers may underfit");
        }

        let warning_text = if warnings.is_empty() {
            "✓ Configuration looks good".to_string()
        } else {
            warnings.join(", ")
        };

        column![
            text(format!("Input size: {} (based on features)", input_size)).size(14),
            text(format!("Output size: 2 (binary classification)")).size(14),
            text(format!("Total parameters: {}", total_params)).size(14),
            text(format!("Memory: ~{} KB", memory_kb)).size(14),
            text(warning_text).size(14),
        ]
        .spacing(4)
        .into()
    }
}
