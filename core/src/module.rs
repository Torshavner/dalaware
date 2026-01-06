use ndarray::Array2;

pub trait Module {
    fn forward(&mut self, input: &Array2<f32>) -> Array2<f32>;

    fn backward(&mut self, grad_output: &Array2<f32>) -> Array2<f32>;

    fn update_parameters(&mut self, learning_rate: f32);
}
