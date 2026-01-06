use ndarray::Array2;

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
}
