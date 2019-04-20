#![allow(non_snake_case)]
use std::collections::HashMap;

//----------------------------------------------------------

#[derive(Debug)]
pub struct Network {
    pub link_map: HashMap<((i16, i16),(i16, i16)), Link>, //((i16, i16), (i16, i16)) = ((from layer, node),(to layer, node))
    pub layer_struct: Vec<Layer>
}

impl Network {
    pub fn create_network(network_size: Vec<i32>, standard_node_function: Nodefunction) -> Network {
        let mut layer_struct: Vec<Layer> = Vec::new();
        let mut link_map: HashMap<((i16, i16),(i16, i16)), Link> = HashMap::new();  // todo add elements to hashmap
        
        for (num, layer_size) in network_size.iter().enumerate() {
            // Input Layer
            if num == 0 {   
                layer_struct.push(Layer::create_layer(network_size[0], 0, Nodefunction::OutputOnly));
            }
            // Output Layer
            else if num == network_size.len() -1{
                layer_struct.push(Layer::create_layer(network_size.last().expect("network_size.last() returned an error").clone(), num as i32, Nodefunction::InputOnly));
            } 
            // Hiddel Layers
            else {
                layer_struct.push(Layer::create_layer(layer_size.clone(), num as i32, standard_node_function.clone())); //? num clone useful ?
            }
        }
        
        //create a Hashmap, that contains all links
        for layer_number in 1..(network_size.len()) {
            for org_node in 0..(network_size[layer_number -1]) {
                for tar_node in 0..(network_size[layer_number]) {
                    link_map.insert(( ((layer_number -1) as i16, org_node as i16),(layer_number as i16,tar_node as i16)) ,
                    Link::create_link(0.5) );
                }
            }
        }

        let ret_network: Network = Network {layer_struct: layer_struct, link_map: link_map }; //? todo add hashmap as attribut
        ret_network
    }

    pub fn propagate_forward(&mut self) {   // ! works, dont touch !
        for i in 1..(self.layer_struct.len() ) {     //layer 1 - max
            let mut node_output_value: Vec<f32> = Vec::new();

            for (n, _node) in self.layer_struct[i].Nodelist.iter().enumerate() { //node 0 - max
                node_output_value.push(0.0);

                for (m, prev_node) in self.layer_struct[i-1].Nodelist.iter().enumerate() { //node 0 - max in prev layer
                    let key = (( (i-1) as i16, m as i16),(i as i16, n as i16));

                    match self.link_map.get(&key) {
                        Some(link) => {
                            node_output_value[n] += link.weight * prev_node.node_output;
                        },
                        None => panic!("Tried to get key of an not existing Link: {} {} , {} {}", (i-1).to_string(), n.to_string(), i.to_string(), m.to_string()),
                    }
                }
            }
            // node_output muss noch festgelegt werden (mit nodefunction)
            for t in 0..node_output_value.len() {   //
                print!("{}: {} | ", t, node_output_value[t]);
                self.layer_struct[i].Nodelist[t].node_output = self.layer_struct[i].Nodelist[t].get_nodefunction_output(&node_output_value[t]);
                print!("out: {} \n", self.layer_struct[i].Nodelist[t].node_output);
            }
        }
    }

    pub fn propagate_backward(&mut self, desired_output: Vec<f32>) {

    }

    pub fn set_network_Input(&mut self, new_input: Vec<f32>) {
        if new_input.len() != self.layer_struct[0].Nodelist.len() {
            panic!("The given input is not the same size as the Input Layer!");
        }

        for (i, item) in self.layer_struct[0].Nodelist.iter_mut().enumerate() {
            item.node_output = new_input[i];
        }
    }

    pub fn get_network_output(&self) -> Vec<f32> {
        let mut ret = Vec::new();

        match self.layer_struct.last() {
            Some(item) => {
                for i in 0..(item.Nodelist.len()) {
                    ret.push(item.Nodelist[i].node_output);
                }
            },
            None => panic!("The last Element in the layer_struct could not be found !"),
        }
        ret
    }
}

//----------------------------------------------------------
#[derive(Debug)]
pub struct Layer {
    size: i32,
    Nodelist: Vec<Node>,
}

impl Layer {
    fn create_layer(size: i32, layer_position: i32, function: Nodefunction) -> Layer {   //input_link_list: Option<Vec<Link>>
        let mut node_list: Vec<Node> = Vec::new();

        // add nodes for the first layer 
        for i in 0..(size) {
            node_list.push( Node::create_node( function.clone(), 0.0, (layer_position, i)) );
        }

        let ret_layer: Layer = Layer {size: size.clone(), Nodelist: node_list};
        ret_layer
    }
}

//----------------------------------------------------------
#[derive(Debug)]
struct Node {
    // all links, that are connected to this Node
    connected_input_links: Option<Vec<Link>>,
    // type of function, that is the node uses
    function_type: Nodefunction,
    node_output: f32,
    node_position: (i32,i32),
}

impl Node {
    //will return the median of all inputs (sum / number_of_items)
    fn create_node(function: Nodefunction, output: f32, position: (i32,i32)) -> Node {
        let node: Node = Node {
            function_type: function,
            node_output: output, 
            connected_input_links: None,
            node_position: position
        };
        node
    }

    fn get_nodefunction_output(&self, input: &f32) -> f32 {
        let input: f32 = input.clone();
        match &self.function_type {
            Sigmoid => 1.0/(1.0 + std::f32::consts::E.powf(-input)),
            OutputOnly => input,
            InputOnly => 1.0/(1.0 + std::f32::consts::E.powf(-input)),
            ReLU => {
                if input < 0.0 {
                    0.0
                } else {
                    input
                }
            }
        }
    }
}

//----------------------------------------------------------
#[derive(Debug)]
pub struct Link {
    weight: f32,
}

impl Link {
    fn create_link(weight: f32) -> Link {
        let ret_link: Link = Link {weight};
        ret_link
    }

    fn adjust_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

//----------------------------------------------------------

#[derive(PartialEq, Clone, Debug)]
pub enum Nodefunction {
    //standard sigmoid function
    Sigmoid,
    //for Nodes in the first Layer
    OutputOnly,
    //for Nodes in the Last Layer
    InputOnly,

    ReLU,
}