use std::task::TaskBuilder;

use uuid::Uuid;
use addr::Addr;
use result::{GossipResult, GossipError, NotListening};

/// A peer describes a member within the cluster/network that
/// is not the current one.
#[deriving(Show, PartialEq)]
pub struct Peer {
    id: Uuid,
    addr: Addr
}


#[deriving(Show, PartialEq)]
pub enum NodeTaskMsg {
    Bind,
    Shutdown
}

pub struct Node {
    id: Uuid,
    addr: Option<Addr>,
    members: Vec<Peer>,
    listening: bool,
    sender: Option<Sender<NodeTaskMsg>>
}

impl Node {
    /// Usage:
    ///
    /// ```rust
    /// use gossip::Node;
    ///
    /// let node = Node::new();
    /// ```
    pub fn new() -> Node {
        Node {
            id: Uuid::new_v4(),
            addr: None,
            members: Vec::new(),
            listening: false,
            sender: None
        }
    }

    /// Usage:
    ///
    /// ```rust
    /// use gossip::Node;
    ///
    /// let node = Node::new();
    /// node.listening();
    /// ```
    pub fn listening(&self) -> bool {
        self.listening
    }

    /// Usage:
    ///
    /// ```rust
    /// use gossip::Node;
    /// use std::io::timer::sleep;
    /// use std::time::duration::Duration;
    ///
    /// let mut node = Node::new();
    ///
    /// // Bind the node to the specified address:
    /// node.bind("127.0.0.1", 5899);
    ///
    /// // Wait for 10 milliseconds then shutdown.
    /// sleep(Duration::milliseconds(10));
    ///
    /// // Kill the server:
    /// match node.shutdown() {
    ///     Ok(_) => println!("Node has successfully been terminated"),
    ///     Err(err) => fail!("Node failed to shutdown")
    /// }
    /// ```
    pub fn bind(&mut self, ip: &str, port: u16) {
        self.listening = true;
        self.addr = Some(Addr::new(ip, port));

        let (tx, rx) = channel();

        TaskBuilder::new().named("NodeBackgroundProcess").spawn(proc() {
            let (sender, receiver) = channel::<NodeTaskMsg>();

            tx.send(sender);

            for m in receiver.iter() {
                match m {
                    Shutdown => break,
                    _ => {}
                }
            }
        });

        self.sender = Some(rx.recv());
    }

    /// Shutdown the current node.
    pub fn shutdown(&mut self) -> GossipResult<()> {
        self.listening = false;

        match self.sender {
            Some(ref tx) => tx.send(Shutdown),
            None => return Err(GossipError::new("The node is not running. Shutdown operation failed!", NotListening))
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_node_without_listening() {
        let node = Node::new();
        assert_eq!(node.listening(), false);
    }

    #[test]
    fn empty_member_set() {
        let node = Node::new();
        assert_eq!(node.members.len(), 0);
    }

    #[test]
    fn bind_listening() {
        let mut node = Node::new();
        node.bind("127.0.0.1", 6888);
        assert_eq!(node.listening(), true);
    }
}
