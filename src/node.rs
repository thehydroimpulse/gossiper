use std::task::TaskBuilder;

use uuid::Uuid;
use addr::Addr;
use result::{GossipResult, GossipError, NotListening};
use broadcast::Broadcast;
use incoming::Incoming;

/// A peer describes a member within the cluster/network that
/// is not the current one.
#[deriving(Show, PartialEq)]
pub struct Peer {
    id: Uuid,
    addr: Addr
}

/// A `Node` is a single member within the gossip protocol. Nodes that
/// join together is called a cluster. This forms the distributed system
/// in which the gossip protocol runs within.
///
/// Each Node is an equal member in the cluster. That means there isn't a
/// single leader. This has a significant trade-off and one must understand
/// it before being able to use the system correctly.
pub struct Node {
    /// Each node generates their own unique Uuid (v4) to uniquely
    /// identify each other within the cluster. Instead of saying
    /// "I'm node A", you would say "I'm node 123e4567-e89b-12d3-a456-426655440000".
    id: Uuid,

    /// A set of other members within the cluster. This forms the basic
    /// information about each Node. This doesn't, however, contain connection
    /// information and what not.
    members: Vec<Peer>
}

impl Node {
    /// Usage:
    ///
    /// ```rust
    /// use gossip::Node;
    /// let mut node = Node::new();
    /// ```
    pub fn new() -> Node {
        Node {
            id: Uuid::new_v4(),
            members: Vec::new()
        }
    }

    /// Initialize the Node to listen on the specified address/port
    /// combination. This will bootup the appropriate tasks to allow
    /// incoming connections and broadcasts.
    #[unimplemented]
    pub fn listen(&mut self, host: &str, port: u16) -> GossipResult<()> {
        Ok(())
    }

    /// Given a peer node, join it's existing cluster. Each node technically
    /// creates their own cluster automatically. Joining multiple nodes together
    /// is an explicit process. The peer node doesn't need to be the same one,
    /// but it's not a bad idea.
    #[unimplemented]
    pub fn join(&mut self, host: &str, port: u16) -> GossipResult<()> {
        Ok(())
    }

    /// Shutdown all the running tasks that are listening to new broadcasts
    /// and incoming connections. This will send one last broadcast
    /// to the current cluster notifying all other nodes of the shutdown.
    ///
    /// Afterwhich tasks will shutdown and the node will be terminated.
    #[unimplemented]
    pub fn shutdown(&mut self) {}

    /// Create a new `Incoming` iterator that iterates over newly received
    /// broadcasts that the user can handle.
    #[unimplemented]
    pub fn incoming_iter(&self) -> Incoming {
        Incoming::new(Vec::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_member_set() {
        let mut node = Node::new("localhost", 5888);
        assert_eq!(node.members.len(), 0);
        node.shutdown();
    }

    #[test]
    fn bind_listening() {
        let mut node = Node::new("127.0.0.1", 6888);
        node.shutdown();
    }
}
