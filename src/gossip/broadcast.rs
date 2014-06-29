use uuid::Uuid;
use serialize::{Encodable, Decodable};
use serialize::json::{Encoder, Decoder, DecoderError};
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
pub struct Broadcast<'a, T> {
    /// A unique id (uuidv4) representing the broadcast. This will allow us to keep
    /// track of it when dealing with many broadcasts and we receive them in
    /// different orders.
    id: Uuid,

    /// Request is an arbitrary type. This allows users
    /// to specify their own custom broadcasts to be sent and received.
    request: T,

    /// Each broadcast may have an **optional** response of a different
    /// type than the request. The closure will be called once we receive
    /// the full response.
    ///
    /// The closure is not guaranteed to be ran on a specific thread.
    response: Option<|response: Box<Response>|: 'a>
}

impl<'a, T: Clone + Encodable<Encoder<'a>, IoError> + Decodable<Decoder, DecoderError>> Broadcast<'a, T> {

    pub fn new(message: T) -> Broadcast<'a, T> {
        Broadcast {
            id: Uuid::new_v4(),
            request: message,
            response: None
        }
    }

    /// Add a response to the broadcast, which isn't required. Once
    /// the response has been received, the closure will be called.
    ///
    /// ```rust
    /// Broadcast::new(Message).with_response(|response| {
    ///     // Do something with the response
    /// });
    /// ```
    pub fn with_response(mut self, response: |response: Box<Response>|: 'a) -> Broadcast<'a, T> {
        self.response = Some(response);
        self
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
    pub fn send<A: Connection>(&self, connection: A) -> GossipResult<()> {
        let v = Version(1);
        connection.send(Message::new_request(v, self.request.clone()));
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
    use message::Message;
    use connection::Connection;

    #[test]
    fn has_no_response() {
        let broadcast = Broadcast::new(123 as int);
        assert!(broadcast.response.is_none());
    }

    #[test]
    fn add_response() {
        let broadcast = Broadcast::new(123 as int).with_response(|response| {});
        assert!(broadcast.response.is_some());
    }

    #[test]
    fn send_broadcast() {

        let addr       = "127.0.0.1";
        let port       = 5988;

        let transport  = TcpTransport::listen(addr, port).unwrap();
        let connection = TcpConnection::connect(addr, port).unwrap();
        let broadcast  = Broadcast::new(123 as int);

        broadcast.send(connection);

        //let msg: GossipResult<int> = transport.receive();
    }
}
