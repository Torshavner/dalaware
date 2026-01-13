use nn_application::load_mnist;
use nn_infrastructure::painter::app::{AppFlags, MnistPainter};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    println!("Loading MNIST dataset...");
    let dataset = load_mnist().expect("Failed to load MNIST dataset");
    println!("Dataset loaded successfully!");
    println!("Training samples: {}", dataset.train_images.nrows());
    println!("Test samples: {}", dataset.test_images.nrows());

    println!("\nLaunching MNIST Painter...");
    println!("Instructions:");
    println!("1. Configure training parameters (epochs, learning rate, batch size)");
    println!("2. Click 'Train Model' to train the neural network");
    println!("3. Draw a digit (0-9) on the canvas using your mouse");
    println!("4. Click 'Predict' to see what the network thinks you drew");
    println!("\nStarting application...");

    MnistPainter::run(AppFlags { dataset })
}
