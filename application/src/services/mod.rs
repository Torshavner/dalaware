pub mod prediction_service;
pub mod training_service;

pub use prediction_service::{PredictionResult, PredictionService};
pub use training_service::{MnistTrainingService, TrainingProgress, TrainingResult};
