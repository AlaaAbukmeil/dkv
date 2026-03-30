#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

pub struct Node {
    pub id: String,
    pub role: Role,
    pub term: u64,
    pub voted_for: Option<String>,
    pub peers: Vec<String>, // addresses like "http://127.0.0.1:50052"
}

impl Node {
    pub fn new(id: String, peers: Vec<String>) -> Self {
        Node {
            id,
            role: Role::Follower,
            term: 0,
            voted_for: None,
            peers,
        }
    }
}