use super::node::*;
use super::util::*;
use super::error::*;

use chrono::{DateTime,Utc};

struct Kbuckets {
	buckets: Vec<Bucket>
}

struct Bucket {
	pub upper_boundary: HashId,
	nodes: Vec<Node>,
	last_changed: DateTime<Utc>,
}

impl Bucket {
	const SIZE: usize = 8;

	pub fn new(upper_boundary: HashId) -> Bucket {
		Bucket { upper_boundary, nodes: Vec::new(), last_changed: Utc::now() }
	}

	pub fn insert(&mut self, node: Node) -> Result<(), BucketCapacityError> {
		if self.nodes.len() >= Bucket::SIZE {
			return Err(BucketCapacityError)
		}

		Ok(self.nodes.push(node))
	}

	pub fn questionables(&self) -> Vec<&Node> {
		self.nodes
			.iter()
			.filter(|node| node.questionable())
			.collect()
	}
}