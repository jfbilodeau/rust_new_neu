#[derive(Debug, Clone)]
struct Neuron {
    bias: f32,
    activation: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            bias: 0.0,
            activation: 0.0,
            weights: vec![],
        }
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}

pub struct NewNeuNet {
    layers: Vec<Layer>,
}

impl NewNeuNet {
    pub fn new(layer_sizes: Vec<usize>) -> NewNeuNet {
        let mut network = NewNeuNet {
            layers: Vec::with_capacity(layer_sizes.len()),
        };

        for neuronCount in layer_sizes {
            let layer = Layer {
                neurons: vec![Neuron::new(); neuronCount]
            };

            network.layers.push(layer);
        }

        for layer_index in 0..network.layers.len()-1 {
            let layerK = &network.layers[layer_index];
            let layerJ = &network.layers[layer_index+1];


        }

        network
    }

    fn normalize(&mut self) {
    }
}