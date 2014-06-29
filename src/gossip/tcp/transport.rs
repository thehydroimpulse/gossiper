use std::io::net::ip::{SocketAddr, IpAddr};
use std::io::{TcpListener, TcpStream, Listener, Acceptor, TimedOut};
use std::io::net::tcp::{TcpAcceptor, TcpStream};
use std::collections::hashmap::HashMap;
use std::comm::{channel, Sender, Receiver};
use std::io::timer::Timer;
use std::ops::Drop;

use serialize::Decodable;
use serialize::json::{Decoder, DecoderError};
use uuid::Uuid;

use transport::Transport;
use result::{GossipResult, io_err};
use connection::Connection;
use message::{Message, Join};
use tcp::connection::TcpConnection;
use server::Server;

#[deriving(PartialEq, Show)]
pub enum ChanMessage {
    StopListening,
    StartListening
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
    ip: String,
    port: u16,
    sender: Sender<ChanMessage>,
    /// A single server might have 10s or even 100s of connections, so
    /// we need an effecient way to fetch them based on the node
    /// we want to communicate with. Each server will have it's own
    /// unique Uuidv4 which we'll use as the key for the hashmap.
    connections: HashMap<Uuid, TcpConnection>
}

impl TcpTransport {

    /// Createa a new local transport. The ip and port are used for the
    /// Acceptor. Thus, having a local address of "0.0.0.0" is the common
    /// practice for the tcp server to be accessible from outside the
    /// current node.
    ///
    /// FIXME: Perhaps we should handle the errors a little nicer?
    pub fn listen(ip: &str, port: u16) -> GossipResult<TcpTransport> {

        let (sender, receiver) = channel();

        let mut transport = TcpTransport {
            ip: ip.to_string(),
            port: port,
            sender: sender,
            connections: HashMap::new()
        };

        transport.handle(receiver);

        Ok(transport)
    }

    pub fn handle(&mut self, rx: Receiver<ChanMessage>) {
        let ip = self.ip.clone();
        let port = self.port.clone();

        let mut timer = Timer::new().unwrap();
        let timeout = timer.oneshot(1000);

        spawn(proc() {

            let listener = match TcpListener::bind(ip.as_slice(), port).map_err(io_err) {
                Ok(listener) => listener,
                Err(err) => fail!("Something bad happened. {}", err)
            };

            let mut acceptor = match listener.listen().map_err(io_err) {
                Ok(acceptor) => acceptor,
                Err(err) => fail!("Oops: {}", err)
            };

            let mut accept = true;
            let mut timer = Timer::new().unwrap();
            let timeout = timer.oneshot(500);

            loop {
                select! {
                    val = rx.recv() => {
                        match val {
                            StopListening => {
                                break;
                            },
                            _ => {}
                        }
                    },
                    () = timeout.recv() => {
                        println!("timed out")
                        break;
                    }
                }

                let stream = acceptor.accept();
            }
        });

    }
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        self.close();
        drop(self);
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
        self.sender.send(StopListening);
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
