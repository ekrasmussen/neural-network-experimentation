mod model;

fn main() {
    // let node_test = model::Node {
    //     value: 2.0,
    //     weights: vec![1.0, 2.1, 2.0]
    // };

    // println!("{}",node_test.value);

    let mut network = model::NeuralNetwork::new();

    network.create_layer(15);

    println!("{}",network.layers[0].len());
}
