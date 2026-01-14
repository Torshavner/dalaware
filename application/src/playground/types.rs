use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivationType {
    ReLU,
    Tanh,
    Sigmoid,
    Linear,
}

impl ActivationType {
    pub const ALL: [ActivationType; 4] = [
        ActivationType::ReLU,
        ActivationType::Tanh,
        ActivationType::Sigmoid,
        ActivationType::Linear,
    ];
}

impl std::fmt::Display for ActivationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivationType::ReLU => write!(f, "ReLU"),
            ActivationType::Tanh => write!(f, "Tanh"),
            ActivationType::Sigmoid => write!(f, "Sigmoid"),
            ActivationType::Linear => write!(f, "Linear"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerConfig {
    pub neurons: usize,
    pub activation: ActivationType,
}

impl LayerConfig {
    pub fn new(neurons: usize, activation: ActivationType) -> Self {
        Self { neurons, activation }
    }
}

impl std::fmt::Display for LayerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}×{}", self.neurons, self.activation)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub hidden_layers: Vec<LayerConfig>,
    pub input_size: usize,
    pub output_size: usize,
}

impl NetworkConfig {
    pub fn new(input_size: usize, hidden_layers: Vec<LayerConfig>) -> Self {
        Self {
            hidden_layers,
            input_size,
            output_size: 2, // Binary classification
        }
    }

    pub fn total_parameters(&self) -> usize {
        let mut params = 0;
        let mut prev_size = self.input_size;

        for layer in &self.hidden_layers {
            params += prev_size * layer.neurons + layer.neurons;
            prev_size = layer.neurons;
        }

        params += prev_size * self.output_size + self.output_size;
        params
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::new(
            2, // Default input size (x, y)
            vec![
                LayerConfig::new(4, ActivationType::ReLU),
                LayerConfig::new(2, ActivationType::ReLU),
            ],
        )
    }
}

impl std::fmt::Display for NetworkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}→", self.input_size)?;
        for layer in &self.hidden_layers {
            write!(f, "{}→", layer.neurons)?;
        }
        write!(f, "{}", self.output_size)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresetType {
    Shallow,  // [4]
    Deep,     // [4, 4, 4]
    Pyramid,  // [6, 4, 2]
    Wide,     // [8, 8]
}

impl PresetType {
    pub const ALL: [PresetType; 4] = [
        PresetType::Shallow,
        PresetType::Deep,
        PresetType::Pyramid,
        PresetType::Wide,
    ];

    pub fn layers(&self, activation: ActivationType) -> Vec<LayerConfig> {
        let sizes = match self {
            PresetType::Shallow => vec![4],
            PresetType::Deep => vec![4, 4, 4],
            PresetType::Pyramid => vec![6, 4, 2],
            PresetType::Wide => vec![8, 8],
        };

        sizes
            .into_iter()
            .map(|neurons| LayerConfig::new(neurons, activation))
            .collect()
    }
}

impl std::fmt::Display for PresetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresetType::Shallow => write!(f, "Shallow [4]"),
            PresetType::Deep => write!(f, "Deep [4,4,4]"),
            PresetType::Pyramid => write!(f, "Pyramid [6,4,2]"),
            PresetType::Wide => write!(f, "Wide [8,8]"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub x: bool,
    pub y: bool,
    pub x_squared: bool,
    pub y_squared: bool,
    pub x_times_y: bool,
    pub sin_x: bool,
    pub sin_y: bool,
}

