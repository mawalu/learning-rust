use std::fmt;

pub struct HashId {
	pub hash: [u8; 20]
}

impl HashId {
	pub fn new(hash: [u8; 20]) -> HashId {
		HashId { hash }
	}

	pub fn distance_hash(&self, hash: &HashId) -> HashId {
		let mut result: [u8; 20] = [0; 20];

		for n in 0..20 {
			result[n] = self.hash[n] ^ hash.hash[n];
		}

		HashId { hash: result }
	}
}

impl fmt::Display for HashId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", hex::encode(self.hash))
	}
}