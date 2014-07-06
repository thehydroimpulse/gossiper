use message::Message;

pub mod encoding;

#[deriving(Show, PartialEq)]
pub enum TagType {
    TyString(String),
    TyInt(int)
}

/// A tagged value represents a value we have received, but we don't know how
/// to handle it, nor can we interpret it into an actual type. When we receive
/// such a value, we tag it and send it off. At a later point, we'll probably
/// know how to decode the tagged value appropriately.
///
/// One thing tagged values **do** have is a unique ID. Meaning, if we tag two
/// values of the same type, they'll have the same ID and these ids can be user
/// generated and the id can be of a variation of types (e.g., a string)
pub struct TaggedValue {
    id: TagType,
    resolved: bool,
    bytes: Vec<u8>
}

impl TaggedValue {
    /// Create a brand new tagged value given it's `id` and contents.
    pub fn new(id: TagType, bytes: Vec<u8>) -> TaggedValue {
        TaggedValue {
            id: id,
            resolved: false,
            bytes: bytes
        }
    }

    /// This essentially resolves the value. We have enough information, or
    /// we're at a point in the application where we can safely resolve the 
    /// message to it's appropriate type.
    ///
    /// FIXME(TheHydroImpulse): We need to add further encodability and decodability
    ///                         constraints to type `M`.
    pub fn get<A: Message>() {

    }
}
