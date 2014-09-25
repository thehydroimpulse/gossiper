use std::task::TaskBuilder;

use uuid::Uuid;
use addr::Addr;
use result::{GossipResult, GossipError, NotListening};
use broadcast::Broadcast;

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

pub struct NodeTask {
    receiver: Receiver<NodeTaskMsg>,
    senders: Vec<Sender<Broadcast>>
}

impl NodeTask {
    pub fn new(tx: Sender<Broadcast>, rx: Receiver<NodeTaskMsg>) -> NodeTask {
        NodeTask {
            receiver: rx,
            senders: vec![tx]
        }
    }

    pub fn run(&mut self) {
        for message in self.receiver.iter() {
            match message {
                Shutdown => break,
                _ => {}
            }
        }
    }
}

pub struct Node {
    id: Uuid,
    addr: Option<Addr>,
    members: Vec<Peer>,
    listening: bool,
    sender: Option<Sender<NodeTaskMsg>>,
    receiver: Option<Receiver<Broadcast>>
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
            sender: None,
            receiver: None
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
    ///
    /// let mut node = Node::new();
    ///
    /// // Bind the node to the specified address:
    /// node.bind("127.0.0.1", 5899);
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
        let (local_tx, local_rx) = channel();
        let tx_task = local_tx.clone();

        TaskBuilder::new().named("NodeTask").spawn(proc() {
            let (sender, receiver) = channel::<NodeTaskMsg>();
            tx.send(sender);

            NodeTask::new(tx_task, receiver).run();
        });

        self.sender = Some(rx.recv());
        self.receiver = Some(local_rx);
    }

    /// Shutdown the current node.
    pub fn shutdown(&mut self) -> GossipResult<()> {
        self.listening = false;

        match self.sender {
            Some(ref tx) => tx.send(Shutdown),
            None => return Err(GossipError::new("The node is not running. Shutdown operation
                                                failed!", NotListening))
        }

        Ok(())
    }

    pub fn get_broadcast(&mut self) -> GossipResult<Broadcast> {
        match self.receiver {
            Some(ref rx) => Ok(rx.recv()),
            None => Err(GossipError::new("Missing receiver", NotListening))
        }
    }

    pub fn join(&mut self, peer: String) {

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
        match node.shutdown() {
            Ok(_) => {},
            Err(err) => fail!("Failed to shutdown")
        }
    }
}
