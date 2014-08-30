use std::comm::{Receiver, Sender};
use uuid::Uuid;
use std::io::timer::{sleep, Timer};
use std::time::duration::Duration;
use std::sync::{Mutex, Arc};
use sync::atomic::{Relaxed, SeqCst, AtomicBool, AtomicUint};
use std::task::TaskBuilder;

use addr::Addr;
use broadcast::Broadcast;
use result::{GossipResult, GossipError};

use node::Node;
use health::{Health};
use state::State;

/// A message channel only works for broadcasts, not other server communication
/// items. Thus, these are external messages while others are most likely internal.
type MessageChan = (Sender<Broadcast>, Receiver<Broadcast>);

type ItemChan = (Sender<ProcessMessage>, Receiver<ProcessMessage>);

#[deriving(Show, PartialEq, Clone)]
pub enum ProcessMessage {
    Shutdown
}

/// Current status of a particular server.
#[deriving(Show, PartialEq, Clone)]
pub enum Status {
    Initializing,
    Running,
    ShuttingDown,
    Failing
}

/// All the possible types of messages we can send to most of the channels that communicate with
/// the server. The most common one is the `Message` variant, which initiates/receives a new
/// broadcast/gossip to the appropriate nodes.
#[deriving(Show, PartialEq)]
pub enum ServerMessage {
    /// A broadcast/gossip that can be sent and/or received!
    Message(Broadcast)
}

pub struct Process {
    cluster: Mutex<Cluster>,
    status: Mutex<Status>,
    running: AtomicBool,
    server_sender: Sender<ProcessMessage>,
    local_tx: Sender<ProcessMessage>,
    local_rx: Receiver<ProcessMessage>
}

impl Process {
    pub fn new(sender: Sender<ProcessMessage>) -> Process {
        let (tx, rx) = channel();

        Process {
            cluster: Mutex::new(Cluster::new(Uuid::new_v4().to_hyphenated_str())),
            status: Mutex::new(Initializing),
            running: AtomicBool::new(false),
            server_sender: sender,
            local_tx: tx,
            local_rx: rx
        }
    }

    pub fn bind(&self) {
        for msg in self.local_rx.iter() {
            match msg {
                Shutdown => break
            }
        }
    }
}

pub struct Cluster {
    name: String,
    members: Vec<Node>
}

impl Cluster {
    pub fn new(name: String) -> Cluster {
        Cluster {
            name: name,
            members: Vec::new()
        }
    }
}

/// A single node/member that belongs to a cluster. A server is the central piece
/// in the gossip system. Each server contains some basic metadata about itself and about
/// the cluster membership.
pub struct Server {
    id: Uuid,
    addr: Option<Addr>,
    local_tx: Sender<ProcessMessage>,
    local_rx: Receiver<ProcessMessage>,
    process: Option<Sender<ProcessMessage>>
}

impl Server {
    /// ```rust
    /// use gossip::{Server};
    ///
    /// let mut server = Server::new();
    /// ```
    pub fn new() -> Server {
        let (tx, rx) = channel();

        let server = Server {
            id: Uuid::new_v4(),
            addr: None,
            local_tx: tx,
            local_rx: rx,
            process: None
        };

        server
    }

    pub fn bind(&mut self, ip: &str, port: u16) {
        self.addr = Some(Addr::new(ip, port));

        let local_tx = self.local_tx.clone();
        let (sender, receiver) = channel();

        TaskBuilder::new().named("ProcessTask").spawn(proc() {
            let process = Process::new(local_tx);
            let tx = process.local_tx.clone();
            sender.send(tx);
            process.bind();
        });

        self.process = Some(receiver.recv());
    }

    pub fn shutdown(&self) {
        match self.process {
            Some(ref process) => process.send(Shutdown),
            None => fail!("Error)")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn should_be_initializing() {
        let server = Server::new();
    }

    #[test]
    fn should_have_no_addr_or_port() {
        let server = Server::new();
        assert!(server.addr.is_none());
    }

    #[test]
    fn should_have_empty_server_list() {
        let server = Server::new();
    }

    #[test]
    fn should_not_hang_when_calling_listen() {
        let mut server = Server::new();
        server.bind("localhost", 5666);
        server.shutdown();
    }
}
