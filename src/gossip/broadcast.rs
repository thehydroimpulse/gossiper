use uuid::Uuid;
use result::GossipResult;
use util::as_byte_slice;
use connection::Connection;
use response::Response;
use encode::encode;
use protocol::Version;

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

impl<'a, T> Broadcast<'a, T> {

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
    pub fn with_response(message: T, response: |response: Box<Response>|: 'a) -> Broadcast<'a, T> {
        Broadcast {
            id: Uuid::new_v4(),
            request: message,
            response: Some(response)
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
    ///
    /// Encoding Format:
    ///
    /// Size (RequestMessage | ResponseMessage)
    pub fn send(&self, connection: Box<Connection>) -> GossipResult<()> {
        // We need a raw byte slice to send over the network.
        let bytes = as_byte_slice(&self.request);

        connection.send(encode(Version(1), &self.request))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_no_response() {
        let broadcast = Broadcast::new(123);
        assert!(broadcast.response.is_none());
    }

    #[test]
    fn add_response() {
        let broadcast = Broadcast::with_response(123, |response| {});
        assert!(broadcast.response.is_some());
    }

    #[test]
    fn send_broadcast() {
        use tcp::transport::TcpTransport;
        use tcp::connection::TcpConnection;
        use std::io::net::ip::Ipv4Addr;
        use transport::Transport;
        use result::GossipResult;

        let addr       = "127.0.0.1";
        let port       = 5988;

        // Create a new transport to start accepting new
        // connections.
        let transport  = TcpTransport::new(addr, port);

        // Establish a new connection to the transport. This will
        // add a connection.
        let connection = box TcpConnection::new(addr, port);

        let broadcast  = Broadcast::new(123);

        //broadcast.send(connection);

        //let msg: GossipResult<Message> = transport.receive();
    }
}
