//! A broadcast represents a single message being sent out.

use uuid::Uuid;
use std::collections::hashmap::HashSet;
use std::io::MemReader;

use result::{GossipResult, GossipError, UnknownError, io_err};
use stream::Stream;

#[deriving(PartialEq, Show)]
pub struct Version(u8);

/// Broadcast represents a single bi-directional communication with two
/// nodes within the cluster. The communication does **not** need to be
/// bi-directional. Responses are completely optional.
///
/// Each broadcast is tagged with a unique ID so that we may track
/// which node has received a given broadcast.
///
/// Each broadcast is sent in a custom binary format. This allows us to store
/// the tag that associates the type of the broadcast in Rust code.
///
/// Format:
///
/// ```notrust
/// bitdata RawBroadcast {
///     RawBroadcast {
///         version: u8,
///         tag_size: u32,
///         tag: &[u8],
///         data_size: u32,
///         data: &[u8]
///     }
/// }
/// ```
///
/// This allows us to effectively parse the metadata we need then
/// forward the decoding to the appropriate format's parser. We also have
/// the ability to interoporate between different formats. As long as each
/// Node has the ability to understand that format.
pub struct Broadcast {
    /// A unique id for the broadcast. This allows the servers
    /// to talk about a unique broadcast in unison and also coordinate
    /// the response (if applicable).
    id: Uuid,
    version: Version,
    /// A tag represents the type of message it is without needing a physical type to decode it to.
    /// Since we may not always have that information.
    tag: String,
    /// The raw bytes of the full broadcast.
    reader: MemReader,
    /// A set of servers that have seen/committed the broadcast.
    committed: HashSet<String>
}

impl Broadcast {
    /// Given a tag and message, create a new instance of Broadcast with
    /// a brand-new unique ID so that we can uniquely identify it.
    pub fn new(bytes: Vec<u8>) -> GossipResult<Broadcast> {
        let mut reader = MemReader::new(bytes.clone());
        let version = try!(reader.read_byte().map_err(io_err));
        let tag = "foo".to_string();
        Ok(Broadcast {
            id: Uuid::new_v4(),
            version: Version(version),
            tag: tag,
            reader: reader,
            committed: HashSet::new()
        })
    }

    pub fn parse(&mut self) {

    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use result::GossipResult;

    #[test]
    fn parse_broadcast() {
        let mut bytes = vec![1u8];
        bytes = bytes.append(b"foo");
        bytes = bytes.append(&[1u8,2,3]);
        let broadcast = Broadcast::new(bytes).unwrap();
        assert_eq!(broadcast.tag, "foo".to_string());
        let Version(ver) = broadcast.version;
        assert_eq!(ver, 1u8);
    }
}
