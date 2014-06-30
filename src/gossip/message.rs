use uuid::Uuid;
use server::Server;
use std::fmt::{Show, Formatter, FormatError};

pub trait Message {}

impl Show for Box<Message + Send> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(write!(f, "Message"));
        Ok(())
    }
}

impl<T: Share> Message for T {}

#[deriving(Clone, Encodable, Decodable, PartialEq, Show)]
pub struct Join {
    id: Uuid,
    ip: String,
    port: u16
}

impl Join {
    pub fn new<'a, T>(server: &Server<'a, T>) -> Join {
        Join {
            id: server.id,
            ip: server.ip.to_string(),
            port: server.port
        }
    }
}
