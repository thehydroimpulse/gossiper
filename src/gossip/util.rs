use std::fmt::{Show,Formatter};
use std::fmt;

pub fn as_byte_slice<'a, T>(x: &'a T) -> &'a [u8] {
    unsafe {
        ::std::slice::raw::buf_as_slice(
            x as *_ as *u8,
            ::std::mem::size_of::<T>(),
            |v| ::std::mem::transmute(v)
        )
    }
}

pub type GossipResult<T> = Result<T, GossipError>;

pub enum Error {
    NotImplemented
}

macro_rules! unimplemented(
    () => (
        return Err(GossipError::new("not implemented".to_owned(), None))
    )
)

/// A gossip error represents an error that happens typically during any I/O.
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
        write!(f, "{}", self.message)
    }
}
