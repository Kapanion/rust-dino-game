use rand::Rng;

pub const INPUTS: usize = 3;

#[derive(Debug)]
pub struct Perceptron {
    bias: f64,
    learning_rate: f64,
    weights: [f64; INPUTS],
}

impl Perceptron {

    pub fn new(bias: f64, learning_rate: f64, weights: &Vec<f64>) -> Self {
        let mut rng = rand::thread_rng();
        let mut perceptron = Perceptron {
            bias,
            learning_rate,
            weights: [0.0; INPUTS],
        };

        if weights.len() == 0 {
            for i in 0..INPUTS {
                perceptron.weights[i] = rng.gen_range(-0.99..0.99) + 0.01;
            }
        } else {
            for (i, w)  in weights.iter().enumerate() {
                perceptron.weights[i] = w + rng.gen_range(-0.0001..0.0001);
            }
        }

        perceptron
    }

    fn sigmoid(&self, value: f64) -> f64 {
        return 1.0 / (1.0 + (-1.0 * (value + self.bias)).exp());
    }

    fn sigmoid_derivative(&self, value: f64) -> f64 {
        self.sigmoid(value) * (1. - self.sigmoid(value))
    }

    pub fn predict(&mut self, inputs: &Vec<f64>) -> f64 {
        let mut weighted_sum: f64 = 0.0;
        for i in 0..inputs.len() {
            weighted_sum += self.weights[i] * inputs[i].clone();
        }
        self.sigmoid(weighted_sum)
    }

    pub fn error(&mut self, value: f64, inputs: &Vec<f64>) {
        let delta = value * self.sigmoid_derivative(value);
        for (i, v) in inputs.iter().enumerate() {
            self.weights[i] += delta * v * self.learning_rate;
        }
        self.bias += delta.abs() * self.learning_rate;
    }

    pub fn get_weight(&mut self) -> Vec<f64> {
        self.weights.to_vec()
    }

    pub fn get_bias(&mut self) -> f64 {
        self.bias
    }
}

