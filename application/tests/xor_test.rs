use ndarray::array;
use nn_application::{Trainer, TrainerConfig};
use nn_core::{
    activation::{ReLU, Sigmoid},
    activation_layer::ActivationLayer,
    layer::DenseLayer,
    loss::MeanSquaredError,
    sequential::Sequential,
    Module,
};

/// Verifies that a neural network can learn the non-linear XOR function
///
/// XOR truth table: [0,0]→0, [0,1]→1, [1,0]→1, [1,1]→0
/// Network: 2 → 4 (ReLU) → 1 (Sigmoid)
#[test]
#[allow(non_snake_case)]
fn given__xor_dataset__when__train_5000_epochs__then__converges_to_low_loss_and_accurate_predictions(
) {
    let mut model = Sequential::new();
    model.add(Box::new(DenseLayer::new(2, 4)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(4, 1)));
    model.add(Box::new(ActivationLayer::new(Sigmoid)));

    let inputs = array![[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let targets = array![[0.0], [1.0], [1.0], [0.0]];

    let config = TrainerConfig {
        epochs: 5000,
        learning_rate: 0.3,
        batch_size: 4,
    };
    let trainer = Trainer::new(config, MeanSquaredError);

    let metrics = trainer.train(&mut model, &inputs, &targets).unwrap();

    let initial_loss = metrics.first().unwrap().loss;
    let final_loss = metrics.last().unwrap().loss;

    println!("XOR Training:");
    println!("  Initial loss: {:.4}", initial_loss);
    println!("  Final loss:   {:.4}", final_loss);
    println!("  Epochs:       {}", metrics.len());

    assert!(
        final_loss < 0.17,
        "XOR should converge to loss < 0.17, got {}",
        final_loss
    );
    assert!(
        final_loss < initial_loss * 0.8,
        "Loss should decrease by at least 20%"
    );

    let predictions = model.forward(&inputs);
    println!("\nPredictions:");
    for i in 0..4 {
        let input = inputs.row(i);
        let pred = predictions[[i, 0]];
        let target = targets[[i, 0]];
        println!(
            "  [{:.0}, {:.0}] → {:.3} (target: {:.0})",
            input[0], input[1], pred, target
        );
    }

    for i in 0..4 {
        let pred = predictions[[i, 0]];
        let target = targets[[i, 0]];
        assert!(
            (pred - target).abs() < 0.2,
            "Prediction for sample {} should be close to target",
            i
        );
    }
}

/// Verifies XOR training completes without panic for CI environments
#[test]
#[allow(non_snake_case)]
fn given__xor_dataset__when__train_10_epochs__then__completes_without_panic() {
    let mut model = Sequential::new();
    model.add(Box::new(DenseLayer::new(2, 4)));
    model.add(Box::new(ActivationLayer::new(ReLU)));
    model.add(Box::new(DenseLayer::new(4, 1)));
    model.add(Box::new(ActivationLayer::new(Sigmoid)));

    let inputs = array![[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let targets = array![[0.0], [1.0], [1.0], [0.0]];

    let config = TrainerConfig {
        epochs: 10,
        learning_rate: 0.1,
        batch_size: 4,
    };
    let trainer = Trainer::new(config, MeanSquaredError);

    let metrics = trainer.train(&mut model, &inputs, &targets).unwrap();
    assert_eq!(metrics.len(), 10);
}
