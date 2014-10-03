//! A broadcast represents a single message being sent out.

use uuid::Uuid;
use std::collections::hashmap::HashSet;
use std::io::MemReader;

use result::{GossipResult, GossipError, UnknownError, io_err};
use node::Stream;

#[deriving(PartialEq, Show)]
#[repr(C)]
pub enum Format {
    MsgPack = 0,
    Json = 1
}

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
///         version: u4,
///         format: u4, // Format and version can be read in one operation.
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
    format: Format,
    /// A tag represents the type of message it is without needing a physical type to decode it to.
    /// Since we may not always have that information.
    tag: String,
    /// The raw bytes of the full broadcast.
    reader: Vec<u8>,
    /// A set of servers that have seen/committed the broadcast.
    committed: HashSet<String>
}

impl Broadcast {
    /// Given a tag and message, create a new instance of Broadcast with
    /// a brand-new unique ID so that we can uniquely identify it.
    pub fn new(bytes: Vec<u8>) -> GossipResult<Broadcast> {
        let mut reader = MemReader::new(bytes.clone());
        let version = try!(reader.read_byte().map_err(io_err));
        let format = match try!(reader.read_byte().map_err(io_err)) {
            0 => MsgPack,
            1 => Json,
            _ => return Err(GossipError::new("Unknown format.", UnknownError))
        };
        let tag = "foo".to_string();
        Ok(Broadcast {
            id: Uuid::new_v4(),
            version: Version(version),
            format: format,
            tag: tag,
            reader: bytes,
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

}
