use oorandom::Rand32;

use crate::get_time;

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
    fn activate(&self, value: f64) -> f64 {
        return 1.0 / (1.0 + (-1.0 * (value + self.bias)).exp());
    }

    pub fn predict(&mut self, perceptron_inputs: &PerceptronInputs) -> f64 {
        let mut weighted_sum: f64 = 0.0;
        for i in 0..INPUTS {
            weighted_sum += self.weights[i] * perceptron_inputs.values[i];
        }
        self.activate(weighted_sum)
    }

    pub fn adjust(&mut self, delta: f64, perceptron_inputs: &PerceptronInputs) {
        for i in 0..INPUTS {
            self.weights[i] += delta * perceptron_inputs.values[i] * self.learning_rate;
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

pub fn init_perceptron(bias: f64, learning_rate: f64, input: usize ) -> Perceptron {

    let mut rng = Rand32::new(get_time());
    let mut perceptron = Perceptron {
        bias: bias,
        learning_rate: learning_rate,
        weights: [0.0; INPUTS],
    };

    for i in 0..input {
        perceptron.weights[i] = f64::from(rng.rand_range(0..999)) / 1000. + 0.01;

        if rng.rand_range(0..10) < 5 {
            perceptron.weights[i] *= -1.
        }
    }

    perceptron
}
