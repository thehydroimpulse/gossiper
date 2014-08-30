use std::comm::{Receiver, Sender};
use uuid::Uuid;
use std::io::timer::{sleep, Timer};
use std::time::duration::Duration;
use std::sync::{Mutex, Arc};
use sync::atomic::{Relaxed, SeqCst, AtomicBool, AtomicUint};
use std::task::TaskBuilder;

use addr::Addr;
use broadcast::Broadcast;
use result::{GossipResult, GossipError, UnknownError};

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

    /// Create a new unbound local server. This will create
    /// an isolated server that has not yet connected to any
    /// network system and is currently offline.
    ///
    /// Starting the server is a manual process.
    ///
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

    /// Bind the server/node to the local specified
    /// interface ports. This will bind to the TCP
    /// stack, as it's the only supported mechanism.
    ///
    /// This will go and spawn the required tasks and start
    /// running everything!
    ///
    /// ```rust
    /// #![allow(unused_must_use)]
    /// use gossip::{Server};
    ///
    /// let mut server = Server::new();
    /// server.bind("localhost", 8777);
    ///
    /// // We need to shutdown otherwise it'll hang.
    /// server.shutdown();
    /// ```
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

    /// A wrapper to the `process` Sender. Because it's an
    /// option, we'll just want to work with results because
    /// it's more composable.
    fn process_send(&self, msg: ProcessMessage) -> GossipResult<()> {
        match self.process {
            Some(ref p) => p.send(msg),
            None => return Err(GossipError::new("Failed to send message. Process is not online.", UnknownError))
        }

        Ok(())
    }

    /// Allow the ability to shutdown a running server from
    /// another task.
    ///
    /// ```rust
    /// use gossip::{Server};
    ///
    /// let mut server = Server::new();
    ///
    /// server.bind("localhost", 4555);
    /// match server.shutdown() {
    ///     Ok(_) => println!("Server has successfully been terminated!"),
    ///     Err(err) => fail!("Error while shutting down the server: {}", err)
    /// }
    /// ```
    pub fn shutdown(&self) -> GossipResult<()> {
        try!(self.process_send(Shutdown));
        Ok(())
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
