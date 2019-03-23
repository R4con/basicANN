#![allow(non_snake_case)]

// TODO: sort file to Network::Node::Link
//----------------------------------------------------------

struct Network {
    dimensions: Vec<Vec<i32>>,
    Vec<&Node>,
}

impl Network {
    fn create_network(&mut self, size: Vec<Vec<i32>>, standard_node_function: Nodefunction) {
        // TODO: programm this, so it will create a new neural network
    }
}

//----------------------------------------------------------

enum Nodefunction {
    Sigmoid,
    Output_only,
    Input_only,
}

struct Node {
    connected_links: Vec<&Link>,
    fuction_type: Nodefunction,
    node_output: <f32>
}

impl Node {
    fn input_sum(&self) -> f32 {
        sum: f32;
        for single_link in connected_links.iter() {
            sum += single_link.get_link_value;
        }
    }
}

//----------------------------------------------------------

struct Link {
    weight: f32,
    input: <Node>
}

impl Link {
    fn get_link_value(&self) ->f32 {
        input.node_output * weight
    }
}