use super::types::{ActivationType, NetworkConfig};
use anyhow::Result;
use nn_core::{
    activation::{ReLU, Sigmoid, Softmax, Tanh},
    activation_layer::ActivationLayer,
    layer::DenseLayer,
    sequential::Sequential,
};

pub struct NetworkBuilder;

impl NetworkBuilder {
    pub fn build(config: &NetworkConfig) -> Result<Sequential> {
        let mut model = Sequential::new();
        let mut prev_size = config.input_size;

        // Add hidden layers
        for layer_config in &config.hidden_layers {
            model.add(Box::new(DenseLayer::new(prev_size, layer_config.neurons)));

            let activation: Box<dyn nn_core::module::Module> = match layer_config.activation {
                ActivationType::ReLU => Box::new(ActivationLayer::new(ReLU)),
                ActivationType::Tanh => Box::new(ActivationLayer::new(Tanh)),
                ActivationType::Sigmoid => Box::new(ActivationLayer::new(Sigmoid)),
                ActivationType::Linear => {
                    // Linear activation is identity, skip adding activation layer
                    prev_size = layer_config.neurons;
                    continue;
                }
            };

            model.add(activation);
            prev_size = layer_config.neurons;
        }

        // Add output layer
        model.add(Box::new(DenseLayer::new(prev_size, config.output_size)));
        model.add(Box::new(ActivationLayer::new(Softmax)));

        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playground::types::LayerConfig;

    #[test]
    fn test_build_simple_network() {
        let config = NetworkConfig::new(
            2,
            vec![LayerConfig::new(4, ActivationType::ReLU)],
        );

        let model = NetworkBuilder::build(&config);
        assert!(model.is_ok());
    }

    #[test]
    fn test_build_deep_network() {
        let config = NetworkConfig::new(
            2,
            vec![
                LayerConfig::new(4, ActivationType::ReLU),
                LayerConfig::new(4, ActivationType::ReLU),
                LayerConfig::new(4, ActivationType::ReLU),
            ],
        );

        let model = NetworkBuilder::build(&config);
        assert!(model.is_ok());
    }

    #[test]
    fn test_build_mixed_activations() {
        let config = NetworkConfig::new(
            2,
            vec![
                LayerConfig::new(4, ActivationType::Tanh),
                LayerConfig::new(2, ActivationType::Sigmoid),
            ],
        );

        let model = NetworkBuilder::build(&config);
        assert!(model.is_ok());
    }

    #[test]
    fn test_total_parameters() {
        // Input: 2, Hidden: [4, 2], Output: 2
        // Params: (2*4+4) + (4*2+2) + (2*2+2) = 12 + 10 + 6 = 28
        let config = NetworkConfig::new(
            2,
            vec![
                LayerConfig::new(4, ActivationType::ReLU),
                LayerConfig::new(2, ActivationType::ReLU),
            ],
        );

        assert_eq!(config.total_parameters(), 28);
    }
}
