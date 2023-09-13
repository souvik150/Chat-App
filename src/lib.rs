use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derivce(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Client{
    Join { name: Arc<String> },
    Post {
        name: Arc<String>,
        message: Arc<String>,
    },
}

#[derivce(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        name: Arc<String>,
        message: Arc<String>,
    },
    Error(String)
}