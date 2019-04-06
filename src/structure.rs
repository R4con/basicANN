#![allow(non_snake_case)]
use std::collections::HashMap;

//----------------------------------------------------------

pub struct Network {
    link_map: HashMap<((i16, i16),(i16, i16)), Link>, //((i16, i16), (i16, i16)) = ((from layer, node),(to layer, node))
    node_struct: Vec<Layer>
}

impl Network {
    pub fn create_network(network_size: Vec<i32>, standard_node_function: Nodefunction) -> Network {
        let mut node_struct: Vec<Layer> = Vec::new();
        let mut link_map: HashMap<((i16, i16),(i16, i16)), Link> = HashMap::new();  // todo add elements to hashmap
        
        for (num, layer_size) in network_size.iter().enumerate() {
            // Input Layer
            if num.clone() == 0 {   
                node_struct.push(Layer::create_layer(network_size[0], 0, Nodefunction::OutputOnly));
            }
            // Output Layer
            else if num.clone() == network_size.len() -1{
                node_struct.push(Layer::create_layer(network_size.last().expect("network_size.last() returned an error").clone(), num.clone(), Nodefunction::InputOnly));
            } 
            // Hiddel Layers
            else {
                node_struct.push(Layer::create_layer(layer_size.clone(), num as i32, Nodefunction::Sigmoid)); // ? num clone useful ?
            }
        }
        
        //create a Hashmap, that contains all links
        for layer_number in 1..network_size.len() {
            for org_node in 0..network_size[layer_number - 1] {
                for tar_node in 0..network_size[layer_number] {
                    link_map.insert((((layer_number -1) as i16, org_node as i16),(layer_number as i16,tar_node as i16)),
                    Link::create_link(0.0, (((layer_number -1) as i16, org_node as i16),(layer_number as i16,tar_node as i16))));
                }
            }
        }

        let ret_network: Network = Network {node_struct: node_struct, link_map: link_map }; // todo add hashmap as attribut
        ret_network
    }

    pub fn propagate_forward(&mut self) {
        for (i, layer) in self.node_struct.iter().enumerate() {
            if node.fuction_type != Nodefunction::OutputOnly {      // todo add compare option for notefunction
                    continue;
            }
            for (n, node) in  layer.Nodelist.iter().enumerate() {
                for m in self.node_struct[i-1].Nodelist.iter().enumerate() {
                    self.link_map.get((i-1, m),(i, n)).weight;
                    // todo do calculation
                }
                let node_output_value: f32 = 0.0;
                
            }
        }
    }
}

//----------------------------------------------------------

struct Layer {
    size: i32,
    Nodelist: Vec<Node>,
}

impl Layer {
    fn create_layer(size: i32, layer_position: i32, function: Nodefunction) -> Layer {   //input_link_list: Option<Vec<Link>>
        let mut node_list: Vec<Node> = Vec::new();

        // add nodes for the first layer 
        for i in 0..size {
            node_list.push( Node::create_node( function, 0.0, (layer_position, i)) );
        }

        let mut ret_layer: Layer = Layer {size: size.clone(), Nodelist: node_list};
        ret_layer
    }
}

//----------------------------------------------------------

struct Node {
    // all links, that are connected to this Node
    connected_input_links: Option<Vec<Link>>,
    // type of function, that is the node uses
    fuction_type: Nodefunction,
    node_output: f32,
    node_position: (i32,i32),
}

impl Node {
    //will return the median of all inputs (sum / number_of_items)
    fn create_node(function: Nodefunction, output: f32, position: (i32,i32)) -> Node {
        let mut node: Node = Node {
            fuction_type: function,
            node_output: output, 
            connected_input_links: None,
            node_position: position
        };
        node
    }

    fn input_sum(&self) -> f32 {    // ? needs Hashmap, so move to Network function
        if let Some(Vec) = link_list {
            let mut sum: f32;
            for single_link in self.connected_input_links {
                sum += single_link.get_link_value();
            }
            sum
        }
    }
}

//----------------------------------------------------------

struct Link {
    weight: f32,
}

impl Link {
    fn create_link(weight: f32, input: ((i16,i16),(i16,i16))) -> Link {
        let mut ret_link: Link = Link {weight, input};
        ret_link
    }

    fn adjust_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
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