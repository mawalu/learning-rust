mod structs;

use crate::structs::node::*;
use crate::structs::util::*;

fn main() {
    let parsed =
        Node::from_str("38636177a357835555a2be8b36b6a2c80bd2bd536a9d70e3b1d3".to_string()).unwrap();
    println!("{:?}", parsed);

    let endpoint1 = Endpoint::new("127.0.0.1", 4444).unwrap();
    let endpoint2 = Endpoint::new("127.0.0.1", 5555).unwrap();

    let id1 = HashId::new([17; 20]);
    let id2 = HashId::new([255; 20]);

    let node1 = Node::new(endpoint1, id1);
    let node2 = Node::new(endpoint2, id2);

    println!("{}", node1);
    println!("{}", node2);

    println!("{}", node1.distance(node2));
}
