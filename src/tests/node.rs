use crate::structs::node::*;
use crate::structs::util::*;

#[test]
fn test_new_node_is_not_questionable() {
	let endpoint = Endpoint::new("127.0.0.1", 4444).unwrap();
	let id = HashId::new([17; 20]);
	let node = Node::new(endpoint, id);

	assert_eq!(node.questionable(), false);
}

#[test]
#[should_panic]
fn test_fail_on_invalid_ip() {
	let _endpoint = Endpoint::new("256.0.0.1", 4444).unwrap();
}

