//! A broadcast represents a single message being sent out.

use uuid::Uuid;
use std::collections::hashmap::HashSet;

use result::GossipResult;
use server::{Server, Node};

/// Broadcast represents a single bi-directional communication with two
/// nodes within the cluster. The communication does **not** need to be
/// bi-directional. Responses are completely optional.
///
/// Each broadcast is tagged with a unique ID so that we may track
/// which node has received a given broadcast.
#[deriving(Show, PartialEq)]
pub struct Broadcast {
    /// A unique id for the broadcast. This allows the servers
    /// to talk about a unique broadcast in unison.
    id: Uuid,
    /// A tag represents the type of message it is without needing a physical type to decode it to.
    /// Since we may not always have that information.
    tag: String,
    /// The raw bytes of the message.
    message: Vec<u8>,
    /// A set of servers that have seen/committed the broadcast.
    committed: HashSet<Node>
}

impl Broadcast {
    /// Given a tag and message, create a new instance of Broadcast with
    /// a brand-new unique ID so that we can uniquely identify it.
    pub fn new(tag: String, message: Vec<u8>) -> Broadcast {
        Broadcast {
            id: Uuid::new_v4(),
            tag: tag,
            message: message,
            committed: HashSet::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use result::GossipResult;
}
