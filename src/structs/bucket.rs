use super::error::*;
use super::node::*;
use super::util::*;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct Kbuckets {
    buckets: Vec<Bucket>,
}

impl Kbuckets {
    pub fn new() -> Kbuckets {
        Kbuckets {
            buckets: vec![Bucket::new(HashId::new([255; 20]))],
        }
    }

    pub fn find_index(&mut self, id: HashId) -> Option<(usize, &mut Bucket)> {
        for (i, bucket) in self.buckets.iter_mut().enumerate() {
            if id <= bucket.upper_boundary {
                return Some((i, bucket));
            }
        }

        None
    }

    pub fn find(&mut self, id: HashId) -> Option<&mut Bucket> {
        match self.find_index(id) {
            Some((_, bucket)) => Some(bucket),
            None => None,
        }
    }

    pub fn split(&mut self, id: HashId) {
        let mut new_bucket = Bucket::new(id);
        let (index, bucket) = self.find_index(id).unwrap();

        let mut i = 0;
        while i < bucket.nodes.len() {
            if bucket.nodes[i].node_id <= new_bucket.upper_boundary {
                let node = bucket.nodes.swap_remove(i);
                new_bucket.nodes.push(node);
            } else {
                i += 1;
            }
        }

        self.buckets.insert(index, new_bucket);
    }
}

#[derive(Debug)]
pub struct Bucket {
    pub upper_boundary: HashId,
    nodes: Vec<Node>,
    last_changed: DateTime<Utc>,
}

impl Bucket {
    const SIZE: usize = 8;

    pub fn new(upper_boundary: HashId) -> Bucket {
        Bucket {
            upper_boundary,
            nodes: Vec::new(),
            last_changed: Utc::now(),
        }
    }

    pub fn insert(&mut self, node: Node) -> Result<(), BucketError> {
        if self.nodes.len() >= Bucket::SIZE {
            return Err(BucketError::new("Bucket is already full".to_string()));
        }

        if self.upper_boundary < node.node_id {
            return Err(BucketError::new(
                "NodeID is not within the buckets boundary".to_string(),
            ));
        }

        self.last_changed = Utc::now();
        self.nodes.push(node);
        Ok(())
    }

    pub fn questionables(&self) -> Vec<&Node> {
        let mut questionable = self
            .nodes
            .iter()
            .filter(|node| node.questionable())
            .collect::<Vec<&Node>>();

        questionable.sort_unstable_by(|a, b| a.last_seen.cmp(&b.last_seen));
        questionable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_node(id: [u8; 20]) -> Node {
        Node::new(Endpoint::new("127.0.0.1", 4444).unwrap(), HashId::new(id))
    }

    #[test]
    fn test_can_insert_node() {
        let mut bucket = Bucket::new(HashId::new([255; 20]));
        let node = get_node([0; 20]);

        bucket.insert(node).unwrap();

        assert_eq!(bucket.nodes.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Bucket is already full")]
    fn test_can_insert_correct_amount_of_nodes() {
        let mut bucket = Bucket::new(HashId::new([255; 20]));

        for n in 0..8 {
            let node = get_node([n; 20]);
            bucket.insert(node).unwrap();
        }

        assert_eq!(bucket.nodes.len(), 8);

        let node = get_node([10; 20]);
        bucket.insert(node).unwrap();
    }

    #[test]
    #[should_panic(expected = "NodeID is not within the buckets boundary")]
    fn test_bucket_boundaries_are_checked() {
        let mut bucket = Bucket::new(HashId::new([17; 20]));
        let node = get_node([18; 20]);

        bucket.insert(node).unwrap();
    }

    #[test]
    fn test_last_changed_is_updated() {
        let mut bucket = Bucket::new(HashId::new([17; 20]));
        let node = get_node([1; 20]);

        let before = bucket.last_changed;
        bucket.insert(node).unwrap();

        assert!(before < bucket.last_changed);
    }

    #[test]
    fn test_list_questionables() {
        let mut bucket = Bucket::new(HashId::new([255; 20]));
        let mut old_node = get_node([3; 20]);
        let mut older_node = get_node([2; 20]);
        let node = get_node([1; 20]);

        old_node.last_seen = old_node
            .last_seen
            .checked_sub_signed(Duration::minutes(16))
            .unwrap();
        older_node.last_seen = older_node
            .last_seen
            .checked_sub_signed(Duration::minutes(20))
            .unwrap();

        bucket.insert(node).unwrap();
        bucket.insert(old_node).unwrap();
        bucket.insert(older_node).unwrap();

        assert_eq!(bucket.questionables().len(), 2);

        assert_eq!(*bucket.questionables()[0], older_node);
        assert_eq!(*bucket.questionables()[1], old_node);
    }

    #[test]
    fn test_split_buckets() {
        let mut buckets = Kbuckets::new();

        assert_eq!(buckets.buckets.len(), 1);
        assert_eq!(buckets.buckets[0].upper_boundary, HashId::new([255; 20]));

        buckets.split(HashId::new([150; 20]));

        assert_eq!(buckets.buckets.len(), 2);
        assert_eq!(buckets.buckets[0].upper_boundary, HashId::new([150; 20]));
        assert_eq!(buckets.buckets[1].upper_boundary, HashId::new([255; 20]));
    }

    #[test]
    fn test_find_bucket() {
        let mut buckets = Kbuckets::new();

        let split_boundary = HashId::new([150; 20]);

        buckets.split(split_boundary);

        assert_eq!(
            buckets.find(HashId::new([10; 20])).unwrap().upper_boundary,
            split_boundary
        );
        assert_eq!(
            buckets.find(HashId::new([200; 20])).unwrap().upper_boundary,
            HashId::new([0xFF; 20])
        );
    }
}
