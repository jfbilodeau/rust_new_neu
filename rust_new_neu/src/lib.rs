use rand_distr::StandardNormal;
use rand::Rng;

pub struct Network {
    layer_sizes: Vec<usize>,
    biases: Vec<Vec<f32>>,
    weights: Vec<Vec<f32>>,
}

impl Network {
    pub fn new(layer_sizes: &Vec<usize>) -> Network {
        let mut network = Network {
            layer_sizes: layer_sizes.clone(),
            biases: vec!(),
            weights: vec!(),
        };

        network.reset(layer_sizes);

        network
    }

    pub fn layer_count(&self) -> usize {
        self.layer_sizes.len()
    }

    pub fn reset(&mut self, layer_sizes: &Vec<usize>) {
        self.layer_sizes = layer_sizes.clone();

        self.biases = vec![vec![]; self.layer_count()];
        self.weights = vec![vec![]; self.layer_count()];

        for x in 1..self.layer_count() {
            let y= self.layer_sizes[x];
            self.biases[x] = rand::thread_rng().sample_iter(StandardNormal).take(y).collect();

            self.weights[x] = rand::thread_rng()
                .sample_iter(StandardNormal)
                .take(self.layer_sizes[x])
                .collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Network;

    #[test]
    fn test_network_new() {
        let network = Network::new(&vec![784, 30, 10]);

        assert_eq!(vec![784, 30, 10], network.layer_sizes);
    }
}
