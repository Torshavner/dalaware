#[derive(Debug, Clone)]
pub enum Message {
    CanvasMouseDown { x: usize, y: usize },
    CanvasMouseMove { x: usize, y: usize },
    CanvasMouseUp,
    ClearCanvas,
    TrainModel,
    PredictDigit,
    TrainingComplete(TrainingResult),
    PredictionComplete(PredictionResult),
    EpochsChanged(String),
    LearningRateChanged(String),
    BatchSizeChanged(String),
}

#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub digit: usize,
    pub confidence: f32,
    pub all_scores: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct TrainingResult {
    pub final_accuracy: f32,
    pub final_loss: f32,
}
