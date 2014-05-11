use std::fmt::{Show,Formatter};
use std::fmt;

pub type GossipResult<T> = Result<T, GossipError>;

pub enum Error {
    NotImplemented
}

macro_rules! unimplemented(
    () => (
        return Err(GossipError::new("not implemented".to_owned(), None))
    )
)

/// A gossip error represents **any** errors that happen within this system.
///
/// FIXME: Instead of a useless `code` property, it might be better to have
///        that be an enum instead. With things like `FailedToJoin`.
pub struct GossipError {
    message: StrBuf,
    error: Option<Error>
}

impl GossipError {
    pub fn new(message: StrBuf, error: Option<Error>) -> GossipError {
        GossipError {
            message: message,
            error: error
        }
    }
}

impl Show for GossipError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.message)
    }
}