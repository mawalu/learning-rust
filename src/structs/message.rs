use serde::{Deserialize, Serialize};
use serde_bencode;

use super::util::*;

pub type MessageId = String;
pub type ClientIdentifier = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse(u8, String);

impl ErrorResponse {
	pub fn new(code: u8, message: String) -> ErrorResponse {
		ErrorResponse(code, message)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Query {
    FindNode {
        id: String,
        target: String,
    },
    GetPeers {
        id: String,
        info_hash: String,
    },
    AnnouncePeer {
        id: String,
        implied_port: Option<bool>,
        port: u16,
        token: String,
        info_hash: String
    },
    Ping {
        id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    FoundPeers {
        id: String,
        token: String,
        values: Vec<String>,
    },
    FoundPeerNodes {
        id: String,
        token: String,
        nodes: String,
    },
    FoundNodes {
        id: String,
        nodes: String,
    },
    Empty {
        id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "y")]
pub enum Message {
    #[serde(rename = "q")]
    Query {
        #[serde(rename = "t")]
        id: MessageId,
        #[serde(rename = "v")]
        client: Option<ClientIdentifier>,
        #[serde(rename = "a")]
        args: Query,
    },
    #[serde(rename = "e")]
    Error {
        #[serde(rename = "t")]
        id: MessageId,
        #[serde(rename = "v")]
        client: Option<ClientIdentifier>,
        #[serde(rename = "e")]
        error: ErrorResponse,
    },
    #[serde(rename = "r")]
    Response {
        #[serde(rename = "t")]
        id: MessageId,
        #[serde(rename = "v")]
        client: Option<ClientIdentifier>,
        #[serde(rename = "r")]
        response: Response,
    },
}

impl Message {
    pub fn from_str(input: String) -> Result<Message, serde_bencode::error::Error> {
        serde_bencode::de::from_str::<Message>(&input)
    }

    pub fn to_str(&self) -> Result<String, serde_bencode::error::Error> {
        serde_bencode::to_string(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ping() {
        let input = "d1:ad2:id40:ffffffffffffffffffffffffffffffffffffffffe1:t2:aa1:v4:aa001:y1:qe"
            .to_string();
        let deserialize = Message::from_str(input).unwrap();

        match deserialize {
            Message::Query { id, client, args } => {
                assert_eq!(id, "aa".to_owned());
                assert_eq!(client.unwrap(), "aa00".to_owned());
                match args {
                    Query::Ping { id } => {
                        assert_eq!(HashId::from_str(id).unwrap(), HashId::new([255; 20]))
                    }
                    _ => panic!("wrong query"),
                }
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_decode_find_node() {
        let input = "d1:ad2:id40:ffffffffffffffffffffffffffffffffffffffff6:target3:fffe1:t2:aa1:v4:aa001:y1:qe".to_string();
        let deserialize = Message::from_str(input).unwrap();

        match deserialize {
            Message::Query { id, client, args } => {
                assert_eq!(id, "aa".to_owned());
                assert_eq!(client.unwrap(), "aa00".to_owned());
                match args {
                    Query::FindNode { id, target: _ } => {
                        assert_eq!(HashId::from_str(id).unwrap(), HashId::new([255; 20]))
                    }
                    _ => panic!("wrong query"),
                }
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_decode_error() {
        let input = "d1:eli201e23:A Generic Error Ocurrede1:t2:aa1:v4:aa001:y1:ee".to_string();
        let deserialize = Message::from_str(input).unwrap();

        match deserialize {
            Message::Error { id, client, error } => {
                assert_eq!(id, "aa".to_owned());
                assert_eq!(client.unwrap(), "aa00".to_owned());
                assert_eq!(error.0, 201);
                assert_eq!(error.1, "A Generic Error Ocurred".to_owned());
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_decode_response() {
        let input = "d1:rd2:id40:ffffffffffffffffffffffffffffffffffffffff5:nodes17:compact_node_info5:token6:secrete1:t2:aa1:v4:aa001:y1:re".to_string();
        let deserialize = Message::from_str(input).unwrap();

        match deserialize {
            Message::Response {
                id,
                client,
                response,
            } => {
                assert_eq!(id, "aa".to_owned());
                assert_eq!(client.unwrap(), "aa00".to_owned());
                match response {
                    Response::FoundPeerNodes {
                        id: _,
                        token,
                        nodes,
                    } => {
                        assert_eq!(token, "secret".to_owned());
                        assert_eq!(nodes, "compact_node_info".to_owned());
                    }
                    _ => {
                        panic!("wrong response");
                    }
                }
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_serialize_ping_response() {
        let node_id = HashId::new([17; 20]);
        let response = Message::Response {
            id: "aa".to_owned(),
            client: None,
            response: Response::Empty {
                id: node_id.to_str(),
            },
        };

        let encoded = response.to_str().unwrap();
        assert_eq!(
            "d1:rd2:id40:1111111111111111111111111111111111111111e1:t2:aa1:y1:re".to_owned(),
            encoded
        );
    }

    #[test]
    fn test_serialize_error() {
        let response = Message::Error {
            id: "aa".to_owned(),
            client: None,
            error: ErrorResponse(
                203,
                "Protocol Error, such as a malformed packet, invalid arguments, or bad token"
                    .to_string(),
            ),
        };

        let encoded = response.to_str().unwrap();
        assert_eq!(encoded, "d1:eli203e75:Protocol Error, such as a malformed packet, invalid arguments, or bad tokene1:t2:aa1:y1:ee".to_string());
    }
}
