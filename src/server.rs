//! Server implementation.

use std::collections::hashmap::HashSet;
use std::comm::{Receiver, Sender};
use std::rc::{Rc, Weak};
use uuid::Uuid;
use std::io::timer::Timer;
use std::time::duration::Duration;

use addr::Addr;
use broadcast::Broadcast;
use result::{GossipResult, GossipError};

use node::Node;
use health::{Health};
use state::State;

#[deriving(Show, PartialEq)]
pub enum ShutdownReason {
    UserInitiatedShutdown,
    NetworkFailure,
    Failure
}

/// Messages that can be sent and received to and from the server task.
#[deriving(Show, PartialEq)]
pub enum ServerMsg {
    /// Receive a particular broadcast. We will commit it in our log that can persist to disk.
    Message(Broadcast),
    /// A signal to kill the current server. This will send a IAmShuttingDown message as
    /// a gossip message to let the cluster know why it's shutting down.
    Shutdown(ShutdownReason),
    /// Kill a specific node in the cluster. This is a state change rather than a gossip. This will
    /// remove a specific node from the cluster.
    KillNode(Node)
}

pub struct ServerTask {
    /// Part of a channel that the server sends messages to.
    receiver: Receiver<ServerMsg>,
    /// Part of a channel that communicates with the server.
    sender: Sender<ServerMsg>
}

impl ServerTask {
    ///
    /// ```rust
    /// use gossip::{ServerTask};
    ///
    /// let mut task = ServerTask::create("127.0.0.1", 4555);
    /// task.close();
    /// ```
    pub fn create(ip: &str, port: u16) -> ServerTask {
        // Create an intermediate channel.
        let (tx, rx) = channel();
        let (sender, receiver) = channel();
        let addr = ip.to_string();

        spawn(proc() {
            // Create a new server.
            let mut server = Server::new(sender.clone());
            tx.send(server.sender.clone());
            server.listen(addr.as_slice(), port);
        });


        ServerTask {
            receiver: receiver,
            sender: rx.recv()
        }
    }

    pub fn shutdown(&mut self, time: Duration) {
        let mut timer = Timer::new().unwrap();
        timer.sleep(time);
        self.close();
    }

    pub fn close(&mut self) {
        self.sender.send(Shutdown(UserInitiatedShutdown));
    }

    pub fn recv(&mut self) -> ServerMsg {
        self.receiver.recv()
    }
}

/// A server/node/peer is the most atomic unit within a cluster. Each node is equal with it's peers,
/// thus we don't have any leader or election processes. Each server is identified with a unique ID
/// that is randomly generated, along with the appropriate state.
///
/// A server requires a bit of metadata about the cluster, the cluster's state and things like
/// channels. Each server has it's own Receiver to handle incoming messages. A separate Receiver
/// is used for users of this library. We then have a Sender that sends to the server's receiver.
/// This is meant to be copied to the appropriate task.
///
/// Usage:
///
/// ```rust
/// use gossip::{Server, Shutdown};
/// use std::time::duration::Duration;
///
/// let mut task = Server::create("127.0.0.1", 5666);
///
/// // Shutdown in the specified time in seconds.
/// task.shutdown(Duration::seconds(1));
///
/// // Wait for new messages. This will block the main task until the
/// // server has been shutdown.
/// loop {
///     match task.recv() {
///         Shutdown(reason) => break,
///         _ => {}
///     }
/// }
/// ```
pub struct Server {
    /// A unique id for the server. This allows servers to talk about each other in
    /// a consistent manner.
    id: Uuid,
    /// Each server has an Addr instance. Regardless of what type of transport it has.
    addr: Option<Addr>,
    /// The state handles the core Gossip protocol. It's basically a giant state machine
    /// that keeps track of which nodes to communicate with, which nodes are alive/dead/failing,
    /// etc...
    state: State,
    /// We need to know a list of servers in the cluster (excluding itself).
    servers: Vec<Node>,
    receiver: Receiver<ServerMsg>,
    sender: Sender<ServerMsg>,
    /// External sender
    tx: Sender<ServerMsg>
}

impl Server {

    pub fn new(sender: Sender<ServerMsg>) -> Server {
        // Create a default channel for the server itself.
        let (tx, rx) = channel();

        let server = Server {
            id: Uuid::new_v4(),
            addr: None,
            state: State::new(),
            servers: Vec::new(),
            receiver: rx,
            sender: tx,
            tx: sender
        };

        server
    }

    /// Create a brand new server with a bunch of defaults. It won't actually connect to
    /// anything nor do anything. That's up to the transports to initiate the connections
    /// and such.
    ///
    /// 1. Chan<ServerMsg>: Server -> User
    /// 2. Chan<Sender<ServerMsg>>: User -> Server
    ///
    /// 1. We need the user to receive real messages.
    /// 2. We need the user to be able to send real messages back to the server.
    pub fn create(ip: &str, port: u16) -> ServerTask {
        ServerTask::create(ip, port)
    }

    /// Bind the server to the specified address. If there's a transport,
    /// it has the possibility of failing, otherwise, it uses an in-memory
    /// function.
    pub fn listen(&mut self, ip: &str, port: u16) -> GossipResult<()> {
        self.addr = Some(Addr::new(ip, port));

        loop {
            match self.receiver.recv() {
                Shutdown(reason) => {
                    println!("Server is shutting down, reason: {}", reason);
                    self.tx.send(Shutdown(UserInitiatedShutdown));
                    break;
                },
                _ => {}
            }
        }

        Ok(())
    }

    /// Shutdown the current server and disconnect from the cluster. This has to first
    /// communicate with the cluster to properly disconnect it, so it's an asynchronous
    /// operation.
    ///
    /// Note that this method is only called from the server's task, not from the user's
    /// tasks. To do the latter, you'll have to use the Sender that the server passes to
    /// you.
    pub fn close(&mut self) {
        //self.ctx.send(Shutdown(UserInitiatedShutdown));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_server() {
        let (tx, rx) = channel();
        let s = Server::new(tx);
        assert!(s.addr.is_none())
    }
}
