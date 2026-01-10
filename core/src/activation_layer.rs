use crate::activation::Activation;
use crate::module::Module;
use ndarray::Array2;

/// Wrapper that adapts an `Activation` function into a `Module`
///
/// This allows activations (ReLU, Sigmoid, Softmax) to be used in Sequential containers
/// alongside DenseLayer.
///
/// Example usage:
/// ```ignore
/// let relu_layer = ActivationLayer::new(ReLU);
/// let sigmoid_layer = ActivationLayer::new(Sigmoid);
/// ```
pub struct ActivationLayer<A: Activation> {
    activation: A,
    cached_input: Option<Array2<f32>>,
}

impl<A: Activation> ActivationLayer<A> {
    #[must_use]
    pub fn new(activation: A) -> Self {
        Self {
            activation,
            cached_input: None,
        }
    }
}

impl<A: Activation> Module for ActivationLayer<A> {
    fn forward(&mut self, input: &Array2<f32>) -> Array2<f32> {
        self.cached_input = Some(input.clone());
        self.activation.activate(input)
    }

    fn backward(&mut self, grad_output: &Array2<f32>) -> Array2<f32> {
        let Some(input) = self.cached_input.as_ref() else {
            panic!("INVARIANT VIOLATION: forward must be called before backward");
        };

        let activation_grad = self.activation.derivative(input);
        grad_output * &activation_grad
    }

    fn update_parameters(&mut self, _learning_rate: f32) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activation::{ReLU, Sigmoid};
    use ndarray::array;

    mod activation_layer_forward {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__relu_layer__when__forward__then__applies_relu() {
            let mut layer = ActivationLayer::new(ReLU);
            let input = array![[2.0, -1.0], [-3.0, 4.0]];

            let output = layer.forward(&input);

            assert_eq!(output[[0, 0]], 2.0);
            assert_eq!(output[[0, 1]], 0.0);
            assert_eq!(output[[1, 0]], 0.0);
            assert_eq!(output[[1, 1]], 4.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__sigmoid_layer__when__forward__then__applies_sigmoid() {
            let mut layer = ActivationLayer::new(Sigmoid);
            let input = array![[0.0]];

            let output = layer.forward(&input);

            assert!((output[[0, 0]] - 0.5).abs() < 1e-6);
        }
    }

    mod activation_layer_backward {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__relu_layer__when__backward__then__applies_relu_derivative() {
            let mut layer = ActivationLayer::new(ReLU);
            let input = array![[2.0, -1.0]];
            layer.forward(&input);

            let grad_output = array![[1.0, 1.0]];
            let grad_input = layer.backward(&grad_output);

            assert_eq!(grad_input[[0, 0]], 1.0);
            assert_eq!(grad_input[[0, 1]], 0.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__sigmoid_layer__when__backward__then__applies_sigmoid_derivative() {
            let mut layer = ActivationLayer::new(Sigmoid);
            let input = array![[0.0]];
            layer.forward(&input);

            let grad_output = array![[1.0]];
            let grad_input = layer.backward(&grad_output);

            assert!((grad_input[[0, 0]] - 0.25).abs() < 1e-6);
        }
    }

    mod activation_layer_update {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__activation_layer__when__update_parameters__then__does_nothing() {
            let mut layer = ActivationLayer::new(ReLU);
            let input = array![[1.0, 2.0]];
            layer.forward(&input);
            layer.backward(&array![[1.0, 1.0]]);

            // Should not panic or change state
            layer.update_parameters(0.1);
        }
    }
}
