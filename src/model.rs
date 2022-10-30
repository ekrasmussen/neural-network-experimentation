pub struct Node {
    pub value: f32,
    pub weights: Vec<f32>
}

impl Node {
    //TODO: implement methods
}

pub struct Layer {
    pub nodes: Vec<Node>
}

impl Layer {
    //TODO: implement methods
}

pub struct NeuralNetwork {
    pub layers: Vec<Node>
}

impl NeuralNetwork {
    pub fn new() -> Self {
        println!("Im new!");
        Self { layers : Vec::new() }
    }
}