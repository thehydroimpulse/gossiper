use serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder, from_msgpack};
use std::io::IoError;
use std::io::Reader;

use tag::{TaggedValue, to_tag, to_bytes, TagType};
use broadcast::Broadcast;
use message::Message;
use result::{GossipResult, io_err};

pub fn encode<'a, T: Message
        + Encodable<Encoder<'a>, IoError>
        + Decodable<Decoder<'a>, IoError>>(ty: TagType, val: Broadcast<T>)
        -> GossipResult<Vec<u8>> {
    let b = try!(Encoder::to_msgpack(&val).map_err(io_err));
    to_bytes(ty, b.as_slice())
}

pub fn decode<'a>(rd: &'a mut Reader) -> GossipResult<TaggedValue> {
    to_tag(rd)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serialize::{Encodable, Decodable};
    use std::io::MemReader;
    use broadcast::Broadcast;
    use tag::TyString;

    #[test]
    fn encode_and_decode() {
        #[deriving(Clone, Encodable, Decodable)]
        struct Foo {
            i: int
        }

        let b = Broadcast::new(Foo { i: 5 });
        let bytes = encode(TyString("Foo".to_string()), b).unwrap();
        let mut rd = MemReader::new(bytes);
        match decode(&mut rd) {
            Ok(tag) => {
                assert_eq!(tag.id, TyString("Foo".to_string()));
            },
            Err(err) => fail!("Unexpected Error: {}", err)
        }
    }
}
