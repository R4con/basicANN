#![allow(non_snake_case)]

// TODO: sort file to Network::Node::Link
//----------------------------------------------------------

pub struct Network {
    dimensions: Vec<i32>,
    network_data_struct: Vec<Layer>
}

impl Network {
    pub fn create_network(&mut self, size: Vec<Vec<i32>>, standard_node_function: Nodefunction) {
        self.network_data_struct = Vec::new();

        // call create Layer
        for layer_size in self.dimensions {
            let tmp_layer = Layer {size: layer_size, Nodelist:  };   // Todo: add Nodelist 
            self.network_data_struct.push(tmp_layer);
        }
    }
}

//----------------------------------------------------------

struct Layer {
    size: i32,
    Nodelist: Vec<Node>,
}

impl Layer {
    
}

//----------------------------------------------------------

enum Nodefunction {
    //standard sigmoid function
    Sigmoid,
    //for Nodes in the first Layer
    OutputOnly,
    //for Nodes in the Last Layer
    InputOnly,
}

struct Node {
    // all links, that are connected to this Node
    connected_links: Vec<Link>,
    // type of function, that is the node uses
    fuction_type: Nodefunction,
    node_output: f32,
}

impl Node {
    //will return the median of all inputs (sum / number_of_items)
    fn input_sum(&self) -> f32 {
        let mut sum: f32;
        for single_link in self.connected_links {
            sum += single_link.get_link_value();
        }
        sum
    }
}
//----------------------------------------------------------

struct Link {
    weight: f32,
    input: &Node, // ! <-------- not working, lifetime parameter missing
}

impl Link {
    fn adjust_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }

    //get the value of this Link (Input * weight)
    fn get_link_value(&self) ->f32 {
        self.input.node_output * self.weight
    }
}


/*

fn create_node(&self, nodetype: Nodefunction) -> &Node { 
let mut links: Vec<Link> = Vec<Link>::new();
}

fn create_layer(&self, layer_size: i32) -> Layer {
    self.size = layer_size;

    for i in ..layer_size {
        self.Nodelist.push(Node::create_node(Nodefunction::Sigmoid));  // TODO: needs List of previous Layer for Links
    }
}

*/