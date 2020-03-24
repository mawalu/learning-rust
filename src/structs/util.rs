use std::cmp::Ordering;
use std::ops::BitXor;
use std::fmt;

#[derive(Copy, Clone, Eq)]
pub struct HashId {
	pub hash: [u8; 20]
}

impl HashId {
	pub fn new(hash: [u8; 20]) -> HashId {
		HashId { hash }
	}
}

impl BitXor for HashId {
	type Output = Self;

	fn bitxor(self, other: HashId) -> HashId {
		let mut result: [u8; 20] = [0; 20];

		for n in 0..20 {
			result[n] = self.hash[n] ^ other.hash[n];
		}

		HashId { hash: result }
	}
}

impl Ord for HashId {
	fn cmp(&self, other: &Self) -> Ordering {
		for n in 0..20 {
			if self.hash[n] > other.hash[n] {
				return Ordering::Greater;
			}

			if self.hash[n] < other.hash[n] {
				return Ordering::Less;
			}
		}

		Ordering::Equal
	}
}

impl PartialOrd for HashId {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for HashId {
	fn eq(&self, other: &Self) -> bool {
		self.hash == other.hash
	}
}

impl fmt::Display for HashId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", hex::encode(self.hash))
	}
}