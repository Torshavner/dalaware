use anyhow::Result;
use ndarray::Array2;

/// MNIST dataset structure
#[derive(Clone)]
pub struct MnistDataset {
    pub train_images: Array2<f32>,
    pub train_labels: Array2<f32>,
    pub test_images: Array2<f32>,
    pub test_labels: Array2<f32>,
}

/// Load MNIST dataset and preprocess
///
/// Downloads if needed, normalizes images, and one-hot encodes labels
pub fn load_mnist() -> Result<MnistDataset> {
    // Try to find data/mnist directory - check both from workspace root and from crate root
    let base_path = if std::path::Path::new("data/mnist").exists() {
        "data/mnist"
    } else if std::path::Path::new("../data/mnist").exists() {
        "../data/mnist"
    } else if std::path::Path::new("../../data/mnist").exists() {
        "../../data/mnist"
    } else {
        // Default - will download if needed
        "data/mnist"
    };

    // Load MNIST data using the mnist crate
    let mnist::Mnist {
        trn_img,
        trn_lbl,
        tst_img,
        tst_lbl,
        ..
    } = mnist::MnistBuilder::new()
        .base_path(base_path)
        .base_url("https://ossci-datasets.s3.amazonaws.com/mnist/")
        .label_format_one_hot()
        .finalize();

    // Convert to ndarray and normalize
    let train_images = preprocess_images(&trn_img, 60000)?;
    let train_labels = preprocess_labels(&trn_lbl, 60000)?;
    let test_images = preprocess_images(&tst_img, 10000)?;
    let test_labels = preprocess_labels(&tst_lbl, 10000)?;

    Ok(MnistDataset {
        train_images,
        train_labels,
        test_images,
        test_labels,
    })
}

/// Preprocess images: convert u8 to f32 and normalize to [0, 1]
fn preprocess_images(images: &[u8], num_samples: usize) -> Result<Array2<f32>> {
    const IMAGE_SIZE: usize = 28 * 28; // 784

    let mut array = Array2::zeros((num_samples, IMAGE_SIZE));

    for (i, chunk) in images.chunks(IMAGE_SIZE).enumerate() {
        if i >= num_samples {
            break;
        }
        for (j, &pixel) in chunk.iter().enumerate() {
            // Normalize pixel values from [0, 255] to [0, 1]
            #[allow(clippy::cast_precision_loss)]
            let normalized = pixel as f32 / 255.0;
            array[[i, j]] = normalized;
        }
    }

    Ok(array)
}

/// Preprocess labels: convert u8 one-hot to f32
fn preprocess_labels(labels: &[u8], num_samples: usize) -> Result<Array2<f32>> {
    const NUM_CLASSES: usize = 10;

    let mut array = Array2::zeros((num_samples, NUM_CLASSES));

    for (i, chunk) in labels.chunks(NUM_CLASSES).enumerate() {
        if i >= num_samples {
            break;
        }
        for (j, &value) in chunk.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let float_value = value as f32;
            array[[i, j]] = float_value;
        }
    }

    Ok(array)
}

/// Load a subset of MNIST for quick testing
pub fn load_mnist_subset(train_size: usize, test_size: usize) -> Result<MnistDataset> {
    let full_dataset = load_mnist()?;

    let train_images = full_dataset
        .train_images
        .slice(ndarray::s![0..train_size, ..])
        .to_owned();
    let train_labels = full_dataset
        .train_labels
        .slice(ndarray::s![0..train_size, ..])
        .to_owned();
    let test_images = full_dataset
        .test_images
        .slice(ndarray::s![0..test_size, ..])
        .to_owned();
    let test_labels = full_dataset
        .test_labels
        .slice(ndarray::s![0..test_size, ..])
        .to_owned();

    Ok(MnistDataset {
        train_images,
        train_labels,
        test_images,
        test_labels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires MNIST dataset download - run manually"]
    fn test_load_mnist() {
        let dataset = load_mnist().expect("Failed to load MNIST");

        assert_eq!(dataset.train_images.nrows(), 60000);
        assert_eq!(dataset.train_images.ncols(), 784);
        assert_eq!(dataset.train_labels.nrows(), 60000);
        assert_eq!(dataset.train_labels.ncols(), 10);
        assert_eq!(dataset.test_images.nrows(), 10000);
        assert_eq!(dataset.test_images.ncols(), 784);
        assert_eq!(dataset.test_labels.nrows(), 10000);
        assert_eq!(dataset.test_labels.ncols(), 10);

        // Check normalization
        assert!(dataset.train_images.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }

    #[test]
    #[ignore = "Requires MNIST dataset download - run manually"]
    fn test_load_mnist_subset() {
        let dataset = load_mnist_subset(100, 20).expect("Failed to load MNIST subset");

        assert_eq!(dataset.train_images.nrows(), 100);
        assert_eq!(dataset.test_images.nrows(), 20);
    }
}
