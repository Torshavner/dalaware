use iced::widget::canvas::Cache;
use iced::widget::{button, canvas::Canvas, column, container, progress_bar, row, text, text_input};
use iced::{Alignment, Element, Fill, Length, Task};
use nn_application::{MnistDataset, MnistTrainingService, PredictionResult, PredictionService, TrainerConfig, TrainingResult};

use super::canvas::PixelCanvas;
use super::message::Message;

pub enum BrushSize {
    Small,
    Medium,
    Large,
}

pub struct AppFlags {
    pub dataset: MnistDataset,
}

pub struct MnistPainter {
    canvas: [u8; 28 * 28],
    canvas_cache: Cache,
    is_drawing: bool,
    prediction: Option<PredictionResult>,
    service: MnistTrainingService,
    brush_size: BrushSize,
    epochs_input: String,
    learning_rate_input: String,
    batch_size_input: String,
    is_training: bool,
    training_result: Option<TrainingResult>,
}

impl MnistPainter {
    fn new(flags: AppFlags) -> (Self, Task<Message>) {
        (
            Self {
                canvas: [0; 28 * 28],
                canvas_cache: Cache::default(),
                is_drawing: false,
                prediction: None,
                service: MnistTrainingService::new(flags.dataset),
                brush_size: BrushSize::Medium,
                epochs_input: String::from("15"),
                learning_rate_input: String::from("0.01"),
                batch_size_input: String::from("64"),
                is_training: false,
                training_result: None,
            },
            Task::none(),
        )
    }

    #[allow(dead_code)]
    fn title(&self) -> String {
        String::from("MNIST Digit Painter - Neural Network from Scratch")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
            Message::TrainModel => {
                if !self.is_training {
                    self.is_training = true;
                    self.training_result = None;

                    let epochs = self.epochs_input.parse().unwrap_or(15);
                    let learning_rate = self.learning_rate_input.parse().unwrap_or(0.01);
                    let batch_size = self.batch_size_input.parse().unwrap_or(64);

                    let config = TrainerConfig {
                        epochs,
                        learning_rate,
                        batch_size,
                        lr_decay: 0.95,
                    };

                    return Task::perform(
                        train_model_async(self.service.clone(), config),
                        Message::TrainingComplete,
                    );
                }
            }
            Message::TrainingComplete(result) => {
                self.is_training = false;
                self.training_result = Some(result);
            }
            Message::PredictDigit => {
                if self.service.has_trained_model() {
                    let prediction = self.service.predict(&self.canvas);
                    self.prediction = Some(prediction);
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
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let canvas_widget = Canvas::new(PixelCanvas {
            pixels: &self.canvas,
            cache: &self.canvas_cache,
        })
        .width(Length::Fixed(560.0))
        .height(Length::Fixed(560.0));

        let canvas_controls = row![
            button("Clear").on_press(Message::ClearCanvas),
            button(if self.service.has_trained_model() {
                "Predict"
            } else {
                "Train First"
            })
            .on_press_maybe(if self.service.has_trained_model() {
                Some(Message::PredictDigit)
            } else {
                None
            }),
        ]
        .spacing(10);

        let training_controls = column![
            text("Training Parameters").size(20),
            row![
                text("Epochs: ").width(Length::Fixed(120.0)),
                text_input("15", &self.epochs_input)
                    .on_input(Message::EpochsChanged)
                    .width(Length::Fixed(100.0)),
            ]
            .spacing(10),
            row![
                text("Learning Rate: ").width(Length::Fixed(120.0)),
                text_input("0.01", &self.learning_rate_input)
                    .on_input(Message::LearningRateChanged)
                    .width(Length::Fixed(100.0)),
            ]
            .spacing(10),
            row![
                text("Batch Size: ").width(Length::Fixed(120.0)),
                text_input("64", &self.batch_size_input)
                    .on_input(Message::BatchSizeChanged)
                    .width(Length::Fixed(100.0)),
            ]
            .spacing(10),
            button(if self.is_training {
                "Training..."
            } else if self.service.has_trained_model() {
                "Retrain Model"
            } else {
                "Train Model"
            })
            .on_press_maybe(if !self.is_training {
                Some(Message::TrainModel)
            } else {
                None
            }),
        ]
        .spacing(10);

        let status_text = if self.is_training {
            column![text("Training in progress...").size(16)]
        } else if let Some(result) = &self.training_result {
            column![
                text(format!(
                    "Training Complete! Accuracy: {:.2}%",
                    result.final_accuracy * 100.0
                ))
                .size(16),
                text(format!("Final Loss: {:.4}", result.final_loss)).size(14),
            ]
        } else if !self.service.has_trained_model() {
            column![text("Configure parameters and train the model to start").size(14)]
        } else {
            column![text("Model ready. Draw a digit and click Predict").size(14)]
        };

        let prediction_display = match &self.prediction {
            None => column![],
            Some(pred) => {
                let bars: Vec<Element<Message>> = pred
                    .all_scores
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

                column![
                    text(format!("Predicted Digit: {}", pred.digit)).size(32),
                    text(format!("Confidence: {:.1}%", pred.confidence * 100.0)).size(20),
                    column(bars).spacing(5),
                ]
            }
        };

        let content = row![
            column![canvas_widget, canvas_controls]
                .spacing(20)
                .align_x(Alignment::Center),
            column![training_controls, status_text, prediction_display]
                .spacing(20)
                .padding(20),
        ]
        .spacing(40);

        container(content)
            .width(Fill)
            .height(Fill)
            .padding(20)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    pub fn run(flags: AppFlags) -> iced::Result {
        iced::application("MNIST Digit Painter", MnistPainter::update, MnistPainter::view)
            .window_size((1200.0, 700.0))
            .run_with(move || MnistPainter::new(flags))
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
}

async fn train_model_async(
    service: MnistTrainingService,
    config: TrainerConfig,
) -> TrainingResult {
    tokio::task::spawn_blocking(move || {
        service.train(config).expect("Training failed")
    })
    .await
    .expect("Training task panicked")
}
