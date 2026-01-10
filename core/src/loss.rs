use ndarray::{Array2, Axis};

pub trait Loss {
    fn calculate(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> f32;

    fn gradient(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> Array2<f32>;
}

pub struct MeanSquaredError;

impl Loss for MeanSquaredError {
    fn calculate(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> f32 {
        let diff = predictions - targets;
        let squared = &diff * &diff;
        #[allow(clippy::cast_precision_loss)]
        let n = predictions.len() as f32;
        squared.sum() / n
    }

    fn gradient(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> Array2<f32> {
        #[allow(clippy::cast_precision_loss)]
        let n = predictions.len() as f32;
        (predictions - targets) * (2.0 / n)
    }
}

pub struct CrossEntropy;

impl Loss for CrossEntropy {
    #[allow(unused_variables)]
    fn calculate(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> f32 {
        let epsilon = 1e-7;
        let loss_matrix = targets * &predictions.mapv(|p| (p + epsilon).ln());

        -loss_matrix
            .sum_axis(Axis(1))
            .mean()
            .unwrap_or(0.0)
    }

    #[allow(unused_variables)]
    fn gradient(&self, predictions: &Array2<f32>, targets: &Array2<f32>) -> Array2<f32> {
        predictions - targets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    mod mean_squared_error {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__perfect_predictions__when__calculate__then__returns_zero() {
            let mse = MeanSquaredError;
            let predictions = array![[1.0, 2.0], [3.0, 4.0]];
            let targets = array![[1.0, 2.0], [3.0, 4.0]];

            let loss = mse.calculate(&predictions, &targets);

            assert!(loss.abs() < 1e-6);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__uniform_error__when__calculate__then__returns_squared_error() {
            let mse = MeanSquaredError;
            let predictions = array![[2.0, 3.0], [4.0, 5.0]];
            let targets = array![[1.0, 2.0], [3.0, 4.0]];

            let loss = mse.calculate(&predictions, &targets);

            assert!((loss - 1.0).abs() < 1e-6);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__predictions_and_targets__when__gradient__then__returns_scaled_difference() {
            let mse = MeanSquaredError;
            let predictions = array![[2.0]];
            let targets = array![[1.0]];

            let grad = mse.gradient(&predictions, &targets);

            assert!((grad[[0, 0]] - 2.0).abs() < 1e-6);
        }
    }

    mod cross_entropy {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__perfect_predictions__when__calculate__then__returns_near_zero() {
            let ce = CrossEntropy;
            let predictions = array![[0.0, 0.0, 1.0]]; // 100% confident, correct
            let targets = array![[0.0, 0.0, 1.0]];     // True class is 2

            let loss = ce.calculate(&predictions, &targets);

            assert!(loss < 0.01); // Should be very small (log(1) = 0)
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__wrong_prediction__when__calculate__then__returns_high_loss() {
            let ce = CrossEntropy;
            let predictions = array![[0.9, 0.05, 0.05]]; // Predicts class 0
            let targets = array![[0.0, 0.0, 1.0]];        // True class is 2

            let loss = ce.calculate(&predictions, &targets);

            assert!(loss > 2.0); // -log(0.05) ≈ 3.0
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__softmax_output__when__calculate__then__computes_cross_entropy() {
            let ce = CrossEntropy;
            let predictions = array![[0.1, 0.2, 0.7]]; // Softmax output
            let targets = array![[0.0, 0.0, 1.0]];      // True class is 2

            let loss = ce.calculate(&predictions, &targets);

            let expected = -(1.0_f32 * 0.7_f32.ln()); // -log(0.7) ≈ 0.357
            assert!((loss - expected).abs() < 0.01);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__batch_predictions__when__calculate__then__averages_loss() {
            let ce = CrossEntropy;
            let predictions = array![
                [0.7, 0.2, 0.1],
                [0.1, 0.8, 0.1]
            ];
            let targets = array![
                [1.0, 0.0, 0.0], // Class 0
                [0.0, 1.0, 0.0]  // Class 1
            ];

            let loss = ce.calculate(&predictions, &targets);

            assert!(loss > 0.0 && loss < 1.0); // Should be averaged
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__softmax_predictions__when__gradient__then__returns_difference() {
            let ce = CrossEntropy;
            let predictions = array![[0.1, 0.2, 0.7]];
            let targets = array![[0.0, 0.0, 1.0]];

            let grad = ce.gradient(&predictions, &targets);

            assert!((grad[[0, 0]] - 0.1).abs() < 1e-6);  // 0.1 - 0.0
            assert!((grad[[0, 1]] - 0.2).abs() < 1e-6);  // 0.2 - 0.0
            assert!((grad[[0, 2]] - (-0.3)).abs() < 1e-6); // 0.7 - 1.0
        }
    }
}
