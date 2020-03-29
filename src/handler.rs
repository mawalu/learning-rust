use crate::structs::error::InvalidHashIdError;
use crate::structs::bucket::Kbuckets;
use crate::structs::message::*;
use crate::structs::node::*;
use crate::structs::util::HashId;

#[derive(Debug)]
pub struct DhtHandler {
    node: Node,
    buckets: Kbuckets,
    identifier: String,
}

impl DhtHandler {
    pub fn new(node: Node) -> DhtHandler {
        DhtHandler {
            node,
            buckets: Kbuckets::new(),
            identifier: "MW01".to_owned(),
        }
    }

    pub fn handle_str(&mut self, input: String) -> Option<String> {
        match Message::from_str(input) {
            Ok(message) => Some(self.handel_message(message).unwrap().to_str().unwrap()),
            Err(_) => {
                println!("Can't parse message");
                None
            }
        }
    }

    fn handel_message(&mut self, message: Message) -> Option<Message> {
        match message {
            Message::Query {
                id,
                args,
                client: _client,
            } => {
                match args {
                    Query::Ping { id: sender } => {
                        HashId::from_str(sender)
                            .map(|node_id| {
                                self.buckets.update_timestamps(&node_id);
                                self.response(&id, Response::Empty { id: self.node.node_id.to_str() })
                            })
                            .or_else(|_| -> Result<Message, InvalidHashIdError>{ Ok(self.protocol_error(&id)) })
                            .ok()
                    }
                    Query::GetPeers {
                        id: sender,
                        info_hash,
                    } => {
                        // Update last seen
                        // list of peers or closest nodes AND a token
                        None
                    }
                    Query::AnnouncePeer {
                        id: sender,
                        implied_port,
                        port,
                        token,
                    } => {
                        // Update last seen
                        // check token
                        // save peer
                        None
                    }
                    Query::FindNode { id: sender, target } => {
                        // Update last seen
                        // Compact node info of target or closest nodes
                        None
                    }
                }
            }
            Message::Response {
                id,
                client,
                response,
            } => {
                match response {
                    Response::FoundPeers { id, token, values } => {
                        // nop
                        None
                    }
                    Response::FoundPeerNodes { id, token, nodes } => {
                        // nop
                        None
                    }
                    Response::FoundNodes { id, nodes } => {
                        // check if we requested them
                        // update node buckets
                        None
                    }
                    Response::Empty { id } => {
                        // Update node last_seen
                        None
                    }
                }
            }
            Message::Error {
                id,
                error,
                client: _,
            } => {
                println!("Recieved error message for message {} {:?}", id, error);
                None
            }
        }
    }

    fn response (&self, id: &String, response: Response) -> Message {
        Message::Response {
            id: id.to_string(),
            response,
            client: Some(self.identifier.clone())
        }
    }

    fn protocol_error (&self, id: &String) -> Message {
        Message::Error {
            id: id.to_string(),
            client: Some(self.identifier.clone()),
            error: ErrorResponse::new(203, "Protocol Error, such as a malformed packet, invalid arguments, or bad token".to_string())
        }
    }
}

mod tests {
    use super::*;

    fn setup () -> DhtHandler {
        DhtHandler::new(
            Node::new(
                Endpoint::new("127.0.0.1", 4444).unwrap(),
                HashId::new([17;20])
            )
        )
    }

    #[test]
    fn test_responde_to_ping() {
        let mut dht = setup();

        let response = dht.handle_str("d1:ad2:id40:ffffffffffffffffffffffffffffffffffffffffe1:t2:aa1:v4:aa001:y1:qe"
            .to_string());

        assert_eq!(response.unwrap(), "d1:rd2:id40:1111111111111111111111111111111111111111e1:t2:aa1:v4:MW011:y1:re");
    }
}