#![allow(non_snake_case)]

mod structure;

fn main() {
    let ANN = vec![3,5,5,3];
    let input = vec![1f32,1f32,1f32];

    let mut new_networkt: structure::Network = structure::Network::create_network(ANN, structure::Nodefunction::ReLU);

    //print!("{:?}", new_networkt.link_map);
    //print!("{:?}", new_networkt.node_struct);
    new_networkt.set_network_Input(input);
    new_networkt.propagate_forward();

    for (i, item) in new_networkt.get_network_output().iter().enumerate() {
        print!("Ouput Nr: {} = {} \n", i, item);
 
    }
}