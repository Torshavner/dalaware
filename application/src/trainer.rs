use nn_core::loss::Loss;
use nn_core::Module;
use ndarray::Array2;

/// Training metrics for a single epoch
#[derive(Debug, Clone)]
pub struct EpochMetrics {
    pub epoch: usize,
    pub loss: f32,
    pub accuracy: f32,
}

/// Configuration for training
pub struct TrainerConfig {
    pub epochs: usize,
    pub learning_rate: f32,
    pub batch_size: usize,
}

impl Default for TrainerConfig {
    fn default() -> Self {
        Self {
            epochs: 10,
            learning_rate: 0.01,
            batch_size: 32,
        }
    }
}

/// Trainer orchestrates the training loop for neural networks
///
/// Handles:
/// - Mini-batch SGD (Stochastic Gradient Descent)
/// - Epoch management
/// - Loss calculation
/// - Metrics tracking (loss, accuracy per epoch)
///
/// Example usage:
/// ```ignore
/// let config = TrainerConfig {
///     epochs: 20,
///     learning_rate: 0.1,
///     batch_size: 64,
/// };
/// let trainer = Trainer::new(config, CrossEntropy);
/// let metrics = trainer.train(&mut model, &train_inputs, &train_targets)?;
/// ```
pub struct Trainer<L: Loss> {
    config: TrainerConfig,
    loss_fn: L,
}

impl<L: Loss> Trainer<L> {
    #[must_use]
    pub fn new(config: TrainerConfig, loss_fn: L) -> Self {
        Self { config, loss_fn }
    }

    /// Train the model on the training dataset
    ///
    /// Evaluates on the validation set after each epoch to track generalization
    ///
    /// # Arguments
    /// * `model` - The neural network model to train
    /// * `train_inputs` - Training data inputs
    /// * `train_targets` - Training data targets
    /// * `val_inputs` - Validation data inputs (typically test set)
    /// * `val_targets` - Validation data targets (typically test set)
    ///
    /// Returns a vector of EpochMetrics with validation loss/accuracy, one per epoch
    pub fn train<M: Module>(
        &self,
        model: &mut M,
        train_inputs: &Array2<f32>,
        train_targets: &Array2<f32>,
        val_inputs: &Array2<f32>,
        val_targets: &Array2<f32>,
    ) -> anyhow::Result<Vec<EpochMetrics>> {
        let mut metrics = Vec::with_capacity(self.config.epochs);

        tracing::info!(
            epochs = self.config.epochs,
            learning_rate = self.config.learning_rate,
            batch_size = self.config.batch_size,
            train_samples = train_inputs.nrows(),
            val_samples = val_inputs.nrows(),
            "Starting training"
        );

        for epoch in 0..self.config.epochs {
            let start = std::time::Instant::now();

            // Train on training set
            self.train_epoch(model, train_inputs, train_targets)?;

            // Evaluate on validation set
            let (loss, accuracy) = self.evaluate(model, val_inputs, val_targets)?;

            let duration = start.elapsed();
            let epoch_metrics = EpochMetrics {
                epoch,
                loss,
                accuracy,
            };

            tracing::info!(
                epoch = epoch + 1,
                total_epochs = self.config.epochs,
                duration_secs = duration.as_secs(),
                loss = %format!("{:.6}", epoch_metrics.loss),
                accuracy = %format!("{:.2}%", epoch_metrics.accuracy * 100.0),
                "Epoch completed"
            );

            metrics.push(epoch_metrics);
        }

        tracing::info!("Training completed successfully");

        Ok(metrics)
    }

    /// Train for a single epoch (one pass through the dataset)
    ///
    /// Performs forward pass, backward pass, and parameter updates for all batches
    fn train_epoch<M: Module>(
        &self,
        model: &mut M,
        inputs: &Array2<f32>,
        targets: &Array2<f32>,
    ) -> anyhow::Result<()> {
        let num_samples = inputs.nrows();
        let batch_size = self.config.batch_size.min(num_samples);

        // Process all samples in batches
        for start_idx in (0..num_samples).step_by(batch_size) {
            let end_idx = (start_idx + batch_size).min(num_samples);

            // Extract batch
            let batch_inputs = inputs.slice(ndarray::s![start_idx..end_idx, ..]).to_owned();
            let batch_targets = targets.slice(ndarray::s![start_idx..end_idx, ..]).to_owned();

            // Forward pass
            let predictions = model.forward(&batch_inputs);

            // Backward pass
            let grad = self.loss_fn.gradient(&predictions, &batch_targets);
            model.backward(&grad);

            // Update parameters
            model.update_parameters(self.config.learning_rate);
        }

        Ok(())
    }

