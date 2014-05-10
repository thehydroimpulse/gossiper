use uuid::Uuid;

// A single broadcast to other nodes within the cluster. A broadcast is a
// fundamental primitive for communication. Each broadcast has
// it's own unique identifier to associate it from other broadcasts.
// Some broadcasts require a response to be received, and because we're
// using TCP, we can't guarantee any ordering. This requires each broadcast
// to be signed by the id.
pub struct Broadcast<'a, T> {
    // A unique id (uuidv4) representing the broadcast. This will allow us to keep
    // track of it when dealing with many broadcasts and we receive them in
    // different orders.
    id: Uuid,
    request: T,
    response: Option<|response: T|: 'a>
}

impl<'a, T> Broadcast<'a, T> {

    /// Create a new Broadcast with a unique uuidv4 id and an empty
    /// response.
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
    pub fn with_response<'b>(&'b mut self, response: |response: T|: 'a) -> &'b Broadcast<'a, T> {
        self.response = Some(response);
        &*self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use message::Empty;

    #[test]
    fn have_no_response() {
        let broadcast = Broadcast::new(Empty);

        assert!(broadcast.response.is_none());
    }
}