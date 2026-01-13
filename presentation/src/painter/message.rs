use nn_application::TrainingResult;

#[derive(Debug, Clone)]
pub enum Message {
    CanvasMouseDown { x: usize, y: usize },
    CanvasMouseMove { x: usize, y: usize },
    CanvasMouseUp,
    ClearCanvas,
    TrainModel,
    PredictDigit,
    TrainingComplete(TrainingResult),
    EpochsChanged(String),
    LearningRateChanged(String),
    BatchSizeChanged(String),
}
