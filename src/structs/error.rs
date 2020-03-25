use std::fmt;

pub struct BucketError {
	message: String
}

impl BucketError {
	pub fn new(message: String) -> BucketError {
		BucketError { message }
	}
}

impl fmt::Debug for BucketError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}