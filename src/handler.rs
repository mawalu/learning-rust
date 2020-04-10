use crate::structs::error::InvalidHashIdError;
use crate::structs::bucket::Kbuckets;
use crate::structs::message::*;
use crate::structs::node::*;
use crate::structs::util::HashId;
use crate::structs::token::TokenAuthority;

#[derive(Debug)]
pub struct DhtHandler {
    node: Node,
    buckets: Kbuckets,
    identifier: String,
    peers: PeerList,
    signer: TokenAuthority
}

impl DhtHandler {
    pub fn new(node: Node) -> DhtHandler {
        DhtHandler {
            node,
            buckets: Kbuckets::new(),
            identifier: "MW01".to_owned(),
            peers: PeerList::new(),
            signer: TokenAuthority::new()
        }
    }

    pub fn handle_str(&mut self, input: String) -> Option<String> {
        let endpoint = Endpoint::new("127.0.0.1", 4444).unwrap();

        match Message::from_str(input) {
            Ok(message) => Some(self.handel_message(message, endpoint).unwrap().unwrap().to_str().unwrap()),
            Err(_) => {
                println!("Can't parse message");
                None
            }
        }
    }

    fn handel_message(&mut self, message: Message, endpoint: Endpoint) -> Result<Option<Message>, InvalidHashIdError> {
        match message {
            Message::Query {
                id,
                args,
                client: _client,
            } => {
                match args {
                    Query::Ping { id: sender_string } => {
                        let sender = HashId::from_str(sender_string)?;
                        self.buckets.update_timestamps(&sender);
                        self.response(&id, Response::Empty { id: self.node.node_id.to_str() })
                    }
                    Query::GetPeers {
                        id: sender_string,
                        info_hash: info_hash_string,
                    } => {
                        let sender = HashId::from_str(sender_string)?;
                        let info_hash = HashId::from_str(info_hash_string)?;
                        self.buckets.update_timestamps(&sender);

                        match self.peers.get(&info_hash) {
                            Some (peers) => self.response(&id, Response::FoundPeers {
                                id: self.node.node_id.to_str(),
                                token: self.signer.sign(&endpoint),
                                values: peers.iter().map(Endpoint::to_string).collect()
                            }),
                            None => self.response(&id, Response::FoundPeerNodes {
                                id: self.node.node_id.to_str(),
                                token: self.signer.sign(&endpoint),
                                nodes: self.buckets.find_closest_nodes(&info_hash)
                                    .unwrap_or(Vec::<Node>::new())
                                    .iter().map(Node::to_str).collect()
                            })
                        }
                    }
                    Query::AnnouncePeer {
                        id: sender_string,
                        info_hash: info_hash_string,
                        implied_port,
                        mut port,
                        token,
                    } => {
                        let sender = HashId::from_str(sender_string)?;
                        let info_hash = HashId::from_str(info_hash_string)?;
                        self.buckets.update_timestamps(&sender);

                        if !self.signer.verify(&token, &endpoint) {
                            return self.protocol_error(&id)
                        }

                        if implied_port.unwrap_or(false) {
                            port = endpoint.port
                        }

                        let mut node = endpoint.clone();
                        node.port = port;

                        match self.peers.get_mut(&info_hash) {
                            Some(node_list) => node_list.push(endpoint),
                            None => {
                                self.peers.insert(info_hash, vec!(node));
                            }
                        };

                        self.response(&id, Response::Empty {
                            id: self.node.node_id.to_str()
                        })
                    }
                    Query::FindNode { id: sender_string, target: target_string } => {
                        let sender = HashId::from_str(sender_string)?;
                        let target = HashId::from_str(target_string)?;
                        self.buckets.update_timestamps(&sender);

                        let mut closest = self.buckets.find_closest_nodes(&target)
                            .unwrap_or(Vec::<Node>::new());

                        if closest.len() > 0 && closest[0].node_id == target {
                            closest.truncate(1);
                        }

                        self.response(&id, Response::FoundNodes {
                            id: self.node.node_id.to_str(),
                            nodes: closest.iter().map(Node::to_str).collect()
                        })
                    }
                }
            }
            Message::Response {
                id,
                client,
                response,
            } => {
                match response {
                    Response::FoundPeers { id: _, token: _, values: _ } => {
                        // nop
                        Ok(None)
                    }
                    Response::FoundPeerNodes { id: _, token: _, nodes: _ } => {
                        // nop
                        Ok(None)
                    }
                    Response::FoundNodes { id: sender_string, nodes } => {
                        let sender = HashId::from_str(sender_string);
                        // check if we requested them
                        // update node buckets
                        Ok(None)
                    }
                    Response::Empty { id: sender_string } => {
                        let sender = HashId::from_str(sender_string)?;
                        self.buckets.update_timestamps(&sender);
                        Ok(None)
                    }
                }
            }
            Message::Error {
                id,
                error,
                client: _,
            } => {
                println!("Recieved error message for message {} {:?}", id, error);
                Ok(None)
            }
        }
    }

    fn response (&self, id: &String, response: Response) -> Result<Option<Message>, InvalidHashIdError> {
        Ok(Some(Message::Response {
            id: id.to_string(),
            response,
            client: Some(self.identifier.clone())
        }))
    }

    fn protocol_error (&self, id: &String) -> Result<Option<Message>, InvalidHashIdError> {
        Ok(Some(Message::Error {
            id: id.to_string(),
            client: Some(self.identifier.clone()),
            error: ErrorResponse::new(203, "Protocol Error, such as a malformed packet, invalid arguments, or bad token".to_string())
        }))
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