impl FeatureConfig {
    pub fn default() -> Self {
        Self {
            x: true,
            y: true,
            x_squared: false,
            y_squared: false,
            x_times_y: false,
            sin_x: false,
            sin_y: false,
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        if self.x {
            count += 1;
        }
        if self.y {
            count += 1;
        }
        if self.x_squared {
            count += 1;
        }
        if self.y_squared {
            count += 1;
        }
        if self.x_times_y {
            count += 1;
        }
        if self.sin_x {
            count += 1;
        }
        if self.sin_y {
            count += 1;
        }
        count
    }

    pub fn apply(&self, x: f32, y: f32) -> Vec<f32> {
        let mut features = Vec::new();
        if self.x {
            features.push(x);
        }
        if self.y {
            features.push(y);
        }
        if self.x_squared {
            features.push(x * x);
        }
        if self.y_squared {
            features.push(y * y);
        }
        if self.x_times_y {
            features.push(x * y);
        }
        if self.sin_x {
            features.push(x.sin());
        }
        if self.sin_y {
            features.push(y.sin());
        }
        features
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrainingState {
    Idle,
    Running,
    Paused,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn given__default_feature_config__when__count__then__returns_two() {
        let config = FeatureConfig::default();

        assert_eq!(config.count(), 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__all_features_enabled__when__count__then__returns_seven() {
        let config = FeatureConfig {
            x: true,
            y: true,
            x_squared: true,
            y_squared: true,
            x_times_y: true,
            sin_x: true,
            sin_y: true,
        };

        assert_eq!(config.count(), 7);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__no_features_enabled__when__count__then__returns_zero() {
        let config = FeatureConfig {
            x: false,
            y: false,
            x_squared: false,
            y_squared: false,
            x_times_y: false,
            sin_x: false,
            sin_y: false,
        };

        assert_eq!(config.count(), 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__default_features__when__apply__then__returns_x_and_y() {
        let config = FeatureConfig::default();

        let features = config.apply(2.0, 3.0);

        assert_eq!(features, vec![2.0, 3.0]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__x_squared_enabled__when__apply__then__includes_x_squared() {
        let mut config = FeatureConfig::default();
        config.x_squared = true;

        let features = config.apply(2.0, 3.0);

        assert_eq!(features, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__all_features__when__apply__then__includes_all_transformations() {
        let config = FeatureConfig {
            x: true,
            y: true,
            x_squared: true,
            y_squared: true,
            x_times_y: true,
            sin_x: true,
            sin_y: true,
        };

        let features = config.apply(2.0, 3.0);

        assert_eq!(features.len(), 7);
        assert_eq!(features[0], 2.0);        // x
        assert_eq!(features[1], 3.0);        // y
        assert_eq!(features[2], 4.0);        // x²
        assert_eq!(features[3], 9.0);        // y²
        assert_eq!(features[4], 6.0);        // x*y
        assert!((features[5] - 2.0_f32.sin()).abs() < 1e-6); // sin(x)
        assert!((features[6] - 3.0_f32.sin()).abs() < 1e-6); // sin(y)
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__network_config__when__total_parameters__then__calculates_correctly() {
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

    #[test]
    #[allow(non_snake_case)]
    fn given__single_layer_config__when__total_parameters__then__includes_output_layer() {
        // Input: 2, Hidden: [4], Output: 2
        // Params: (2*4+4) + (4*2+2) = 12 + 10 = 22
        let config = NetworkConfig::new(
            2,
            vec![LayerConfig::new(4, ActivationType::ReLU)],
        );

        assert_eq!(config.total_parameters(), 22);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__no_hidden_layers__when__total_parameters__then__only_output_layer() {
        // Input: 2, Hidden: [], Output: 2
        // Params: (2*2+2) = 6
        let config = NetworkConfig::new(2, vec![]);

        assert_eq!(config.total_parameters(), 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__preset_shallow__when__layers__then__returns_single_layer() {
        let layers = PresetType::Shallow.layers(ActivationType::ReLU);

        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].neurons, 4);
        assert_eq!(layers[0].activation, ActivationType::ReLU);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__preset_deep__when__layers__then__returns_three_layers() {
        let layers = PresetType::Deep.layers(ActivationType::Tanh);

        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0].neurons, 4);
        assert_eq!(layers[1].neurons, 4);
        assert_eq!(layers[2].neurons, 4);
        assert_eq!(layers[0].activation, ActivationType::Tanh);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__preset_pyramid__when__layers__then__returns_decreasing_sizes() {
        let layers = PresetType::Pyramid.layers(ActivationType::Sigmoid);

        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0].neurons, 6);
        assert_eq!(layers[1].neurons, 4);
        assert_eq!(layers[2].neurons, 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__preset_wide__when__layers__then__returns_wide_layers() {
        let layers = PresetType::Wide.layers(ActivationType::Linear);

        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].neurons, 8);
        assert_eq!(layers[1].neurons, 8);
        assert_eq!(layers[0].activation, ActivationType::Linear);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__layer_config__when__display__then__shows_neurons_and_activation() {
        let layer = LayerConfig::new(4, ActivationType::ReLU);

        assert_eq!(layer.to_string(), "4×ReLU");
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__network_config__when__display__then__shows_architecture() {
        let config = NetworkConfig::new(
            2,
            vec![
                LayerConfig::new(4, ActivationType::ReLU),
                LayerConfig::new(2, ActivationType::Tanh),
            ],
        );

        assert_eq!(config.to_string(), "2→4→2→2");
    }

    #[test]
    #[allow(non_snake_case)]
    fn given__default_network_config__when__created__then__has_expected_structure() {
        let config = NetworkConfig::default();

        assert_eq!(config.input_size, 2);
        assert_eq!(config.output_size, 2);
        assert_eq!(config.hidden_layers.len(), 2);
        assert_eq!(config.hidden_layers[0].neurons, 4);
        assert_eq!(config.hidden_layers[1].neurons, 2);
    }
}
