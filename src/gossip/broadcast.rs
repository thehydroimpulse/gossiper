use uuid::Uuid;
use serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};
use std::io::IoError;

use result::GossipResult;
use connection::Connection;
use response::Response;
use version::Version;
use message::Message;

/// Broadcast represents a single bi-directional communication with two
/// nodes within the cluster. The communication does **not** need to be
/// bi-directional. Responses are completely optional.
///
/// Each broadcast is tagged with a unique ID so that we may track
/// which node has received a given broadcast.
#[deriving(Clone, PartialEq, Encodable, Decodable)]
pub struct Broadcast<T> {
    /// A unique id (uuidv4) representing the broadcast. This will allow us to keep
    /// track of it when dealing with many broadcasts and we receive them in
    /// different orders.
    id: Uuid,

    /// Request is an arbitrary type. This allows users
    /// to specify their own custom broadcasts to be sent and received.
    body: T
}

impl<'a, T: Message + Clone + Encodable<Encoder<'a>, IoError> + Decodable<Decoder<'a>, IoError>> Broadcast<T> {

    pub fn new(message: T) -> Broadcast<T> {
        Broadcast {
            id: Uuid::new_v4(),
            body: message
        }
    }

    /// Send the broadcast to a given server.
    ///
    /// `send` only works with a single server. It requires a
    /// connection (tcp or otherwise) to be established between the two clients.
    ///
    /// ```rust
    /// let broadcast = Broadcast::new(123);
    /// let connection = TcpConnection::new(Ipv4Addr(127, 0, 0, 1), 5499);
    ///
    /// // Send the broadcast.
    /// match broadcast.send(connection) {
    ///     Ok(msg) => {},
    ///     Err(err) => {}
    /// }
    /// ```
    pub fn send<A: Connection>(&self, connection: &mut A) -> GossipResult<()> {
        connection.send(self.clone());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use result::GossipResult;
    use tcp::transport::TcpTransport;
    use tcp::connection::TcpConnection;
    use std::io::net::ip::Ipv4Addr;
    use transport::Transport;
    use connection::Connection;
}
