use std::io::net::ip::{SocketAddr, IpAddr};
use std::io::{TcpListener, TcpStream, Listener, Acceptor, TimedOut, IoError};
use std::io::net::tcp::{TcpAcceptor, TcpStream};
use std::collections::hashmap::HashMap;
use std::comm::{channel, Sender, Receiver};
use std::io::timer::Timer;
use std::ops::Drop;
use std::task::TaskBuilder;
use std::clone::Clone;

use serialize::{Decodable, Encodable};
use serialize::json::{Encoder, Decoder, DecoderError};
use uuid::Uuid;

use transport::Transport;
use result::{GossipResult, io_err};
use connection::Connection;
use message::{Message, Join};
use tcp::connection::TcpConnection;
use server::Server;

/// Messages that the AcceptingManager is communicating with.
enum AcceptingMsg {
    Exit
}

/// Wrap the common idiom of accepting a string and port into a
/// single source that's easier to pass around. It also mitigates the
/// naming issue of port (u16) and port (Receiver).
#[deriving(Show, PartialEq, Clone)]
pub struct Addr {
    pub ip: String,
    pub port: u16
}

impl Addr {
    /// Create a new instance of the Addr record.
    pub fn new(ip: String, port: u16) -> Addr {
        Addr {
            ip: ip,
            port: port
        }
    }
}

/// Alias the type to be easier to use.
type AcceptingTask = Sender<AcceptingMsg>;

/// The AcceptingManager is responsible for managing incoming
/// TCP connections/streams. The manager first creates a new TcpListener
/// and TcpAcceptor in order to start listening in. All communication
/// then happens through the use of channels. A new StreamManager will
/// be created in order to handle the appropriate streams.
struct AcceptingManager {
    port: Receiver<AcceptingMsg>,
    acceptor: Option<TcpAcceptor>,
    addr: Addr
}

impl AcceptingManager {
    /// Given a Receiver (port) to read new messages from and an address
    /// create a new context. This does **not** actually create a new
    /// TcpAcceptor and doesn't bind to anything. That happens in the
    /// .start() method.
    pub fn new(port: Receiver<AcceptingMsg>, addr: Addr) -> AcceptingManager {
        AcceptingManager {
            port: port,
            acceptor: None,
            addr: addr
        }
    }

    /// Start listening on the Tcp address and start processing incoming
    /// streams. We also need to continue listening on the channel to see if
    /// we should be shutting down or not.
    ///
    /// Note: It's impossible to shutdown while the task is blocking on
    ///       acceptor.accept() and no more connections are coming through.
    ///       You'll have to physically shutdown the whole process, which is
    ///       kinda ugly, but works.
    pub fn start(&mut self) {

        let acceptor = TcpListener::bind(self.addr.ip.as_slice(), self.addr.port).listen();

        loop {
            match self.port.recv() {
                Exit => break
            }
        }
    }
}

/// Create a new AcceptingTask responsible for accepting brand new
/// tcp connections and passing them on.
///
/// We initialize a new setup channel that is used to return the correct
/// Sender, which is created inside the task.
fn create_accepting_task(addr: Addr) -> AcceptingTask {
    let (setup_chan, setup_port) = channel();
    let builder = TaskBuilder::new().named("AcceptingManager");

    builder.spawn(proc() {
        let (chan, port) = channel();
        setup_chan.send(chan);
        AcceptingManager::new(port, addr).start();
    });

    setup_port.recv()
}

/// A tcp transport has two fundamental elements within: An acceptor (server)
/// and a set of connections. The only job of the acceptor is to, well,
/// accept new connections and store them.
///
/// Connections are how all the nodes will communicate. Each node within
/// the system has a connection to every other node in the cluster. That
/// means, if node A wants to communicate with server B, it'll look up
/// server's B connection and send a message through that medium.
pub struct TcpTransport {
    addr: Addr,
    sender: AcceptingTask
}

impl TcpTransport {

    /// Createa a new local transport. The ip and port are used for the
    /// Acceptor. Thus, having a local address of "0.0.0.0" is the common
    /// practice for the tcp server to be accessible from outside the
    /// current node.
    pub fn listen(ip: &str, port: u16) -> GossipResult<TcpTransport> {

        let addr = Addr::new(ip.to_string(), port);
        let sender = create_accepting_task(addr.clone());

        let mut transport = TcpTransport {
            addr: addr,
            sender: sender
        };

        Ok(transport)
    }
}

#[unsafe_destructor]
impl Drop for TcpTransport {
    fn drop(&mut self) {
        self.close();
    }
}

impl Transport for TcpTransport {

    /// By default, a node does **not** join a cluster automatically. Thus,
    /// one has to manually initiate the join operation.
    ///
    /// To join a cluster, one just needs to establish a connection
    /// with one that already has membership (peer node). The join operation first
    /// establishes the new connection **and** sends a "join" broadcast.
    ///
    /// The peer node is responsible for propagating the new membership details
    /// through a new broadcast.
    fn join<'a, T>(&self, ip: &str, port: u16, server: &Server<'a, T>) -> GossipResult<()> {

        // Establish a new connection with the peer node.
        let mut conn = try!(TcpConnection::connect(ip, port));
        try!(conn.send(Join::new(server)));

        Ok(())
    }

    /// Receive a message from any of the connections.
    fn receive<T: Decodable<Decoder, DecoderError>>(&self) -> GossipResult<T> {
        unimplemented!()
    }

    /// Terminate the accept, along with disconnecting all connections. However,
    /// before doing so, the node will send one last broadcast letting the
    /// cluster know it's going offline.
    fn close(&mut self) -> GossipResult<()> {
        self.sender.send(Exit);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::net::ip::{Ipv4Addr};
    use tcp::connection::TcpConnection;
    use connection::Connection;
    use transport::Transport;

    #[test]
    fn new_transport() {
        let addr = "127.0.0.1";
        let port = 5499;

        let mut transport = TcpTransport::listen(addr, port).unwrap();
        let mut connection = TcpConnection::connect(addr, port).unwrap();
    }
}
