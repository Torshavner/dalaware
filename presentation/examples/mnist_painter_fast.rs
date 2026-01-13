use std::env;
use nn_application::load_mnist;
use nn_presentation::painter::app::{AppFlags, MnistPainter};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    match env::current_dir() {
        Ok(path) => println!("The current directory is {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }

    println!("===============================================");
    println!("MNIST Digit Painter - Fast Launch");
    println!("===============================================\n");

    println!("Checking for MNIST dataset...");

    // Check if files exist
    let data_path = std::path::Path::new("data/mnist");
    if !data_path.exists() || std::fs::read_dir(data_path).unwrap().count() < 4 {
        println!("\n⚠️  MNIST dataset not found or incomplete.");
        println!("\nTo download manually, run these commands:\n");
        println!("  mkdir -p data/mnist");
        println!("  cd data/mnist");
        println!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/train-images-idx3-ubyte.gz");
        println!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/train-labels-idx1-ubyte.gz");
        println!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/t10k-images-idx3-ubyte.gz");
        println!("  curl -O https://ossci-datasets.s3.amazonaws.com/mnist/t10k-labels-idx1-ubyte.gz");
        println!("  cd ../..\n");
        println!("Or let the automatic download complete (may take several minutes)...\n");
    }

    println!("Loading MNIST dataset (this may take a while on first run)...");
    let dataset = match load_mnist() {
        Ok(ds) => {
            println!("✓ Dataset loaded successfully!");
            println!("  Training samples: {}", ds.train_images.nrows());
            println!("  Test samples: {}\n", ds.test_images.nrows());
            ds
        }
        Err(e) => {
            eprintln!("\n❌ Failed to load MNIST dataset: {}", e);
            eprintln!("\nPlease download manually using the commands above.");
            std::process::exit(1);
        }
    };

    println!("Launching MNIST Painter...");
    println!("\nInstructions:");
    println!("  1. Configure training parameters (epochs, learning rate, batch size)");
    println!("  2. Click 'Train Model' to train the neural network");
    println!("  3. Draw a digit (0-9) on the canvas using your mouse");
    println!("  4. Click 'Predict' to see what the network thinks you drew\n");

    MnistPainter::run(AppFlags { dataset })
}
