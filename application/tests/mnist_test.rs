use nn_application::{load_mnist, load_mnist_subset, Trainer, TrainerConfig};
use nn_core::{
    activation::{ReLU, Softmax},
    activation_layer::ActivationLayer,
    layer::DenseLayer,
    loss::CrossEntropy,
    sequential::Sequential,
};

/// Verifies that a small MNIST subset trains successfully with loss decrease
#[test]
#[ignore = "Requires MNIST dataset download - run with `cargo test -- --ignored`"]
#[allow(non_snake_case)]
fn given__mnist_subset_100_samples__when__train_10_epochs__then__loss_decreases_and_accuracy_exceeds_random(
) {
    println!("Loading MNIST subset...");
    let dataset = load_mnist_subset(100, 20).expect("Failed to load MNIST subset");

    println!("Building network...");
    let mut model = Sequential::new();
    model.add(Box::new(DenseLayer::new(784, 64)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(64, 32)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(32, 10)));
    model.add(Box::new(ActivationLayer::new(Softmax)));

    let config = TrainerConfig {
        epochs: 10,
        learning_rate: 0.1,
        batch_size: 32,
    };
    let trainer = Trainer::new(config, CrossEntropy);

    println!("Training...");
    let metrics = trainer
        .train(&mut model, &dataset.train_images, &dataset.train_labels)
        .expect("Training failed");

    let initial_loss = metrics.first().unwrap().loss;
    let final_loss = metrics.last().unwrap().loss;

    println!("\nTraining Results (Small Subset):");
    println!("  Initial loss: {:.4}", initial_loss);
    println!("  Final loss:   {:.4}", final_loss);
    println!("  Initial acc:  {:.2}%", metrics.first().unwrap().accuracy * 100.0);
    println!("  Final acc:    {:.2}%", metrics.last().unwrap().accuracy * 100.0);

    assert!(
        final_loss < initial_loss,
        "Loss should decrease during training"
    );

    let (test_loss, test_accuracy) = trainer
        .evaluate(&mut model, &dataset.test_images, &dataset.test_labels)
        .expect("Evaluation failed");

    println!("\nTest Set:");
    println!("  Loss:     {:.4}", test_loss);
    println!("  Accuracy: {:.2}%", test_accuracy * 100.0);

    assert!(test_accuracy > 0.15, "Accuracy should be better than random");
}

/// Verifies full MNIST training achieves >90% test accuracy
///
/// Network: 784 → 128 (ReLU) → 64 (ReLU) → 10 (Softmax)
/// Training time: ~2-5 minutes on modern CPU
#[test]
#[ignore = "Long-running test - requires MNIST download - run with `cargo test test_mnist_full_training -- --ignored --nocapture`"]
#[allow(non_snake_case)]
fn given__full_mnist_dataset__when__train_15_epochs__then__achieves_90_percent_accuracy() {
    println!("Loading full MNIST dataset...");
    let dataset = load_mnist().expect("Failed to load MNIST");

    println!("Dataset loaded:");
    println!("  Training samples: {}", dataset.train_images.nrows());
    println!("  Test samples:     {}", dataset.test_images.nrows());

    println!("\nBuilding network: 784 → 128 → 64 → 10");
    let mut model = Sequential::new();
    model.add(Box::new(DenseLayer::new(784, 128)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(128, 64)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(64, 10)));
    model.add(Box::new(ActivationLayer::new(Softmax)));

    let epochs = 15;
    let config = TrainerConfig {
        epochs,
        learning_rate: 0.01,
        batch_size: 64,
    };
    let trainer = Trainer::new(config, CrossEntropy);

    println!("\nTraining for {} epochs...", epochs);
    let metrics = trainer
        .train(&mut model, &dataset.train_images, &dataset.train_labels)
        .expect("Training failed");

    println!("\nEpoch | Train Loss | Train Acc");
    println!("------|------------|----------");
    for metric in &metrics {
        println!(
            "{:5} | {:10.4} | {:8.2}%",
            metric.epoch + 1,
            metric.loss,
            metric.accuracy * 100.0
        );
    }

    println!("\nEvaluating on test set...");
    let (test_loss, test_accuracy) = trainer
        .evaluate(&mut model, &dataset.test_images, &dataset.test_labels)
        .expect("Evaluation failed");

    println!("\n╔════════════════════════════════╗");
    println!("║   MNIST Test Set Results       ║");
    println!("╠════════════════════════════════╣");
    println!("║ Loss:     {:<20.4} ║", test_loss);
    println!("║ Accuracy: {:<18.2}% ║", test_accuracy * 100.0);
    println!("╚════════════════════════════════╝");

    assert!(
        test_accuracy > 0.90,
        "Test accuracy should be >90%, got {:.2}%",
        test_accuracy * 100.0
    );

    println!("\n✅ SUCCESS! Network achieved >90% accuracy on MNIST!");
}

/// Verifies MNIST dataset loads with correct dimensions, normalization, and one-hot encoding
#[test]
#[ignore = "Requires MNIST dataset download"]
#[allow(non_snake_case)]
fn given__mnist_dataset__when__load__then__returns_correct_dimensions_and_normalized_data() {
    let dataset = load_mnist().expect("Failed to load MNIST");

    assert_eq!(dataset.train_images.nrows(), 60000);
    assert_eq!(dataset.train_images.ncols(), 784);
    assert_eq!(dataset.train_labels.nrows(), 60000);
    assert_eq!(dataset.train_labels.ncols(), 10);
    assert_eq!(dataset.test_images.nrows(), 10000);
    assert_eq!(dataset.test_images.ncols(), 784);
    assert_eq!(dataset.test_labels.nrows(), 10000);
    assert_eq!(dataset.test_labels.ncols(), 10);

    assert!(dataset.train_images.iter().all(|&x| x >= 0.0 && x <= 1.0));
    assert!(dataset.test_images.iter().all(|&x| x >= 0.0 && x <= 1.0));

    for i in 0..10 {
        let label_sum: f32 = dataset.train_labels.row(i).sum();
        assert!(
            (label_sum - 1.0).abs() < 0.01,
            "Label should be one-hot encoded"
        );
    }

    println!("✅ MNIST dataset loaded and validated successfully!");
}
