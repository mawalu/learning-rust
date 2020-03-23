use std::net::Ipv4Addr;
use std::str::FromStr;
use std::fmt;

use chrono::{DateTime,Utc,Duration};
use super::util::*;

pub struct Node {
	pub endpoint: Endpoint,
	pub node_id: HashId,
	last_seen: DateTime<Utc>,
	failed_queries: u8
}

impl Node {
	pub fn new(endpoint: Endpoint, node_id: HashId) -> Node {
		Node {endpoint, node_id, last_seen: Utc::now(), failed_queries: 0}
	}

	pub fn questionable(&self) -> bool {
		Utc::now() - self.last_seen > Duration::minutes(15)
	}

	pub fn distance(&self, hash: &HashId) -> HashId {
		self.node_id.distance_hash(hash)
	}
}

pub struct Endpoint {
	port: u16,
	addr: Ipv4Addr
}

impl Endpoint {
	pub fn new (addr: &str, port: u16) -> Result<Endpoint, std::net::AddrParseError> { 
		match Ipv4Addr::from_str(addr) {
			Ok(parsed) => Ok(Endpoint { addr: parsed, port }),
			Err(error) => Err(error) 
		}
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