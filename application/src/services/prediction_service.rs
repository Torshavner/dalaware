use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub digit: usize,
    pub confidence: f32,
    pub all_scores: Vec<f32>,
}

pub trait PredictionService: Send {
    fn predict(&mut self, raw_pixels: &[u8; 28 * 28]) -> PredictionResult;
}

pub fn normalize_mnist_input(raw_pixels: &[u8; 28 * 28]) -> Array2<f32> {
    let normalized: Vec<f32> = raw_pixels
        .iter()
        .map(|&pixel| f32::from(pixel) / 255.0)
        .collect();

    Array2::from_shape_vec((1, 784), normalized).expect("Input shape must be 784")
}

pub fn extract_prediction_from_output(output: &Array2<f32>) -> PredictionResult {
    let scores: Vec<f32> = output.row(0).to_vec();

    let (digit, &confidence) = scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or((0, &0.0));

    PredictionResult {
        digit,
        confidence,
        all_scores: scores,
    }
}
