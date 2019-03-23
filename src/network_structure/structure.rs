#![allow(non_snake_case)]

struct Network {
    dimensions: Vec<Vec<i32>>,
    Vec<&Node>,
}

enum Nodefunction {
    Sigmoid,
}

struct Node {
    connected_links: Vec<Link>,
    fuction_type: Nodefunction,
}

struct Link {
    weight: f32,
}