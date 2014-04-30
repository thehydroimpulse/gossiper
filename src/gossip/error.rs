use std::fmt::{Show,Formatter};
use std::fmt;

pub type GossipResult<T> = Result<T, GossipError>;

/// A gossip error represents **any** errors that happen within this system.
///
/// FIXME: Instead of a useless `code` property, it might be better to have
///        that be an enum instead. With things like `FailedToJoin`.
pub struct GossipError {
    message: ~str,
    code: uint
}

impl GossipError {
    pub fn new(message: ~str, code: uint) -> GossipError {
        GossipError {
            message: message,
            code: code
        }
    }
}

impl Show for GossipError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.message)
    }
}