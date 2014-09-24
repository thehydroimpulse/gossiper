use std::comm::{Receiver, Sender};
use uuid::Uuid;
use std::io::timer::{sleep, Timer};
use std::time::duration::Duration;
use std::sync::{Mutex, Arc, RWLock};
use sync::atomic::{Relaxed, SeqCst, AtomicBool, AtomicUint};
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::task::TaskBuilder;

use addr::Addr;
use broadcast::Broadcast;
use result::{GossipResult, GossipError, UnknownError};

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
    cluster: RWLock<Cluster>,
    status: RWLock<Status>,
    running: AtomicBool
}

impl Process {
    pub fn new(sender: Sender<Broadcast>) -> Process {
        Process {
            cluster: RWLock::new(Cluster::new(Uuid::new_v4().to_hyphenated_string())),
            status: RWLock::new(Initializing),
            running: AtomicBool::new(false)
        }
    }

    pub fn bind(&self, addr: Addr) {
        let listener = TcpListener::bind(addr.ip.as_slice(), addr.port);
        let mut listener = listener.listen();
    }

    pub fn shutdown(&self) -> GossipResult<()> {
        Ok(())
    }
}

pub struct Cluster {
    id: String,
    members: Vec<String>
}

impl Cluster {
    pub fn new(name: String) -> Cluster {
        Cluster {
            id: name,
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
    process: Arc<Process>,
    rx: Receiver<Broadcast>
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

        Server {
            id: Uuid::new_v4(),
            addr: None,
            process: Arc::new(Process::new(tx.clone())),
            rx: rx
        }
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
        let addr = Addr::new(ip, port);
        self.addr = Some(addr.clone());

        let process = self.process.clone();

        let mut value = self.process.status.write();
        *value = Running;

        TaskBuilder::new().named("ProcessTask").spawn(proc() {
            process.bind(addr);
        });
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
        self.process.shutdown()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use addr::Addr;

    #[test]
    fn process_bind_should_not_write_status() {
        let (sender, receiver) = channel();
        let process = Process::new(sender);
        assert!(*(process.status.read()) == Initializing);
        process.bind(Addr::new("localhost", 4444));
        assert!(*(process.status.read()) == Initializing);
        process.shutdown();
    }

    #[test]
    fn server_should_change_status_when_listening() {
        let mut server = Server::new();
        server.bind("localhost", 5999);
        assert!(*(server.process.status.read()) == Running);
        server.shutdown();
    }

    #[test]
    fn should_have_no_addr_or_port() {
        let server = Server::new();
        assert!(server.addr.is_none());
    }

    #[test]
    fn should_have_empty_server_list() {
        let server = Server::new();
        assert!(server.process.cluster.read().members.len() == 0);
    }

    #[test]
    fn should_not_hang_when_calling_listen() {
        let mut server = Server::new();
        server.bind("localhost", 5666);
        server.shutdown();
    }
}
