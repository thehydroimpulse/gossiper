use uuid::Uuid;
use message::Message;

// A single broadcast to other nodes within the cluster. A broadcast is a
// fundamental primitive for communication. Each broadcast has
// it's own unique identifier to associate it from other broadcasts.
// Some broadcasts require a response to be received, and because we're
// using TCP, we can't guarantee any ordering. This requires each broadcast
// to be signed by the id.
pub struct Broadcast {
    // A unique id (uuidv4) representing the broadcast. This will allow us to keep
    // track of it when dealing with many broadcasts and we receive them in
    // different orders.
    id: Uuid,
    message: Message,
    sent: bool
}

impl Broadcast {
    pub fn new(message: Message) -> Broadcast {
        Broadcast {
            id: Uuid::new_v4(),
            message: message,
            sent: false
        }
    }
}

#[cfg(test)]
mod test {}