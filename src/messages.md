# Each message

 * t: transaction ID. String of binary numbers
 * y: message typo. query: q, r: response, e: error
 * v: client and version identifier 2 + 2 chars. optional

# Contact encoding

 * peer: binary_ip . binary_port
 * node: peer_id . binary_ip . binary_port

# KrpcCalls

## query

 * q: query name, string
 * a: arguments, dict

## response

 * r: response, dict

## error:
 * e: error, list(code, description)

# Queries

 * id: node_id of sending / responding node

## ping

## find_node

### request
 * target: node_id

### response
 * nodes: compact node info of target node, or list of 8 closest nodes

## get_peers

### request

 * info_hash

### response

 * token: String. hmac(requesting_ip, secret)
 * values: list of peers OR nodes: 8 nodes closest to infohash

## announce_peers

### request

 * info_hash
 * port
 * token: Our token from get_peers. Check if ip matches
 * implied_port: Optional. If set use source_port as port

### response
