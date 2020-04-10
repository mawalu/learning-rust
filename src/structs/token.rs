use crate::structs::node::Endpoint;
use rand::Rng;
use sha1::{Sha1, Digest};

type Secret = [u8; 32];

#[derive(Debug)]
pub struct TokenAuthority {
	current_secret: Secret,
	last_secret: Secret
}

impl TokenAuthority {
	pub fn new() -> TokenAuthority {
		TokenAuthority {
			current_secret: TokenAuthority::random_secret(),
			last_secret: TokenAuthority::random_secret()
		}
	}

	pub fn rotate(&mut self) {
		self.last_secret = self.current_secret;
		self.current_secret = TokenAuthority::random_secret();
	}

	pub fn sign(&self, data: &Endpoint) -> String {
		TokenAuthority::sign_with(data, &self.current_secret)
	}

	pub fn verify(&self, token: &String, data: &Endpoint) -> bool {
		if &TokenAuthority::sign_with(data, &self.current_secret) == token {
			return true;
		}

		if &TokenAuthority::sign_with(data, &self.last_secret) == token {
			return true;
		}

		false
	}

	fn random_secret() -> Secret {
		rand::thread_rng().gen()
	}

	fn sign_with(data: &Endpoint, secret: &Secret) -> String {
		let mut hasher = Sha1::new();
		let mut input = secret.to_vec();

		input.extend_from_slice(&data.addr.octets());
		hasher.input(input);

		hex::encode(hasher.result())
	}
}

mod tests {
	use super::*;

	#[test]
	fn test_sign_token() {
		let mut signer = TokenAuthority::new();
		let old_secret = signer.current_secret;

		let data1 = Endpoint::new("127.0.0.1", 4444).unwrap();
		let data2 = Endpoint::new("127.0.0.2", 5555).unwrap();

		let token1 = signer.sign(&data1);
		let token2 = signer.sign(&data2);

		assert!(signer.verify(&token1, &data1));
		assert!(signer.verify(&token2, &data2));
		assert_eq!(signer.verify(&token1, &data2), false);

		signer.rotate();
		assert!(signer.verify(&token1, &data1));
		assert_eq!(signer.last_secret, old_secret);

		signer.rotate();
		assert_eq!(signer.verify(&token1, &data1), false);
	}
}