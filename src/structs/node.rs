use std::net::Ipv4Addr;
use std::str::FromStr;
use std::fmt;

use chrono::{DateTime,Utc,Duration};
use super::util::*;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Node {
	pub endpoint: Endpoint,
	pub node_id: HashId,
	pub last_seen: DateTime<Utc>,
	failed_queries: u8
}

impl Node {
	pub fn new(endpoint: Endpoint, node_id: HashId) -> Node {
		Node {endpoint, node_id, last_seen: Utc::now(), failed_queries: 0}
	}

	pub fn questionable(&self) -> bool {
		Utc::now() - self.last_seen > Duration::minutes(15)
	}

	pub fn distance(self, node: Node) -> HashId {
		self.node_id ^ node.node_id
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.node_id == other.node_id
	}
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct Endpoint {
	port: u16,
	addr: Ipv4Addr
}

impl Endpoint {
	pub fn new (addr: &str, port: u16) -> Result<Endpoint, std::net::AddrParseError> {
		Ok(Endpoint { addr: Ipv4Addr::from_str(addr)?, port })
	}
}

impl PartialEq for Endpoint {
	fn eq(&self, other: &Self) -> bool {
		self.addr == other.addr && self.port == other.port
	}
}

impl fmt::Display for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "NodeID: {}, last seen: {}, failed queries: {}, endpoint: {}", self.node_id, self.last_seen, self.failed_queries, self.endpoint)
	}
}

impl fmt::Display for Endpoint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.addr, self.port)
	}
}

#[cfg(test)]
mod tests {
	use chrono::Duration;
	use super::*;

	fn get_node() -> Node {
		Node::new(
			Endpoint::new("127.0.0.1", 4444).unwrap(),
			HashId::new([17; 20])
		)
	}

	#[test]
	fn test_new_node_is_not_questionable() {
		let node = get_node();

		assert_eq!(node.questionable(), false);
	}

	#[test]
	#[should_panic]
	fn test_fail_on_invalid_ip() {
		let _endpoint = Endpoint::new("256.0.0.1", 4444).unwrap();
	}

	#[test]
	fn test_node_is_questionable_after_15_minutes() {
		let mut node = get_node();

		node.last_seen = node.last_seen.checked_sub_signed(Duration::minutes(14)).unwrap();
		assert_eq!(node.questionable(), false);

		node.last_seen = node.last_seen.checked_sub_signed(Duration::minutes(1)).unwrap();
		assert!(node.questionable());

	}
}