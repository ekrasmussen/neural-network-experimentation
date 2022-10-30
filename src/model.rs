pub struct Node {
    pub value: f32,
    pub weights: Vec<f32>
}

pub struct Layer {
    pub nodes: Vec<Node>
}

pub struct NeuralNetwork {
    pub layers: Vec<Layer>
}