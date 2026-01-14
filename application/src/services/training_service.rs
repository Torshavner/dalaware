use crate::services::prediction_service::{
    extract_prediction_from_output, normalize_mnist_input, PredictionResult, PredictionService,
};
use crate::{MnistDataset, Trainer, TrainerConfig};
use nn_core::activation::{ReLU, Softmax};
use nn_core::activation_layer::ActivationLayer;
use nn_core::layer::DenseLayer;
use nn_core::loss::CrossEntropy;
use nn_core::sequential::Sequential;
use nn_core::Module;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct TrainingProgress {
    pub epoch: usize,
    pub loss: f32,
    pub accuracy: f32,
}

#[derive(Debug, Clone)]
pub struct TrainingResult {
    pub final_accuracy: f32,
    pub final_loss: f32,
    pub progress: Vec<TrainingProgress>,
}

#[derive(Clone)]
pub struct MnistTrainingService {
    model: Arc<Mutex<Option<Sequential>>>,
    dataset: MnistDataset,
}

impl MnistTrainingService {
    pub fn new(dataset: MnistDataset) -> Self {
        Self {
            model: Arc::new(Mutex::new(None)),
            dataset,
        }
    }

    pub fn train(&self, config: TrainerConfig) -> anyhow::Result<TrainingResult> {
        let mut model = Sequential::new();
        model.add(Box::new(DenseLayer::new(784, 128)));
        model.add(Box::new(ActivationLayer::new(ReLU)));
        model.add(Box::new(DenseLayer::new(128, 64)));
        model.add(Box::new(ActivationLayer::new(ReLU)));
        model.add(Box::new(DenseLayer::new(64, 10)));
        model.add(Box::new(ActivationLayer::new(Softmax)));

        let trainer = Trainer::new(config, CrossEntropy);
        let metrics = trainer.train(
            &mut model,
            &self.dataset.train_images,
            &self.dataset.train_labels,
            &self.dataset.test_images,   // Validate on test set
            &self.dataset.test_labels,   // Validate on test set
        )?;

        let progress: Vec<TrainingProgress> = metrics
            .iter()
            .map(|m| TrainingProgress {
                epoch: m.epoch,
                loss: m.loss,
                accuracy: m.accuracy,
            })
            .collect();

        let final_metrics = metrics.last().ok_or_else(|| {
            anyhow::anyhow!("Training produced no metrics")
        })?;

        let result = TrainingResult {
            final_accuracy: final_metrics.accuracy,
            final_loss: final_metrics.loss,
            progress,
        };

        *self.model.lock().unwrap() = Some(model);
        Ok(result)
    }

    pub fn has_trained_model(&self) -> bool {
        self.model.lock().unwrap().is_some()
    }
}

impl PredictionService for MnistTrainingService {
    fn predict(&mut self, raw_pixels: &[u8; 28 * 28]) -> PredictionResult {
        let mut model_guard = self.model.lock().unwrap();
        let model = model_guard.as_mut().expect("Model must be trained before prediction");

        let input = normalize_mnist_input(raw_pixels);
        let output = model.forward(&input);
        extract_prediction_from_output(&output)
    }
}
