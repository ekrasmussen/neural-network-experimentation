//Node

pub struct Node {
    pub value: f32,
    pub weights: Vec<f32>
}

impl Node {
    pub fn new() -> Self {
        Self { value: 1.0, weights: Vec::new()}
    }
}

//Layer

pub struct Layer {
    pub nodes: Vec<Node>
}

impl Layer {
    pub fn new() -> Self {
        Self { nodes : Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_nodes(&mut self, node_amount: u32) {
        for _n in 0..node_amount {
            self.nodes.push(Node::new())
        }
    }

}

//Neural Network

pub struct NeuralNetwork {
    pub layers: Vec<Layer>
}

impl NeuralNetwork {
    pub fn new() -> Self {
        Self { layers : Vec::new() }
    }

    pub fn create_layer(&mut self, node_amount: u32) {
        let mut new_layer = Layer::new();
        new_layer.add_nodes(node_amount);
        self.layers.push(new_layer);
    }
}