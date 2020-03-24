use crate::structs::util::*;

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