    /// Evaluate the model (forward pass only, no training)
    ///
    /// Returns (loss, accuracy)
    pub fn evaluate<M: Module>(
        &self,
        model: &mut M,
        inputs: &Array2<f32>,
        targets: &Array2<f32>,
    ) -> anyhow::Result<(f32, f32)> {
        // Forward pass
        let predictions = model.forward(inputs);

        // Calculate loss
        let loss = self.loss_fn.calculate(&predictions, targets);

        // Calculate accuracy
        let accuracy = Self::calculate_accuracy(&predictions, targets);

        Ok((loss, accuracy))
    }

    /// Calculate accuracy (% of correct predictions)
    ///
    /// For classification: argmax(predictions) == argmax(targets)
    fn calculate_accuracy(
        predictions: &Array2<f32>,
        targets: &Array2<f32>,
    ) -> f32 {
        let num_samples = predictions.nrows();
        if num_samples == 0 {
            return 0.0;
        }

        let mut correct = 0;

        for i in 0..num_samples {
            // Find argmax for predictions
            let pred_row = predictions.row(i);
            let pred_max_idx = pred_row
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            // Find argmax for targets
            let target_row = targets.row(i);
            let target_max_idx = target_row
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            if pred_max_idx == target_max_idx {
                correct += 1;
            }
        }

        #[allow(clippy::cast_precision_loss)]
        let accuracy = correct as f32 / num_samples as f32;
        accuracy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nn_core::layer::DenseLayer;
    use nn_core::loss::MeanSquaredError;
    use nn_core::sequential::Sequential;
    use ndarray::array;

    mod trainer_construction {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__trainer_config__when__new__then__creates_trainer() {
            let config = TrainerConfig {
                epochs: 5,
                learning_rate: 0.05,
                batch_size: 16,
            };
            let _trainer = Trainer::new(config, MeanSquaredError);

            // Should not panic
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__default_config__when__new__then__uses_defaults() {
            let config = TrainerConfig::default();

            assert_eq!(config.epochs, 10);
            assert_eq!(config.learning_rate, 0.01);
            assert_eq!(config.batch_size, 32);
        }
    }

    mod trainer_evaluation {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__simple_model__when__evaluate__then__returns_loss_and_accuracy() {
            let config = TrainerConfig::default();
            let trainer = Trainer::new(config, MeanSquaredError);

            let mut model = Sequential::new();
            model.add(Box::new(DenseLayer::new(2, 1)));

            let inputs = array![[1.0, 1.0], [2.0, 2.0]];
            let targets = array![[2.0], [4.0]];

            let result = trainer.evaluate(&mut model, &inputs, &targets);

            assert!(result.is_ok());
            let (loss, accuracy) = result.unwrap();
            assert!(loss >= 0.0);
            assert!(accuracy >= 0.0 && accuracy <= 1.0);
        }
    }

    mod trainer_training {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__simple_model__when__train__then__loss_decreases() {
            let config = TrainerConfig {
                epochs: 100,
                learning_rate: 0.1,
                batch_size: 2,
            };
            let trainer = Trainer::new(config, MeanSquaredError);

            let mut model = Sequential::new();
            model.add(Box::new(DenseLayer::new(2, 1)));

            // Simple linear problem: y = x1 + x2
            let train_inputs = array![[1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0]];
            let train_targets = array![[2.0], [4.0], [6.0], [8.0]];

            // Use same data for validation in this test
            let val_inputs = train_inputs.clone();
            let val_targets = train_targets.clone();

            let result = trainer.train(
                &mut model,
                &train_inputs,
                &train_targets,
                &val_inputs,
                &val_targets,
            );

            assert!(result.is_ok());
            let metrics = result.unwrap();

            // First epoch loss should be higher than last epoch loss
            assert!(metrics.first().unwrap().loss > metrics.last().unwrap().loss);
        }
    }
}
