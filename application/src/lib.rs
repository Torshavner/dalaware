// Application layer: Training orchestration and network construction

pub mod mnist_loader;
pub mod services;
pub mod trainer;

pub use mnist_loader::{load_mnist, load_mnist_subset, MnistDataset};
pub use services::{MnistTrainingService, PredictionResult, PredictionService, TrainingProgress, TrainingResult};
pub use trainer::{EpochMetrics, Trainer, TrainerConfig};
