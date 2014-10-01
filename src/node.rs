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

/// Messages from the task to the user.
#[deriving(Show, PartialEq)]
pub enum TaskMessage {
    Shutdown
}

pub enum Message {
    BroadcastMessage(Broadcast),
    NodeMessage(TaskMessage)
}

pub struct NodeTask {
    receiver: Receiver<TaskMessage>,
    senders: Vec<Sender<Broadcast>>,
    task_msg_tx: Vec<Sender<TaskMessage>>
}

impl NodeTask {
    pub fn new(tx: Sender<Broadcast>, task_msg_tx: Sender<TaskMessage>, rx: Receiver<TaskMessage>)
        -> NodeTask {
        NodeTask {
            receiver: rx,
            senders: vec![tx],
            task_msg_tx: vec![task_msg_tx]
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.receiver.recv() {
                Shutdown => {
                    for sender in self.task_msg_tx.iter() {
                        sender.send(Shutdown);
                    }

                    break;
                }
            }
        }
    }
}

pub struct Node {
    id: Uuid,
    addr: Addr,
    members: Vec<Peer>,
    sender: Sender<TaskMessage>,
    broadcast_rx: Receiver<Broadcast>,
    task_msg_rx: Receiver<TaskMessage>
}

impl Node {

    /// Usage:
    ///
    /// ```rust
    /// use gossip::Node;
    ///
    /// // Bind the node to the specified address:
    /// let mut node = Node::new("127.0.0.1", 5899);
    ///
    /// // Kill the server:
    /// node.shutdown()
    /// ```
    pub fn new(ip: &str, port: u16) -> Node {
        let (tx, rx) = channel();
        let (broadcast_tx, broadcast_rx) = channel();
        let (task_msg_tx, task_msg_rx) = channel();

        TaskBuilder::new().named("NodeTask").spawn(proc() {
            let (sender, receiver) = channel::<TaskMessage>();
            tx.send(sender);

            let mut task = NodeTask::new(broadcast_tx, task_msg_tx, receiver);
            task.run();
        });

        Node {
            id: Uuid::new_v4(),
            addr: Addr::new(ip, port),
            members: Vec::new(),
            sender: rx.recv(),
            broadcast_rx: broadcast_rx,
            task_msg_rx: task_msg_rx
        }
    }

    /// Shutdown the current node.
    pub fn shutdown(&mut self) {
        self.sender.send(Shutdown);
    }

    pub fn get_message(&mut self) {
    }

    pub fn get_broadcast(&mut self) -> Broadcast {
        self.broadcast_rx.recv()
    }

    pub fn join(&mut self, host: &str, port: u16) {

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
