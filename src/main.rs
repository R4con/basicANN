#![allow(non_snake_case)]

mod structure;

fn main() {
    let ANN = vec![3,5,5,3];

    let new_networkt: structure::Network = structure::Network::create_network(ANN, structure::Nodefunction::Sigmoid);
}