use std::env;
use nn_application::load_mnist;
use nn_infrastructure::init_tracing;
use nn_presentation::painter::app::{AppFlags, MnistPainter};

fn main() -> iced::Result {
    // Initialize structured logging
    init_tracing().expect("Failed to initialize tracing");

    match env::current_dir() {
        Ok(path) => tracing::info!(current_dir = %path.display(), "Application starting"),
        Err(e) => tracing::error!(error = ?e, "Failed to get current directory"),
    }

    tracing::info!("===============================================");
    tracing::info!("MNIST Digit Painter - Fast Launch");
    tracing::info!("===============================================");

    tracing::info!("Checking for MNIST dataset...");

    // Check if files exist
    let data_path = std::path::Path::new("data/mnist");
    if !data_path.exists() || std::fs::read_dir(data_path).unwrap().count() < 4 {
        tracing::warn!("MNIST dataset not found or incomplete");
        tracing::info!("To download manually, run these commands:");
        tracing::info!("  mkdir -p data/mnist");
        tracing::info!("  cd data/mnist");
        tracing::info!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/train-images-idx3-ubyte.gz");
        tracing::info!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/train-labels-idx1-ubyte.gz");
        tracing::info!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/t10k-images-idx3-ubyte.gz");
        tracing::info!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/t10k-labels-idx1-ubyte.gz");
        tracing::info!("  cd ../..");
        tracing::info!("Or let the automatic download complete (may take several minutes)...");
    }

    tracing::info!("Loading MNIST dataset (this may take a while on first run)...");
    let dataset = match load_mnist() {
        Ok(ds) => {
            tracing::info!(
                train_samples = ds.train_images.nrows(),
                test_samples = ds.test_images.nrows(),
                "Dataset loaded successfully"
            );
            ds
        }
        Err(e) => {
            tracing::error!(error = ?e, "Failed to load MNIST dataset");
            tracing::error!("Please download manually using the commands above");
            std::process::exit(1);
        }
    };

    tracing::info!("Launching MNIST Painter application");
    tracing::info!("Instructions:");
    tracing::info!("  1. Configure training parameters (epochs, learning rate, batch size)");
    tracing::info!("  2. Click 'Train Model' to train the neural network");
    tracing::info!("  3. Draw a digit (0-9) on the canvas using your mouse");
    tracing::info!("  4. Click 'Predict' to see what the network thinks you drew");

    MnistPainter::run(AppFlags { dataset })
}
