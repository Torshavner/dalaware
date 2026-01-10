use crate::module::Module;
use ndarray::{Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

#[derive(Debug, thiserror::Error)]
pub enum LayerError {
    #[error("forward pass must be called before backward pass")]
    MissingForwardPass,
}

pub struct DenseLayer {
    pub(crate) weights: Array2<f32>,
    pub(crate) biases: Array1<f32>,
    cached_input: Option<Array2<f32>>,
    weight_gradients: Option<Array2<f32>>,
    bias_gradients: Option<Array1<f32>>,
}

impl DenseLayer {
    #[must_use]
    pub fn new(input_size: usize, output_size: usize) -> Self {
        #[allow(clippy::cast_precision_loss)]
        let scale = (2.0 / input_size as f32).sqrt();
        let weights = Array2::random((input_size, output_size), Uniform::new(-scale, scale));
        let biases = Array1::zeros(output_size);

        Self {
            weights,
            biases,
            cached_input: None,
            weight_gradients: None,
            bias_gradients: None,
        }
    }
}

impl Module for DenseLayer {
    fn forward(&mut self, input: &Array2<f32>) -> Array2<f32> {
        self.cached_input = Some(input.clone());
        input.dot(&self.weights) + &self.biases
    }

    fn backward(&mut self, grad_output: &Array2<f32>) -> Array2<f32> {
        let Some(input) = self.cached_input.as_ref() else {
            panic!("INVARIANT VIOLATION: forward must be called before backward");
        };

        self.weight_gradients = Some(input.t().dot(grad_output));
        self.bias_gradients = Some(grad_output.sum_axis(ndarray::Axis(0)));

        grad_output.dot(&self.weights.t())
    }

    fn update_parameters(&mut self, learning_rate: f32) {
        if let Some(ref weight_grads) = self.weight_gradients {
            self.weights = &self.weights - &(weight_grads * learning_rate);
        }

        if let Some(ref bias_grads) = self.bias_gradients {
            self.biases = &self.biases - &(bias_grads * learning_rate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    mod dense_layer_forward {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__2x2_identity_weights__when__forward__then__returns_input_plus_bias() {
            let mut layer = DenseLayer::new(2, 2);
            layer.weights = array![[1.0, 0.0], [0.0, 1.0]];
            layer.biases = array![0.0, 0.0];

            let input = array![[1.0, 2.0]];
            let output = layer.forward(&input);

            assert_eq!(output[[0, 0]], 1.0);
            assert_eq!(output[[0, 1]], 2.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__simple_weights__when__forward__then__computes_linear_transformation() {
            let mut layer = DenseLayer::new(2, 1);
            layer.weights = array![[0.5], [0.3]];
            layer.biases = array![0.1];

            let input = array![[2.0, 4.0]];
            let output = layer.forward(&input);

            let expected = 2.0 * 0.5 + 4.0 * 0.3 + 0.1;
            assert!((output[[0, 0]] - expected).abs() < 1e-6);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__batch_input__when__forward__then__processes_all_samples() {
            let mut layer = DenseLayer::new(2, 1);
            layer.weights = array![[1.0], [1.0]];
            layer.biases = array![0.0];

            let input = array![[1.0, 2.0], [3.0, 4.0]];
            let output = layer.forward(&input);

            assert_eq!(output.shape(), &[2, 1]);
            assert_eq!(output[[0, 0]], 3.0);
            assert_eq!(output[[1, 0]], 7.0);
        }
    }

    mod dense_layer_backward {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__forward_called__when__backward__then__computes_input_gradient() {
            let mut layer = DenseLayer::new(2, 2);
            layer.weights = array![[1.0, 2.0], [3.0, 4.0]];

            let input = array![[1.0, 2.0]];
            layer.forward(&input);

            let grad_output = array![[1.0, 1.0]];
            let grad_input = layer.backward(&grad_output);

            assert_eq!(grad_input[[0, 0]], 1.0 * 1.0 + 1.0 * 2.0);
            assert_eq!(grad_input[[0, 1]], 1.0 * 3.0 + 1.0 * 4.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__forward_called__when__backward__then__stores_weight_gradients() {
            let mut layer = DenseLayer::new(2, 1);
            layer.weights = array![[0.5], [0.3]];

            let input = array![[2.0, 4.0]];
            layer.forward(&input);

            let grad_output = array![[1.0]];
            layer.backward(&grad_output);

            let weight_grads = layer.weight_gradients.as_ref().unwrap();
            assert_eq!(weight_grads[[0, 0]], 2.0);
            assert_eq!(weight_grads[[1, 0]], 4.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__forward_called__when__backward__then__stores_bias_gradients() {
            let mut layer = DenseLayer::new(2, 2);

            let input = array![[1.0, 2.0], [3.0, 4.0]];
            layer.forward(&input);

            let grad_output = array![[1.0, 2.0], [3.0, 4.0]];
            layer.backward(&grad_output);

            let bias_grads = layer.bias_gradients.as_ref().unwrap();
            assert_eq!(bias_grads[0], 4.0);
            assert_eq!(bias_grads[1], 6.0);
        }
    }

    mod dense_layer_update {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__gradients_computed__when__update_parameters__then__applies_gradient_descent() {
            let mut layer = DenseLayer::new(2, 1);
            layer.weights = array![[1.0], [1.0]];
            layer.biases = array![0.0];

            let input = array![[1.0, 1.0]];
            layer.forward(&input);

            let grad_output = array![[1.0]];
            layer.backward(&grad_output);

            let learning_rate = 0.1;
            layer.update_parameters(learning_rate);

            assert!((layer.weights[[0, 0]] - 0.9).abs() < 1e-6);
            assert!((layer.weights[[1, 0]] - 0.9).abs() < 1e-6);
            assert!((layer.biases[0] - (-0.1)).abs() < 1e-6);
        }
    }
}
