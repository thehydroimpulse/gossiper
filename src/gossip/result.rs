use std::io;
use std::str::SendStr;
use serialize::json;

pub type GossipResult<T> = Result<T, GossipError>;

/// Convert an IoError to a GossipError
pub fn io_err(io: io::IoError) -> GossipError {
    GossipError {
        kind: IoError(io.clone()),
        desc: io.desc.into_maybe_owned()
    }
}

pub fn decoder_err(err: json::DecoderError) -> GossipError {
    GossipError {
        kind: DecoderError(err.clone()),
        desc: "DecoderError".into_maybe_owned()
    }
}

/// A gossip error represents an error that happens typically during any I/O.
#[deriving(Show)]
pub struct GossipError {
    kind: GossipErrorKind,
    desc: SendStr,
}

#[deriving(Show)]
pub enum GossipErrorKind {
    IoError(io::IoError),
    DecoderError(json::DecoderError),
    TaggedDecodingError
}

impl GossipError {
    pub fn new<T: IntoMaybeOwned<'static>>(desc: T, kind: GossipErrorKind) -> GossipError {
        GossipError {
            kind: kind,
            desc: desc.into_maybe_owned(),
        }
    }
}
