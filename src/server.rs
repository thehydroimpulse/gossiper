use std::comm::{Receiver, Sender};
use uuid::Uuid;
use std::io::timer::Timer;
use std::time::duration::Duration;

use addr::Addr;
use broadcast::Broadcast;
use result::{GossipResult, GossipError};

use node::Node;
use health::{Health};
use state::State;

/// Each shutdown command needs to have a reason. Sometimes, we may not know the reason
/// of a particular shutdown (the network may fail, but we don't know if it's them or us, etc...).
/// This also improves error handling and error reporting to where it allows one to report the type
/// of shutdown and when.
#[deriving(Show, PartialEq)]
pub enum ShutdownReason {
    UserInitiatedShutdown,
    NetworkFailure,
    Failure
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
    Message(Broadcast),
    /// A shutdown command to the local node. This is an internal variant and thus isn't used
    /// to kill another member within the cluster. If we have received one of those commands, then
    /// this is emitted internally.
    Shutdown(ShutdownReason),
}

/// Handle the communication and setup of the server. Each server is spawned on
/// a separate task for isolation and independence. Thus, the only communication mechanism
/// is through the use of channels. Instead of requiring the user of the crate to initialize
/// all the local channels and such, this handles it automatically resulting in a super
/// clean API.
pub struct Server {
    /// Part of a channel that the server sends messages to.
    receiver: Receiver<ServerMessage>,
    /// Part of a channel that communicates with the server.
    sender: Sender<ServerMessage>
}

impl Server {
    /// Create a new `Server` that will handle the initialization of
    /// a server in a separate task. This will also handle creating all the required
    /// channels and such.
    ///
    /// ```rust
    /// use gossip::{Server};
    ///
    /// let mut server = Server::create("127.0.0.1", 4555);
    ///
    /// // We need to shutdown the system so that we don't hang in the
    /// // tests.
    /// server.shutdown();
    /// ```
    pub fn create(ip: &str, port: u16) -> Server {
        // Create an intermediate channel to send and receive another Sender
        // that will be used in the `Server`. This channel is only used
        // once and then thrown away.
        let (tx, rx) = channel();

        // Create another channel that will act as the local receiver. The sender
        // will be sent to the server.
        let (sender, receiver) = channel();

        // A slice does not implement `Send` so we have to allocate it first.
        let addr = ip.to_string();

        spawn(proc() {
            // Create a new server sending our `Sender`. This allows the server to send the user
            // messages over the channel.
            let mut server = InternalServer::new(sender.clone());

            // Send back the server's sender portion of the channel. This is how
            // we can further communicate with the server.
            tx.send(server.sender.clone());

            // Start the server. This will run forever until it's killed.
            server.listen(addr.as_slice(), port).unwrap();
        });

        Server {
            receiver: receiver,
            sender: rx.recv()
        }
    }

    /// Call the shutdown method after a set duration. This uses a
    /// synchronous timer.
    pub fn shutdown_in(&mut self, time: Duration) {
        let mut timer = Timer::new().unwrap();
        timer.sleep(time);
        self.shutdown();
    }

    /// Shutdown the current server now. This may not happen right away,
    /// but as soon as the proper protocol has been followed.
    pub fn shutdown(&mut self) {
        self.sender.send(Shutdown(UserInitiatedShutdown));
    }

