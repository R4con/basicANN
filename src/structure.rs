#![allow(non_snake_case)]

// TODO: sort file to Network::Node::Link
//----------------------------------------------------------

struct Network {
    dimensions: Vec<i32>,
    network_data_struct: Vec<&Layer>
}

impl Network {
    pub fn create_network(&mut self, size: Vec<Vec<i32>>, standard_node_function: Nodefunction) {
        // TODO: programm this, so it will create a new neural network
        self.network_data_struct = Vec::new();

        // call create Layer
        for layer_size in dimensions {
            self.network_data_struct.push(create_layer(layer_size));
        }
    }
}

//----------------------------------------------------------

struct Layer {
    size: i32,
    Nodelist: Vec<&Node>,
}

impl Layer {
    fn create_layer(&mut self, layer_size: i32) -> &Layer {
        size = layer_size;
        self.Layer = Vec::new()

        for i in (..layer_size) {
            self.Nodelist.push(create_node());  // TODO: needs List of previous Layer for Links
        }
    }
}

//----------------------------------------------------------

enum Nodefunction {
    //standard sigmoid function
    Sigmoid,
    //for Nodes in the first Layer
    Output_only,
    //for Nodes in the Last Layer
    Input_only,
}

struct Node {
    // all links, that are connected to this Node
    connected_links: Vec<&Link>,
    // type of function, that is used to 
    fuction_type: Nodefunction,
    node_output: <f32>
}

impl Node {
    fn create_node(&self, ) { // TODO:

    }

    //will return the median of all inputs (sum / number_of_items)
    fn input_sum(&self) -> f32 {
        sum: f32;
        for single_link in connected_links.iter() {
            sum += single_link.get_link_value;
        }
        sum
    }
}
//----------------------------------------------------------

struct Link {
    weight: f32,
    input: <Node>
}

impl Link {
    fn create_Link(&self) {

    }

    //get the value of this Link (Input * weight)
    fn get_link_value(&self) ->f32 {
        input.node_output * weight
    }
}
