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
        //will return the median of all inputs (sum / number_of_items)
        fn input_sum(&self) -> f32 {
            sum: f32;
            for single_link in connected_links.iter() {
                sum += single_link.get_link_value;
            }
            sum
        }
        
        //----------------------------------------------------------

        struct Link {
            weight: f32,
            input: <Node>
        }

        impl Link {
            //get the value of this Link (Input * weight)
            fn get_link_value(&self) ->f32 {
                input.node_output * weight
            }
        }
    }
}