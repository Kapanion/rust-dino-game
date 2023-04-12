use rand::Rng;

pub const INPUTS: usize = 3;

pub struct PerceptronInputs {
    pub values: [f64; INPUTS],
}

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
                perceptron.weights[i] = rng.gen_range(-1.0..1.0) + 0.01;
            }
        } else {
            for (i, x)  in weights.iter().enumerate() {
                perceptron.weights[i] = x.clone() + rng.gen_range(-0.1..0.1);
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

    pub fn predict(&mut self, perceptron_inputs: &PerceptronInputs) -> f64 {
        let mut weighted_sum: f64 = 0.0;
        for i in 0..INPUTS {
            weighted_sum += self.weights[i] * perceptron_inputs.values[i];
        }
        self.sigmoid(weighted_sum)
    }

    pub fn error(&mut self, value: f64, perceptron_inputs: &PerceptronInputs) {
        let delta = value * self.sigmoid_derivative(value);
        for (i, v) in perceptron_inputs.values.iter().enumerate() {
            self.weights[i] += delta * v * self.learning_rate;
        }
        self.bias += delta * self.learning_rate;
    }

    pub fn get_weights(&mut self) -> [f64; INPUTS] {
        self.weights
    }

    pub fn get_bias(&mut self) -> f64 {
        self.bias
    }
}

