use serde::{Serialize, Deserialize};
use serde_bencode::de;

use super::util::*;

pub type MessageId = String;
pub type ClientIdentifier = String;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Arguments {
	#[serde(rename = "p")]
	Ping {
		id: String
	},
	FindNode {

	},
	GetPeers {

	},
	AnnouncePeer {

	}
}

#[derive(Debug, Deserialize)]
#[serde(tag = "y")]
pub enum Message {
	#[serde(rename = "q")]
	Query {
		#[serde(rename = "t")]
		id: MessageId,
		#[serde(rename = "v")]
		client: Option<ClientIdentifier>,
		#[serde(rename = "a")]
		args: Arguments
	},
	#[serde(rename = "e")]
	Error {
		#[serde(rename = "t")]
		id: MessageId,
		#[serde(rename = "v")]
		client: Option<ClientIdentifier>,
	},
	Response {

	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_decode_ping() {
		let input = "d1:ad2:id20:ffffffffffffffffffffe1:q4:ping1:t2:aa1:y1:qe";
		let deserialize = de::from_str::<Message>(input).unwrap();

		match deserialize {
			Message::Query { id, client, args } => {
				assert_eq!(id, "aa".to_owned());
				match args {
					Arguments::Ping { id } => assert_eq!(id, "ffffffffffffffffffff".to_owned()),
					_ => panic!("wrong query")
				}
			},
			_ => panic!("wrong command"),
		}
	}
}