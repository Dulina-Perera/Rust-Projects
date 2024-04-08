use super::matrix::Matrix;

pub struct Network<'a> {
    pub layers: Vec<usize>,
    pub weights: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub activation: Activation<'a>,
    pub data: Vec<Matrix>,
}

impl Network<'a> {
    pub fn new<'a>(layers: Vec<usize>, activation: Activation<'a>) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        return Network {
            layers: layers,
            weights: weights,
            biases: biases,
            activation: activation,
            data: vec![],
        };
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Input size does not match network input size.");
        }

        let mut curr = Matrix::from(vec![inputs]).transpose();
        self.data = vec![curr.clone()];

        for i in 0..self.layers.len() - 1 {
            curr = self.weights[i]
                .dot_multiply(&curr)
                .add(&self.biases[i])
                .map(&self.activation.func);
            self.data.push(curr.clone());
        }

        return curr.data.to_owned();
    }
}
