use std::fmt::{Show,Formatter};
use std::fmt;
use std::io;

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

/// Convert an IoError to a GossipError
pub fn to_gossip(io: io::IoError) -> GossipError {
    GossipError {
        kind: IoError(io.clone()),
        desc: StaticDescription(io.desc)
    }
}

/// A gossip error represents an error that happens typically during any I/O.
#[deriving(Show)]
pub struct GossipError {
    kind: GossipErrorKind,
    desc: GossipErrorDescription,
}

#[deriving(Show)]
pub enum GossipErrorDescription {
    StaticDescription(&'static str),
    BoxedDescription(String)
}

#[deriving(Show)]
pub enum GossipErrorKind {
    IoError(io::IoError)
}

impl GossipError {
    pub fn new<T: Show>(desc: T, kind: GossipErrorKind) -> GossipError {
        GossipError {
            desc: BoxedDescription(desc.to_str()),
            kind: kind
        }
    }
}