    /// A slightly nicer interface for sending messages to the Server. This
    /// allows us to keep the `receiver` and `sender` private.
    pub fn recv(&mut self) -> ServerMessage {
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
/// use gossip::{InternalServer, Shutdown};
/// use std::time::duration::Duration;
///
/// let mut server = InternalServer::create("127.0.0.1", 5666);
///
/// // Shutdown in the specified time in seconds.
/// server.shutdown_in(Duration::seconds(1));
///
/// // Wait for new messages. This will block the main task until the
/// // server has been shutdown.
/// loop {
///     match server.recv() {
///         Shutdown(reason) => break,
///         _ => {}
///     }
/// }
/// ```
pub struct InternalServer {
    /// A unique id for the server. This allows servers to talk about each other in
    /// a consistent manner.
    id: Uuid,
    /// The server may have an address to bind to. We make it optional to have a cleaner API
    /// to initially create the server, as the address is only needed at the .listen method.
    addr: Option<Addr>,
    /// Keep track of the server's status so that operations can be applied or rejected depending
    /// on which status it's current set on. This allows us to track whether the server
    /// has already initiated a shutdown or whether it's still running.
    status: Status,
    /// The state handles the core Gossip protocol. It's basically a giant state machine
    /// that keeps track of which nodes to communicate with, which nodes are alive/dead/failing,
    /// etc...
    state: State,
    /// A collection of servers within the cluster. Note that these aren't allocated/running
    /// servers, so we use another record called `Node` to handle that understanding.
    servers: Vec<Node>,
    /// The internal receiver to handle incoming messages. These can be messages coming from a
    /// transport, or from a user.
    receiver: Receiver<ServerMessage>,
    /// The sender-half of the previous channel. This is copied to the user and transports to send
    /// incoming messages. These aren't always broadcasts! Broadcasts are simply one kind of server
    /// message.
    sender: Sender<ServerMessage>,
    /// This is the user sender so that we can communicate with the end user of this crate. Any
    /// broadcasts that we don't handle will be relayed to the user's sender and they can take
    /// care of it.
    tx: Sender<ServerMessage>
}

impl InternalServer {

    /// Create a standard, default server. This only initializes the server
    /// but does **not** start it! So it doesn't spawn a new thread.
    ///
    /// This function requires a Sender so that the server can relay messages
    /// externally (such as the user of the crate).
    pub fn new(sender: Sender<ServerMessage>) -> InternalServer {
        // Create an internal channel for the server. The sender is often
        // copied around to various components.
        let (tx, rx) = channel();

        InternalServer {
            id: Uuid::new_v4(),
            addr: None,
            status: Initializing,
            state: State::new(),
            servers: Vec::new(),
            receiver: rx,
            sender: tx,
            tx: sender
        }
    }

    /// Create a new `Server` that is responsible for fully initializing and starting
    /// the server component. A `Server` is what the user will interact with as the server
    /// is running within another task, so the communication protocol is through channels
    /// exclusively.
    pub fn create(ip: &str, port: u16) -> Server {
        Server::create(ip, port)
    }

    /// Bind the server to the specified address. If there's a transport,
    /// it has the possibility of failing.
    pub fn listen(&mut self, ip: &str, port: u16) -> GossipResult<()> {
        self.addr = Some(Addr::new(ip, port));
        self.status = Running;

        // FIXME: We'll probably need a select! macro invocation to allow the ability
        // to read from multiple sources.
        loop {
            // Receive the next message from the channel. This can be from any component.
            match self.receiver.recv() {
                // Ok, so we are tasked with shutting down. We'll relay the shutdown
                // message back to the user so they can do some cleanup or other operations.
                //
                // FIXME: We'll need to broadcast to the cluster that this server is shutting down.
                Shutdown(reason) => {
                    self.tx.send(Shutdown(reason));
                    break;
                },
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn default_server() {
        let (tx, rx) = channel();
        let s = InternalServer::new(tx);
        assert!(s.addr.is_none())
        assert_eq!(s.status, Initializing);
    }

    #[test]
    fn server_access_with_mutex() {
        let (tx, rx) = channel();

        // Create the task local mutex.
        let mutex = Arc::new(Mutex::new(InternalServer::new(tx)));

        // Copy the mutex (through the Arc) for use in another task.
        let mutex2 = mutex.clone();

        // We need to get the sender from the server.
        let mut val = mutex.lock();
        let sender = (*val).sender.clone();

        // Release the mutex.
        drop(val);

        spawn(proc() {
            let mut val = mutex2.lock();
            val.listen("127.0.0.1", 8777);
        });

        // Shutdown the server so we can finish the test.
        sender.send(Shutdown(UserInitiatedShutdown));

        match rx.recv() {
            Shutdown(reason) if reason == UserInitiatedShutdown => {
                assert!(true);
            },
            _ => fail!("Unexpected output")
        }
    }

    #[test]
    fn shutdown_server() {
        let (tx, rx) = channel();
        let mut s = InternalServer::new(tx);

        s.sender.send(Shutdown(UserInitiatedShutdown));
    }
}
