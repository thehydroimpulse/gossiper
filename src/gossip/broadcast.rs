use uuid::Uuid;
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
    response: Option<|response: Box<Message>|: 'a>
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
    ///
    /// To allow chaining, we'll return a reference to the broadcast
    /// object.
    pub fn with_response(message: T,
                         response: |response: Box<Message>|: 'a) -> Broadcast<'a, T> {
        Broadcast {
            id: Uuid::new_v4(),
            request: message,
            response: Some(response)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn have_no_response() {
        let broadcast = Broadcast::new(123);
        assert!(broadcast.response.is_none());
    }

    #[test]
    fn add_response() {
        let mut broadcast = Broadcast::with_response(123, |response| {

        });

        assert!(broadcast.response.is_some());
    }
}