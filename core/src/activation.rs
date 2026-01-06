use ndarray::Array2;

pub trait Activation {
    fn activate(&self, input: &Array2<f32>) -> Array2<f32>;

    fn derivative(&self, input: &Array2<f32>) -> Array2<f32>;
}

pub struct Sigmoid;

impl Activation for Sigmoid {
    fn activate(&self, input: &Array2<f32>) -> Array2<f32> {
        input.mapv(|x| 1.0 / (1.0 + (-x).exp()))
    }

    fn derivative(&self, input: &Array2<f32>) -> Array2<f32> {
        let activated = self.activate(input);
        &activated * &(1.0 - &activated)
    }
}

pub struct ReLU;

impl Activation for ReLU {
    fn activate(&self, input: &Array2<f32>) -> Array2<f32> {
        input.mapv(|x| x.max(0.0))
    }

    fn derivative(&self, input: &Array2<f32>) -> Array2<f32> {
        input.mapv(|x| if x > 0.0 { 1.0 } else { 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    mod sigmoid_activation {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__zero_input__when__activate__then__returns_half() {
            let sigmoid = Sigmoid;
            let input = array![[0.0]];

            let result = sigmoid.activate(&input);

            assert!((result[[0, 0]] - 0.5).abs() < 1e-6);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__large_positive_input__when__activate__then__approaches_one() {
            let sigmoid = Sigmoid;
            let input = array![[10.0]];

            let result = sigmoid.activate(&input);

            assert!(result[[0, 0]] > 0.9999);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__large_negative_input__when__activate__then__approaches_zero() {
            let sigmoid = Sigmoid;
            let input = array![[-10.0]];

            let result = sigmoid.activate(&input);

            assert!(result[[0, 0]] < 0.0001);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__zero_input__when__derivative__then__returns_quarter() {
            let sigmoid = Sigmoid;
            let input = array![[0.0]];

            let result = sigmoid.derivative(&input);

            assert!((result[[0, 0]] - 0.25).abs() < 1e-6);
        }
    }

    mod relu_activation {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn given__positive_input__when__activate__then__returns_same_value() {
            let relu = ReLU;
            let input = array![[5.0, 3.2], [1.1, 0.5]];

            let result = relu.activate(&input);

            assert_eq!(result[[0, 0]], 5.0);
            assert_eq!(result[[0, 1]], 3.2);
            assert_eq!(result[[1, 0]], 1.1);
            assert_eq!(result[[1, 1]], 0.5);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__negative_input__when__activate__then__returns_zero() {
            let relu = ReLU;
            let input = array![[-5.0, -3.2], [-1.1, -0.5]];

            let result = relu.activate(&input);

            assert_eq!(result[[0, 0]], 0.0);
            assert_eq!(result[[0, 1]], 0.0);
            assert_eq!(result[[1, 0]], 0.0);
            assert_eq!(result[[1, 1]], 0.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__mixed_input__when__activate__then__clips_negatives_to_zero() {
            let relu = ReLU;
            let input = array![[2.0, -1.0], [-3.0, 4.0]];

            let result = relu.activate(&input);

            assert_eq!(result[[0, 0]], 2.0);
            assert_eq!(result[[0, 1]], 0.0);
            assert_eq!(result[[1, 0]], 0.0);
            assert_eq!(result[[1, 1]], 4.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__positive_input__when__derivative__then__returns_one() {
            let relu = ReLU;
            let input = array![[5.0]];

            let result = relu.derivative(&input);

            assert_eq!(result[[0, 0]], 1.0);
        }

        #[test]
        #[allow(non_snake_case)]
        fn given__negative_input__when__derivative__then__returns_zero() {
            let relu = ReLU;
            let input = array![[-5.0]];

            let result = relu.derivative(&input);

            assert_eq!(result[[0, 0]], 0.0);
        }
    }
}
