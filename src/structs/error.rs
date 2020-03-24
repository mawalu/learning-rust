use std::fmt;

pub struct BucketCapacityError;

impl fmt::Debug for BucketCapacityError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The bucket has already reached its max capacity")
    }
}