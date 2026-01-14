use crate::module::Module;
use ndarray::Array2;

pub struct Sequential {
    layers: Vec<Box<dyn Module>>,
}

impl Sequential {
    #[must_use]
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add(&mut self, layer: Box<dyn Module>) {
        self.layers.push(layer);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.layers.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}

impl Default for Sequential {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for Sequential {
    fn forward(&mut self, input: &Array2<f32>) -> Array2<f32> {
        // Forward pass: pipe data through layers sequentially
        self.layers
            .iter_mut()
            .fold(input.clone(), |acc, layer| layer.forward(&acc))
    }

    fn backward(&mut self, grad_output: &Array2<f32>) -> Array2<f32> {
        // Backward pass: reverse order (chain rule)
        self.layers
            .iter_mut()
            .rev()
            .fold(grad_output.clone(), |acc, layer| layer.backward(&acc))
    }

    fn update_parameters(&mut self, learning_rate: f32) {
        // Update all layers
        for layer in &mut self.layers {
            layer.update_parameters(learning_rate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activation::ReLU;
    use crate::activation_layer::ActivationLayer;
    use crate::layer::DenseLayer;
    use ndarray::array;

    mod sequential_construction {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__new_sequential__when__created__then__is_empty() {
            let model = Sequential::new();

            assert_eq!(model.len(), 0);
            assert!(model.is_empty());
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__sequential__when__add_layer__then__increases_length() {
            let mut model = Sequential::new();

            model.add(Box::new(DenseLayer::new(2, 2)));

            assert_eq!(model.len(), 1);
            assert!(!model.is_empty());
        }
    }

    mod sequential_forward {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__single_dense_layer__when__forward__then__applies_transformation() {
            let mut model = Sequential::new();
            let mut dense = DenseLayer::new(2, 2);
            dense.weights = array![[1.0, 0.0], [0.0, 1.0]];
            dense.biases = array![0.0, 0.0];
            model.add(Box::new(dense));

            let input = array![[1.0, 2.0]];
            let output = model.forward(&input);

            assert_eq!(output[[0, 0]], 1.0);
            assert_eq!(output[[0, 1]], 2.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__dense_plus_relu__when__forward__then__applies_both() {
            let mut model = Sequential::new();

            let mut dense = DenseLayer::new(2, 2);
            dense.weights = array![[1.0, -1.0], [1.0, -1.0]];
            dense.biases = array![0.0, 0.0];

            model.add(Box::new(dense));
            model.add(Box::new(ActivationLayer::new(ReLU)));

            let input = array![[2.0, 1.0]]; // 2 + 1 = 3, -2 + -1 = -3
            let output = model.forward(&input);

            assert_eq!(output[[0, 0]], 3.0); // ReLU keeps positive
            assert_eq!(output[[0, 1]], 0.0); // ReLU zeros negative
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__multi_layer_network__when__forward__then__pipes_through_all_layers() {
            let mut model = Sequential::new();

            // Layer 1: 2 → 3
            let mut dense1 = DenseLayer::new(2, 3);
            dense1.weights = array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
            dense1.biases = array![0.0, 0.0, 1.0];

            // Layer 2: 3 → 1
            let mut dense2 = DenseLayer::new(3, 1);
            dense2.weights = array![[1.0], [1.0], [1.0]];
            dense2.biases = array![0.0];

            model.add(Box::new(dense1));
            model.add(Box::new(ActivationLayer::new(ReLU)));
            model.add(Box::new(dense2));

            let input = array![[2.0, 3.0]];
            let output = model.forward(&input);

            // dense1: [2, 3] → [2, 3, 1]
            // relu: [2, 3, 1] → [2, 3, 1]
            // dense2: [2, 3, 1] → [6]
            assert_eq!(output[[0, 0]], 6.0);
        }
    }

    mod sequential_backward {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__single_dense_layer__when__backward__then__propagates_gradient() {
            let mut model = Sequential::new();
            let mut dense = DenseLayer::new(2, 2);
            dense.weights = array![[1.0, 2.0], [3.0, 4.0]];
            model.add(Box::new(dense));

            let input = array![[1.0, 2.0]];
            model.forward(&input);

            let grad_output = array![[1.0, 1.0]];
            let grad_input = model.backward(&grad_output);

            // grad_input = grad_output @ weights.T
            // [1, 1] @ [[1, 3], [2, 4]] = [1*1 + 1*2, 1*3 + 1*4] = [3, 7]
            assert_eq!(grad_input[[0, 0]], 3.0);
            assert_eq!(grad_input[[0, 1]], 7.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__dense_plus_relu__when__backward__then__applies_in_reverse() {
            let mut model = Sequential::new();

            let mut dense = DenseLayer::new(2, 2);
            dense.weights = array![[1.0, -1.0], [1.0, -1.0]];
            dense.biases = array![0.0, 0.0];

            model.add(Box::new(dense));
            model.add(Box::new(ActivationLayer::new(ReLU)));

            let input = array![[2.0, 1.0]];
            model.forward(&input); // Output: [3, 0] after ReLU

            let grad_output = array![[1.0, 1.0]];
            let grad_input = model.backward(&grad_output);

            // ReLU derivative: [1, 0] (second was negative)
            // Grad after ReLU: [1, 0]
            // Grad through dense: [1, 0] @ weights.T
            assert_eq!(grad_input[[0, 0]], 1.0);
            assert_eq!(grad_input[[0, 1]], 1.0);
        }
    }

    mod sequential_update {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__sequential_with_layers__when__update_parameters__then__updates_all_trainable() {
            let mut model = Sequential::new();

            let mut dense = DenseLayer::new(2, 1);
            dense.weights = array![[1.0], [1.0]];
            dense.biases = array![0.0];

            model.add(Box::new(dense));
            model.add(Box::new(ActivationLayer::new(ReLU)));

            let input = array![[1.0, 1.0]];
            model.forward(&input);

            let grad_output = array![[1.0]];
            model.backward(&grad_output);

            let learning_rate = 0.1;
            model.update_parameters(learning_rate);

            // After update, weights should have changed
            // (This test just ensures no panic - actual values depend on gradients)
        }
    }
}
