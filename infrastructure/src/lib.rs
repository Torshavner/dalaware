pub mod observability;
pub mod painter;

pub use observability::init_tracing;
pub use painter::{MnistPainter, Message, PredictionResult};
