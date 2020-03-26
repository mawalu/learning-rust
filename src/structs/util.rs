use std::cmp::Ordering;
use std::ops::BitXor;
use std::fmt;

use super::error::*;

#[derive(Copy, Clone, Eq, Debug)]
pub struct HashId {
	pub hash: [u8; 20]
}

impl HashId {
	pub fn new(hash: [u8; 20]) -> HashId {
		HashId { hash }
	}

	pub fn from_str(input: String) -> Result<HashId, InvalidHashIdError> {
		let vec = match hex::decode(input) {
			Ok(vec) => vec,
			Err(_e) => return Err(InvalidHashIdError {})
		};

		if vec.len() != 20 {
			return Err(InvalidHashIdError {})
		}

		let mut hash = [0; 20];
		for i in 0..20 {
			hash[i] = vec[i];
		}

		Ok(HashId { hash })
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_equal_to_itself() {
		let hash = HashId::new([1; 20]);
		assert!(hash == hash);
	}

	#[test]
	fn test_equal_to_same_hash() {
		let hash1 = HashId::new([1; 20]);
		let hash2 = HashId::new([1; 20]);

		assert!(hash1 == hash2);
	}

	#[test]
	fn test_not_equal_to_other_hash() {
		let hash1 = HashId::new([1; 20]);
		let hash2 = HashId::new([0; 20]);

		assert_eq!(hash1 == hash2, false);
	}

	#[test]
	fn test_simple_ordering() {
		let hash1 = HashId::new([1; 20]);
		let hash2 = HashId::new([0; 20]);

		assert!(hash1 > hash2);
		assert_eq!(hash1 <= hash2, false);
	}

	#[test]
	fn test_endianes_ordering() {
		let hash1 = HashId::new([1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
		let hash2 = HashId::new([0,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255]);

		assert!(hash1 > hash2);
	}

	#[test]
	fn test_distance_to_self_is_null() {
		let hash = HashId::new([17; 20]);
		let empty = HashId::new([0; 20]);

		assert!(hash ^ hash == empty);
	}

	#[test]
	fn test_compute_correct_distance() {
		let hash1 = HashId::new([17; 20]);
		let hash2 = HashId::new([255; 20]);

		let correct = HashId::new([238; 20]);

		assert!(hash1 ^ hash2 == correct);
	}
}